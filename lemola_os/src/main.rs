#![no_std]
#![no_main]

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello World!";
static MESSAGE: &[u8] = b"Welcome to lemolaOS";

// #[no_mangle]  関数の名前を暗号化するのを防ぐ
// extern "C"  Cの呼び出し規則を使用する
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // this function is the entry point, since the linker looks for a function
    // named '_start' by default

    // cast to 'raw pointer'
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            // raw pointer
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    print(MESSAGE, HELLO.len() + 2);

    loop{}
}

fn print(str: &[u8], written: usize) {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in str.iter().enumerate() {
        unsafe {
            // raw pointer
            *vga_buffer.offset((written + i) as isize * 2) = byte;
            *vga_buffer.offset((written + i) as isize * 2 + 1) = 0xb;
        }
    }
}


// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}

