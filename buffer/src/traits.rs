use crate::with_buffer;
use crate::Buffer;
use crate::BufferRef;
use std::fs;
use std::io;
use std::io::Read;
use std::net;
use std::process;

/// A utility function for unsafely implementing `ReadBufferRef` for readers
/// that don't read the buffer passed to `Read::read`.
///
/// # Safety
///
/// The caller must ensure that `reader.read(..)` will not access the passed
/// buffer after returning. (Some `Read` impls ignore the passed buffer and use
/// an internal one instead.)
///
/// # Errors
///
/// Returns the underlying `Read::read` I/O error.
pub unsafe fn read_buffer_ref<'d, T: Read>(
    reader: &mut T,
    mut buf: BufferRef<'d, '_>,
) -> io::Result<&'d [u8]> {
    let read = reader.read(buf.uninitialized_mut())?;
    buf.advance(read);
    Ok(buf.initialized())
}

/// An internal trait to be implemented by `T: Read` which do not access the
/// read buffer in `Read::read`.
///
/// # Safety
///
/// Implementors must not access the read buffer passed to `Read::read` after
/// returning from `read`.
pub unsafe trait ReadBufferMarker: Read {}

/// An internal trait to be implemented by `T: Read` which do not access the
/// read buffer in `Read::read`. Prefer implementing `ReadBufferMarker` over
/// this.
pub trait ReadBufferRef: Read {
    /// Reads (equivalently to `Read::read`) into the buffer ref and returns
    /// the newly written bytes.
    ///
    /// # Errors
    ///
    /// Returns the underlying `Read::read` I/O error.
    fn read_buffer_ref<'d>(&mut self, buf: BufferRef<'d, '_>) -> io::Result<&'d [u8]>;
}

/// Trait to read to `T: Buffer`.
///
/// This trait should be imported to read into buffers.
pub trait ReadBuffer: ReadBufferRef {
    /// Reads (equivalently to `Read::read`) into the buffer and returns the
    /// newly read bytes.
    ///
    /// # Errors
    ///
    /// Returns the underlying `Read::read` I/O error.
    fn read_buffer<'d, B: Buffer<'d>>(&mut self, buf: B) -> io::Result<&'d [u8]> {
        with_buffer(buf, |buf| self.read_buffer_ref(buf))
    }
}

impl<T: ReadBufferRef> ReadBuffer for T {}
impl<T: ReadBufferMarker> ReadBufferRef for T {
    fn read_buffer_ref<'d>(&mut self, buf: BufferRef<'d, '_>) -> io::Result<&'d [u8]> {
        unsafe { read_buffer_ref(self, buf) }
    }
}

unsafe impl ReadBufferMarker for fs::File {}
unsafe impl ReadBufferMarker for io::Empty {}
unsafe impl ReadBufferMarker for io::Repeat {}
unsafe impl ReadBufferMarker for io::Stdin {}
unsafe impl ReadBufferMarker for net::TcpStream {}
unsafe impl ReadBufferMarker for process::ChildStderr {}
unsafe impl ReadBufferMarker for process::ChildStdout {}
unsafe impl ReadBufferMarker for &[u8] {}
unsafe impl ReadBufferMarker for &fs::File {}
unsafe impl ReadBufferMarker for &net::TcpStream {}
unsafe impl ReadBufferMarker for io::StdinLock<'_> {}
unsafe impl<R> ReadBufferMarker for io::Take<R> where R: ReadBufferMarker {}
unsafe impl<R> ReadBufferMarker for io::BufReader<R> where R: ReadBufferMarker {}
unsafe impl<T, U> ReadBufferMarker for io::Chain<T, U>
where
    T: ReadBufferMarker,
    U: ReadBufferMarker,
{
}

unsafe impl<R> ReadBufferMarker for &mut R where R: ReadBufferMarker {}
unsafe impl<R> ReadBufferMarker for Box<R> where R: ReadBufferMarker {}
