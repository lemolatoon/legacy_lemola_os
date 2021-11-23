#![no_std]
#![no_main]

use core::panic::PanicInfo;

// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}

// #[no_mangle]  関数の名前を暗号化するのを防ぐ
// extern "C"  Cの呼び出し規則を使用する
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop{}
}
