use crate::relative_size_of_mult;
use crate::slice;
use std::mem;

/// Reinterpret a `Vec<T>` as a `Vec<U>` without copying.
///
/// # Safety
///
/// The caller must ensure the same invariants as for [`crate::slice::transmute`],
/// and additionally that the allocation's alignment is valid for `U`.
#[must_use]
pub unsafe fn transmute<T, U>(vec: Vec<T>) -> Vec<U> {
    slice::transmute::<T, U>(&vec); // Error checking done there.

    let ptr = vec.as_ptr();
    let len = vec.len();
    let cap = vec.capacity();
    mem::forget(vec);
    let new_cap = cap * mem::size_of::<T>() / mem::size_of::<U>();

    // We "take ownership" of the allocation via `from_raw_parts`, so the raw
    // pointer must be mutable.
    let ptr = (ptr as *mut T).cast::<U>();
    Vec::from_raw_parts(ptr, relative_size_of_mult::<T, U>(len), new_cap)
}
