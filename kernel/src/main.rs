#![no_std]
#![no_main]

mod vga;

use core::fmt::Write;
use core::panic::PanicInfo;
use bootloader_api::{entry_point, BootloaderConfig};

static BOOTLOADER_CONFIG: BootloaderConfig = {
    let mut config = BootloaderConfig::new_default();
    config.frame_buffer.minimum_framebuffer_height = Some(720);
    config.kernel_stack_size = 100 * 1024;
    config
};

entry_point!(kernel_main, config = &BOOTLOADER_CONFIG);

#[panic_handler]
fn panic(_panic_info: &PanicInfo) -> ! {
    write!(vga::WRITER.lock(), "TacOS kernel panicked! hanging up...").unwrap();

    loop {}
}

fn kernel_main(_bootinfo: &'static mut bootloader_api::BootInfo) -> ! {
    panic!();
    write!(vga::WRITER.lock(), "hello bombel").unwrap();

    loop {}
}

