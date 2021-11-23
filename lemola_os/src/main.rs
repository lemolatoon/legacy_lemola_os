#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod vga_buffer;

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
        break;
        unsafe {
            // raw pointer
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
            
        }
    }

    vga_buffer::print_something();


    

    loop{}
}

struct Printer {
    str_counter: usize,
}

impl Printer {
    fn new() -> Printer {
        Self {str_counter: 0}
    }

    fn print(&mut self, str: &[u8]) {
        let vga_buffer = 0xb8000 as *mut u8;

        for (i, &byte) in str.iter().enumerate() {
            unsafe {
                // raw pointer
                *vga_buffer.offset((self.str_counter + i) as isize * 2) = byte;
                *vga_buffer.offset((self.str_counter + i) as isize * 2 + 1) = 0xb;
                // self.print_byte(byte);
            }
        }
        self.str_counter += str.len();
    }

    fn print_byte(&mut self, byte: u8) {
        let vga_buffer = 0xb8000 as *mut u8;
        unsafe {
            // raw pointer
                *vga_buffer.offset(self.str_counter as isize * 2) = byte;
                *vga_buffer.offset(self.str_counter as isize * 2 + 1) = 0xb;
        }
        self.str_counter += 1;
    }

    fn panic(msg: &[u8]) -> usize {
        let mut printer = Self {str_counter: 900};
        printer.print(msg);
        return 0;
    }
}

fn num2byte(num: usize) -> u8 {
    let result = match num {
        0 => 48,
        1 => 49,
        2 => 50,
        3 => 51,
        4 => 52,
        5 => 53,
        6 => 54,
        7 => 55,
        8 => 56,
        9 => 57,
        _ => Printer::panic(b"illegal argument at num2VGA"),
    };

    result as u8
}



// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}

