#![no_std]
#![no_main]

mod kernel;
mod limine;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() -> ! {
    if !limine::base_revision_supported() {
        loop {}
    }

    kernel::lib::init::sys_init();
    kernel::lib::consoleio::print(b"Test output");
    loop {}
}
