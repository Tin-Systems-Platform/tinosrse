#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
mod kernel;
use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)] // don't mangle the name of this function
pub extern "C" fn kernel_main(multiboot_info_ptr: usize, magic: usize) -> ! {

    if magic != 0x36D76289 {
        loop {}
    }

    let boot_info_ptr = multiboot_info_ptr as *const _;
    let boot_info = unsafe { 
        multiboot2::BootInformation::load(boot_info_ptr)
            .expect("Multiboot2-info parsing failed!") 
    };

    // this function is the entry point, since the linker looks for a function
    // named `_start` by default
    kernel::lib::init::sys_init();
    loop {}
}