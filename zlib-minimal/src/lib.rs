//! A minimal zlib wrapper
//!
//! This wrapper only exposes the `uncompress` method of zlib, both without
//! indirection and as idiomatic Rust function.

extern crate libz_sys as raw;

use libc::c_ulong;
use std::fmt;

#[inline]
#[allow(clippy::cast_possible_truncation)]
fn c_ulong_to_usize(v: c_ulong) -> usize {
    // On 32-bit platforms zlib's `uLong` can exceed `usize`; the API here is
    // already constrained by the destination slice length, so truncation is
    // effectively impossible in practice.
    v as usize
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Error {
    inner: i32,
}

impl Error {
    /// Convert a raw zlib return value into `Result`.
    ///
    /// # Errors
    ///
    /// Returns `Err` when `val` is not `Z_OK`.
    pub fn from_raw(val: i32) -> Result<(), Error> {
        if val == raw::Z_OK {
            Ok(())
        } else {
            Err(Error { inner: val })
        }
    }
    /// Classify this error when the underlying zlib error code is known.
    #[must_use]
    pub fn kind(self) -> Option<ErrorKind> {
        Some(match self.inner {
            raw::Z_MEM_ERROR => ErrorKind::OutOfMemory,
            raw::Z_BUF_ERROR => ErrorKind::OutputBufferTooSmall,
            raw::Z_DATA_ERROR => ErrorKind::InvalidInput,
            _ => return None,
        })
    }
    #[must_use]
    pub fn raw_error(self) -> i32 {
        self.inner
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ErrorKind {
    OutOfMemory,
    OutputBufferTooSmall,
    InvalidInput,
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind() {
            Some(k) => k.fmt(f),
            None => write!(f, "UnknownZlibError({})", self.raw_error()),
        }
    }
}

/// The wrapper for zlib's `uncompress` function.
///
/// Uncompresses the `src` parameter into the `dest` parameter and returning
/// the number of bytes written. If the decompression fails for some reason,
/// Err is returned. In this case, the `dest` buffer may or may not be
/// modified.
///
/// # Errors
///
/// Returns `Err` if zlib reports an error.
pub fn uncompress(dest: &mut [u8], src: &[u8]) -> Result<usize, Error> {
    let mut output_size = dest.len() as c_ulong;
    Error::from_raw(unsafe {
        raw::uncompress(
            dest.as_mut_ptr(),
            &mut output_size,
            src.as_ptr(),
            src.len() as c_ulong,
        )
    })
    .map(|()| c_ulong_to_usize(output_size))
}

/// The wrapper for zlib's `compress` function.
///
/// Compresses the `src` parameter into the `dest` parameter and returning the
/// number of bytes written. If the compression fails for some reason, Err is
/// returned. In this case, the `dest` buffer may or may not be modified.
///
/// # Errors
///
/// Returns `Err` if zlib reports an error.
pub fn compress(dest: &mut [u8], src: &[u8]) -> Result<usize, Error> {
    let mut output_size = dest.len() as c_ulong;
    Error::from_raw(unsafe {
        raw::compress(
            dest.as_mut_ptr(),
            &mut output_size,
            src.as_ptr(),
            src.len() as c_ulong,
        )
    })
    .map(|()| c_ulong_to_usize(output_size))
}

/// The wrapper for zlib's `compressBound` function.
///
/// Returns an upper bound on the compressed size for `compress()`.
#[must_use]
pub fn compress_bound(source_len: usize) -> usize {
    #[allow(clippy::cast_possible_truncation)]
    {
        c_ulong_to_usize(unsafe { raw::compressBound(source_len as c_ulong) })
    }
}

/// Compress data into a newly allocated `Vec`.
///
/// # Errors
///
/// Returns `Err` if zlib reports an error.
pub fn compress_vec(source: &[u8]) -> Result<Vec<u8>, Error> {
    let upper_bound = compress_bound(source.len());
    let mut dest = vec![0u8; upper_bound];

    let output_length = compress(&mut dest, source)?;
    dest.truncate(output_length);

    Ok(dest)
}
