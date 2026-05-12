use crate::format;
use crate::format::item::INPUT_LEN;
use crate::format::Header;
use crate::raw;
use crate::raw::Callback;
use std::fs::File;
use std::io;
use std::io::Read;
use std::ops;
use std::path::Path;

pub use crate::raw::Buffer;
pub use crate::raw::Item;
pub use crate::raw::Pos;

#[derive(Debug)]
pub enum Error {
    Teehistorian(format::Error),
    Io(io::Error),
}

impl From<format::Error> for Error {
    fn from(err: format::Error) -> Error {
        Error::Teehistorian(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<raw::Error<io::Error>> for Error {
    fn from(err: raw::Error<io::Error>) -> Error {
        match err {
            raw::Error::Teehistorian(e) => Error::Teehistorian(e),
            raw::Error::Cb(e) => Error::Io(e),
        }
    }
}

struct CallbackData {
    file: File,
}

pub struct Reader {
    callback_data: CallbackData,
    raw: raw::Reader,
}

impl Reader {
    #[allow(clippy::missing_errors_doc)]
    fn new_impl(file: File, buffer: &mut Buffer) -> Result<(Header<'_>, Reader), Error> {
        let mut callback_data = CallbackData { file };
        let (header, raw) = raw::Reader::new(&mut callback_data, buffer)?;
        Ok((
            header,
            Reader {
                callback_data,
                raw,
            },
        ))
    }
    #[allow(clippy::missing_errors_doc)]
    pub fn new(file: File, buffer: &mut Buffer) -> Result<(Header<'_>, Reader), Error> {
        Reader::new_impl(file, buffer)
    }
    #[allow(clippy::missing_errors_doc)]
    pub fn open<P: AsRef<Path>>(
        path: P,
        buffer: &mut Buffer,
    ) -> Result<(Header<'_>, Reader), Error> {
        fn inner<'a>(path: &Path, buffer: &'a mut Buffer) -> Result<(Header<'a>, Reader), Error> {
            Reader::new_impl(File::open(path)?, buffer)
        }
        inner(path.as_ref(), buffer)
    }
    #[allow(clippy::missing_errors_doc)]
    pub fn read<'a>(&mut self, buffer: &'a mut Buffer) -> Result<Option<Item<'a>>, Error> {
        Ok(self.raw.read(&mut self.callback_data, buffer)?)
    }
    #[must_use]
    pub fn player_pos(&self, cid: i32) -> Option<Pos> {
        self.raw.player_pos(cid)
    }
    #[must_use]
    pub fn input(&self, cid: i32) -> Option<[i32; INPUT_LEN]> {
        self.raw.input(cid)
    }
    #[must_use]
    pub fn cids(&self) -> ops::Range<i32> {
        self.raw.cids()
    }
}

impl Callback for CallbackData {
    type Error = io::Error;
    fn read_at_most(&mut self, buffer: &mut [u8]) -> io::Result<Option<usize>> {
        match self.file.read(buffer) {
            Ok(0) => Ok(None),
            Ok(read) => Ok(Some(read)),
            Err(ref e) if e.kind() == io::ErrorKind::Interrupted => Ok(Some(0)),
            Err(e) => Err(e),
        }
    }
}
