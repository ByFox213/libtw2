use crate::error::Error;
use crate::error::InvalidIntString;
use arrayvec::ArrayVec;
use libtw2_buffer::CapacityError;
use libtw2_common::slice;
use libtw2_packer::ExcessData;
use libtw2_packer::Packer;
use libtw2_packer::Unpacker;
use libtw2_packer::Warning;
use libtw2_warn::Warn;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use std::fmt;
use std::io::Write;
use std::mem;
use std::str;
use uuid::Uuid;
use zerocopy::byteorder::big_endian;

pub const CLIENTS_DATA_NONE: ClientsData<'static> = ClientsData { inner: b"" };

#[derive(Clone, Copy, Debug)]
pub struct ClientsData<'a> {
    inner: &'a [u8],
}

impl ClientsData<'_> {
    #[must_use]
    pub fn from_bytes(bytes: &[u8]) -> ClientsData<'_> {
        ClientsData { inner: bytes }
    }
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        self.inner
    }
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub struct AddrPacked {
    ip_address: [u8; 16],
    port: big_endian::U16,
}

pub trait AddrPackedSliceExt {
    fn from_bytes<'a, W: Warn<ExcessData>>(warn: &mut W, bytes: &'a [u8]) -> &'a Self;
    fn as_bytes(&self) -> &[u8];
}

impl AddrPackedSliceExt for [AddrPacked] {
    fn from_bytes<'a, W: Warn<ExcessData>>(warn: &mut W, bytes: &'a [u8]) -> &'a [AddrPacked] {
        let remainder = bytes.len() % mem::size_of::<AddrPacked>();
        if remainder != 0 {
            warn.warn(ExcessData);
        }
        let actual_len = bytes.len() - remainder;
        unsafe { slice::transmute(&bytes[..actual_len]) }
    }
    fn as_bytes(&self) -> &[u8] {
        unsafe { slice::transmute(self) }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct TuneParam(pub i32);

impl TuneParam {
    #[allow(clippy::cast_possible_truncation)]
    #[must_use]
    pub fn from_float(float: f32) -> TuneParam {
        TuneParam((float * 100.0) as i32)
    }
    #[allow(clippy::cast_precision_loss)]
    #[must_use]
    pub fn to_float(self) -> f32 {
        (self.0 as f32) / 100.0
    }
}
#[allow(clippy::missing_errors_doc)]
pub fn int_from_string(bytes: &[u8]) -> Result<i32, InvalidIntString> {
    str::from_utf8(bytes).map_or(Err(InvalidIntString), |s| {
        s.parse().map_err(|_| InvalidIntString)
    })
}

#[must_use]
#[allow(clippy::unwrap_used)]
pub fn string_from_int(int: i32) -> ArrayVec<[u8; 16]> {
    let mut result = ArrayVec::new();
    write!(&mut result, "{int}").unwrap();
    result
}

#[derive(Clone, Copy, Deserialize, Eq, Ord, Hash, PartialEq, PartialOrd, Serialize)]
#[serde(untagged)]
pub enum MessageId {
    Ordinal(i32),
    Uuid(Uuid),
}

impl fmt::Debug for MessageId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MessageId::Ordinal(i) => i.fmt(f),
            MessageId::Uuid(u) => u.fmt(f),
        }
    }
}

impl fmt::Display for MessageId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl From<i32> for MessageId {
    fn from(i: i32) -> MessageId {
        MessageId::Ordinal(i)
    }
}

impl From<Uuid> for MessageId {
    fn from(u: Uuid) -> MessageId {
        MessageId::Uuid(u)
    }
}

pub trait Protocol<'a> {
    type System;
    type Game;
    #[allow(clippy::missing_errors_doc)]
    fn decode_system<W>(
        warn: &mut W,
        id: MessageId,
        p: &mut Unpacker<'a>,
    ) -> Result<Self::System, Error>
    where
        W: Warn<Warning>;
    #[allow(clippy::missing_errors_doc)]
    fn decode_game<W>(
        warn: &mut W,
        id: MessageId,
        p: &mut Unpacker<'a>,
    ) -> Result<Self::Game, Error>
    where
        W: Warn<Warning>;
}

#[derive(Clone, Copy, Debug)]
pub enum SystemOrGame<S, G> {
    System(S),
    Game(G),
}

impl<S, G> SystemOrGame<S, G> {
    fn is_game(&self) -> bool {
        match *self {
            SystemOrGame::System(_) => false,
            SystemOrGame::Game(_) => true,
        }
    }
    fn is_system(&self) -> bool {
        !self.is_game()
    }
}

impl SystemOrGame<MessageId, MessageId> {
    #[allow(clippy::missing_errors_doc)]
    pub fn decode_id<W>(
        warn: &mut W,
        p: &mut Unpacker<'_>,
    ) -> Result<SystemOrGame<MessageId, MessageId>, Error>
    where
        W: Warn<Warning>,
    {
        let id = p.read_int(warn)?;
        let sys = id & 1 != 0;
        let msg = id >> 1;
        let msg = if msg != 0 {
            MessageId::Ordinal(msg)
        } else {
            MessageId::Uuid(p.read_uuid()?)
        };
        Ok(if sys {
            SystemOrGame::System(msg)
        } else {
            SystemOrGame::Game(msg)
        })
    }
    fn internal_id(self) -> MessageId {
        match self {
            SystemOrGame::System(msg) | SystemOrGame::Game(msg) => msg,
        }
    }
    #[allow(
        clippy::missing_errors_doc,
        clippy::missing_panics_doc,
        clippy::cast_sign_loss,
        clippy::cast_possible_wrap
    )]
    pub fn encode_id<'d>(self, mut p: Packer<'d, '_>) -> Result<&'d [u8], CapacityError> {
        let iid = match self.internal_id() {
            MessageId::Ordinal(i) => {
                assert!(i != 0);
                i as u32
            }
            MessageId::Uuid(_) => 0,
        };
        assert!((iid & (1 << 31)) == 0);
        let flag = u32::from(self.is_system());
        p.write_int(((iid << 1) | flag) as i32)?;
        if let MessageId::Uuid(u) = self.internal_id() {
            p.write_uuid(u)?;
        }
        Ok(p.written())
    }
}

#[allow(clippy::missing_errors_doc)]
pub fn decode<'a, W, P>(
    warn: &mut W,
    _proto: P,
    p: &mut Unpacker<'a>,
) -> Result<SystemOrGame<P::System, P::Game>, Error>
where
    W: Warn<Warning>,
    P: Protocol<'a>,
{
    let msg_id = SystemOrGame::decode_id(warn, p)?;
    Ok(match msg_id {
        SystemOrGame::System(msg_id) => SystemOrGame::System(P::decode_system(warn, msg_id, p)?),
        SystemOrGame::Game(msg_id) => SystemOrGame::Game(P::decode_game(warn, msg_id, p)?),
    })
}
