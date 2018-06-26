#[no_mangle]
pub extern fn crc32(ptr: *mut u8, length: u32) -> u32 {
  unsafe {
    let buf : &[u8] = std::slice::from_raw_parts(ptr, length as usize);
    0
  }
}