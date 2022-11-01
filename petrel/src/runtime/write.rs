unsafe fn write<T>(dest: *const u8, object: T) {
    std::ptr::write(dest as *mut T, object);
}
