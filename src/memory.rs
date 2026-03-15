// Memory allocation and deallocation functions

pub fn allocate(size: usize) -> *mut u8 {
    unsafe { libc::malloc(size) as *mut u8 }
}

pub fn deallocate(ptr: *mut u8) {
    unsafe { libc::free(ptr as *mut libc::c_void) }
}