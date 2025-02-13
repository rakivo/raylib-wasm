/// Took from the standard library of [Rust](https://github.com/rust-lang/rust)

use std::{ptr, slice};
use std::mem::MaybeUninit;
use std::ffi::{CStr, CString};

// Make sure to stay under 4096 so the compiler doesn't insert a probe frame:
// https://docs.rs/compiler_builtins/latest/compiler_builtins/probestack/index.html
#[cfg(not(target_os = "espidf"))]
const MAX_STACK_ALLOCATION: usize = 384;
#[cfg(target_os = "espidf")]
const MAX_STACK_ALLOCATION: usize = 32;

#[inline]
pub fn run_with_cstr(bytes: &[u8], f: impl FnOnce(&CStr)) {
    // Dispatch and dyn erase the closure type to prevent mono bloat.
    // See https://github.com/rust-lang/rust/pull/121101.
    if bytes.len() >= MAX_STACK_ALLOCATION {
        run_with_cstr_allocating(bytes, f)
    } else {
        unsafe { run_with_cstr_stack(bytes, f) }
    }
}

/// # Safety
///
/// `bytes` must have a length less than `MAX_STACK_ALLOCATION`.
unsafe fn run_with_cstr_stack(
    bytes: &[u8],
    f: impl FnOnce(&CStr),
) {
    let mut buf = MaybeUninit::<[u8; MAX_STACK_ALLOCATION]>::uninit();
    let buf_ptr = buf.as_mut_ptr() as *mut u8;

    unsafe {
        ptr::copy_nonoverlapping(bytes.as_ptr(), buf_ptr, bytes.len());
        buf_ptr.add(bytes.len()).write(0);
    }

    match CStr::from_bytes_with_nul(unsafe { slice::from_raw_parts(buf_ptr, bytes.len() + 1) }) {
        Ok(s) => f(s),
        Err(_) => panic!("file name contained an unexpected NUL byte")
    }
}

#[inline]
fn run_with_cstr_allocating(bytes: &[u8], f: impl FnOnce(&CStr)) {
    match CString::new(bytes) {
        Ok(s) => f(&s),
        Err(_) => panic!("file name contained an unexpected NUL byte")
    }
}
