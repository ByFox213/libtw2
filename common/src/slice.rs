use std::mem;
use std::slice;

/// Compute `mult * size_of::<T>() / size_of::<U>()`.
///
/// # Panics
///
/// Panics if `size_of::<U>() == 0` or if the division is not exact.
#[must_use]
pub fn relative_size_of_mult<T, U>(mult: usize) -> usize {
    // Panics if mem::size_of::<U>() is 0.
    assert!(mult * mem::size_of::<T>() % mem::size_of::<U>() == 0);
    mult * mem::size_of::<T>() / mem::size_of::<U>()
}

#[must_use]
pub fn relative_size_of<T, U>() -> usize {
    relative_size_of_mult::<T, U>(1)
}

/// Reinterpret a slice of `T` as a slice of `U`.
///
/// # Safety
///
/// The caller must ensure:
/// - `x` is properly aligned for `U`.
/// - `x`'s byte length is a multiple of `size_of::<U>()`.
///
/// # Panics
///
/// Panics if the alignment or size constraints are violated.
pub unsafe fn transmute<T, U>(x: &[T]) -> &[U] {
    assert!(mem::align_of::<T>() % mem::align_of::<U>() == 0);
    slice::from_raw_parts(
        x.as_ptr().cast::<U>(),
        relative_size_of_mult::<T, U>(x.len()),
    )
}

/// Reinterpret a mutable slice of `T` as a mutable slice of `U`.
///
/// # Safety
///
/// Same requirements as [`transmute`], plus the resulting `&mut [U]` must not
/// alias any other references for the duration of its borrow.
pub unsafe fn transmute_mut<T, U>(x: &mut [T]) -> &mut [U] {
    transmute::<T, U>(x); // For the error checking.
    slice::from_raw_parts_mut(
        x.as_mut_ptr().cast::<U>(),
        relative_size_of_mult::<T, U>(x.len()),
    )
}
