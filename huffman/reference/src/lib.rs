extern crate libtw2_huffman_reference_sys as sys;

use libtw2_buffer as buffer;
use libtw2_buffer::with_buffer;
use libtw2_buffer::Buffer;
use libtw2_buffer::BufferRef;
use libtw2_common::num::Cast;

pub struct Huffman {
    huffman: Vec<u8>,
}

impl Huffman {
    #[allow(clippy::missing_panics_doc)]
    #[must_use]
    pub fn from_frequencies(frequencies: &[u32]) -> Huffman {
        assert!(frequencies.len() == 256);
        let array = unsafe { &*(frequencies.as_ptr().cast()) };
        Huffman::from_frequencies_array(array)
    }
    #[must_use]
    pub fn from_frequencies_array(frequencies: &[u32; 256]) -> Huffman {
        let huffman_size = unsafe { sys::huffman_size() };
        let huffman = Vec::with_capacity(huffman_size);
        let mut result = Huffman { huffman };
        // Implicit assumption that `c_uint == u32`. Screams when it breaks, so
        // it's fine.
        unsafe {
            sys::huffman_init(result.inner_huffman_mut(), frequencies);
        }
        result
    }
    #[allow(clippy::missing_errors_doc)]
    pub fn compress<'a, B: Buffer<'a>>(
        &self,
        input: &[u8],
        buffer: B,
    ) -> Result<&'a [u8], buffer::CapacityError> {
        with_buffer(buffer, |b| self.compress_impl(input, b))
    }
    fn compress_impl<'d>(
        &self,
        input: &[u8],
        mut buffer: BufferRef<'d, '_>,
    ) -> Result<&'d [u8], buffer::CapacityError> {
        let result_len = unsafe {
            sys::huffman_compress(
                self.inner_huffman(),
                input.as_ptr().cast(),
                input.len().assert_i32(),
                buffer.uninitialized_mut().as_mut_ptr().cast(),
                buffer.remaining().assert_i32(), // TODO: saturating conversion to i32?
            )
        };
        match result_len.try_usize() {
            Some(l) => unsafe {
                buffer.advance(l);
                Ok(buffer.initialized())
            },
            None => Err(buffer::CapacityError),
        }
    }
    #[allow(clippy::missing_errors_doc)]
    pub fn decompress<'a, B: Buffer<'a>>(
        &self,
        input: &'a [u8],
        buffer: B,
    ) -> Result<&'a [u8], libtw2_huffman::DecompressionError> {
        with_buffer(buffer, |b| self.decompress_impl(input, b))
    }
    fn decompress_impl<'d>(
        &self,
        input: &[u8],
        mut buffer: BufferRef<'d, '_>,
    ) -> Result<&'d [u8], libtw2_huffman::DecompressionError> {
        let result_len = unsafe {
            sys::huffman_decompress(
                self.inner_huffman(),
                input.as_ptr().cast(),
                input.len().assert_i32(),
                buffer.uninitialized_mut().as_mut_ptr().cast(),
                buffer.remaining().assert_i32(), // TODO: saturating conversion to i32?
            )
        };
        match result_len.try_usize() {
            Some(l) => unsafe {
                buffer.advance(l);
                Ok(buffer.initialized())
            },
            None => Err(libtw2_huffman::DecompressionError::Capacity(
                buffer::CapacityError,
            )),
        }
    }
    fn inner_huffman_mut(&mut self) -> *mut libc::c_void {
        self.huffman.as_mut_ptr().cast()
    }
    fn inner_huffman(&self) -> *const libc::c_void {
        self.huffman.as_ptr().cast()
    }
}
