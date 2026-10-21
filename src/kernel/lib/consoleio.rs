pub fn print(msg: &[u8]) {
    let Some(hhdm_offset) = crate::limine::hhdm_offset() else {
        loop {}
    };
    let vga_buffer = (hhdm_offset + 0xb8000) as *mut u8;

    for (i, &byte) in msg.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
}