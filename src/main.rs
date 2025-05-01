#![no_std]

use core::panic::PanicInfo;

/// Function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

fn main() {}
