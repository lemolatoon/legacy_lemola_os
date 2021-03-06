#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod uefi;

#[entry]
fn efi_main() {
    loop{}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}
