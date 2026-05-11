use arrayvec::ArrayVec;
use std::ascii;
use std::fmt;
use std::ops;
use std::str;

pub struct AlmostString([u8]);

impl AlmostString {
    #[must_use]
    pub fn new(bytes: &[u8]) -> &AlmostString {
        unsafe { &*(bytes as *const [u8] as *const AlmostString) }
    }
}

pub struct AlmostStringSlice<'a>([&'a [u8]]);

impl<'a> AlmostStringSlice<'a> {
    #[must_use]
    pub fn new<'b>(bytes_slice: &'b [&'a [u8]]) -> &'b AlmostStringSlice<'a> {
        unsafe { &*(bytes_slice as *const [&'a [u8]] as *const AlmostStringSlice<'a>) }
    }
}

pub struct Bytes([u8]);

impl Bytes {
    #[must_use]
    pub fn new(bytes: &[u8]) -> &Bytes {
        unsafe { &*(bytes as *const [u8] as *const Bytes) }
    }
}

pub struct BytesSlice<'a>([&'a [u8]]);

impl<'a> BytesSlice<'a> {
    #[must_use]
    pub fn new<'b>(bytes_slice: &'b [&'a [u8]]) -> &'b BytesSlice<'a> {
        unsafe { &*(bytes_slice as *const [&'a [u8]] as *const BytesSlice<'a>) }
    }
}

struct Byte {
    string: ArrayVec<[u8; 4]>,
}

impl Byte {
    fn new(byte: u8) -> Byte {
        let mut string = ArrayVec::new();
        if byte == b'\\' || byte == b'\"' {
            string.push(b'\\');
            string.push(byte);
        } else {
            string.extend(ascii::escape_default(byte));
        }
        Byte { string }
    }
}

impl ops::Deref for Byte {
    type Target = str;
    fn deref(&self) -> &str {
        unsafe { str::from_utf8_unchecked(&self.string) }
    }
}

impl fmt::Debug for Bytes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("b\"")?;
        for &byte in &self.0 {
            f.write_str(&Byte::new(byte))?;
        }
        f.write_str("\"")?;
        Ok(())
    }
}

impl fmt::Debug for BytesSlice<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_list()
            .entries(self.0.iter().copied().map(Bytes::new))
            .finish()
    }
}

impl fmt::Debug for AlmostString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // FIXME: Replace this with a UTF-8 decoder.
        let string = String::from_utf8_lossy(&self.0);
        fmt::Debug::fmt(&string, f)
    }
}

impl fmt::Display for AlmostString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // FIXME: Replace this with a UTF-8 decoder.
        let string = String::from_utf8_lossy(&self.0);
        if string.chars().any(|c| c < ' ' || c == '"') {
            fmt::Debug::fmt(&string, f)
        } else {
            fmt::Display::fmt(&string, f)
        }
    }
}

impl fmt::Debug for AlmostStringSlice<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_list()
            .entries(self.0.iter().copied().map(AlmostString::new))
            .finish()
    }
}
