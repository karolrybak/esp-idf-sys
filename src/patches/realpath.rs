use core::ffi;
use crate::bindings;

static mut __REALPATH_INTERNAL_REFERENCE: *mut ffi::c_void = realpath as *mut _;

pub fn link_patches() -> *mut ffi::c_void {
    unsafe { __REALPATH_INTERNAL_REFERENCE }
}

#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn realpath(path: *const ffi::c_char, resolved_path: *mut ffi::c_char) -> *mut ffi::c_char {
    // A very simple realpath implementation for ESP-IDF/Newlib
    // It doesn't actually canonicalize, just copies the path to resolved_path.
    // If resolved_path is NULL, it's supposed to allocate, but we don't handle that here for simplicity,
    // as it's just a stub to satisfy the linker for Rust's std.
    
    if path.is_null() {
        return core::ptr::null_mut();
    }
    
    if !resolved_path.is_null() {
        // Assume resolved_path has enough space (at least PATH_MAX, usually 4096 on POSIX, but smaller on ESP-IDF)
        // In ESP-IDF, we can't easily know PATH_MAX if not defined.
        // We'll just do a basic copy.
        let mut i = 0;
        loop {
            let c = *path.add(i);
            *resolved_path.add(i) = c;
            if c == 0 { break; }
            i += 1;
        }
        resolved_path
    } else {
        // If resolved_path is NULL, we should return NULL or a pointer to Allocated memory.
        // Rust's std usually passes a buffer.
        core::ptr::null_mut()
    }
}
