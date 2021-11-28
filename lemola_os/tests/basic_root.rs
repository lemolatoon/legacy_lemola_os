#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(lemola_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

#[test_case]
fn test_of_test() {
    assert_eq!(1.0, 1.0);
}

// called before various kinds of initialization called in `_start` in main.rs
#[test_case]
fn test_println() {
    use lemola_os::println;
    println!("test_println output");
}


#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    lemola_os::test_panic_handler(info);
}
