#![no_std]
#![no_main]

mod vga;

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
fn panic(panic_info: &PanicInfo) -> ! {
    vga::WRITER.lock().set_cursor_color(vga::ColorCode::new(
        vga::Color::White,
        vga::Color::Black,
    ));

    println!("{}", panic_info);

    loop {}
}

fn kernel_main(_bootinfo: &'static mut bootloader_api::BootInfo) -> ! {
    println!("hello world\n");

    loop {}
}

