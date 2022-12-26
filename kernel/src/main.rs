#![no_std]
#![no_main]

use core::panic::PanicInfo;
use bootloader_api::{entry_point, BootloaderConfig};

static BOOTLOADER_CONFIG: BootloaderConfig = {
    let mut config = BootloaderConfig::new_default();
    config.frame_buffer.minimum_framebuffer_height = Some(720);
    config.kernel_stack_size = 100 * 1024;
    config
};

entry_point!(kernel_main, config = &BOOTLOADER_CONFIG);

static PANIC_MESSAGE: &[u8] = b"tacOS kernel paniced! haning up...";

#[panic_handler]
fn panic(_panic_info: &PanicInfo) -> ! {
    let vga_buffer = 0xb8000 as *mut u8;
    for (i, &byte) in PANIC_MESSAGE.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}

static MESSAGE: &[u8] = b"Os Bombelka!!!!";

fn kernel_main(_bootinfo: &'static mut bootloader_api::BootInfo) -> ! {
    let vga_buffer = 0xb8000 as *mut u8;
    for (i, &byte) in MESSAGE.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}

