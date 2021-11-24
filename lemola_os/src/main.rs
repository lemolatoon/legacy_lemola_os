#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::fmt;

mod vga_buffer;

static HELLO: &str = "Hello World!";
static MESSAGE: &str = "Welcome to lemolaOS";

// #[no_mangle]  関数の名前を暗号化するのを防ぐ
// extern "C"  Cの呼び出し規則を使用する
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // this function is the entry point, since the linker looks for a function
    // named '_start' by default

    use core::fmt::Write;
    vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
    writeln!(vga_buffer::WRITER.lock(), ", some numbers: {} {}", 42, 1.337).unwrap();
    writeln!(vga_buffer::WRITER.lock(), "Is the last sentence ended with new_line(\\n)?").unwrap();

    let mut printer = Printer::new();
    writeln!(printer, "{}", HELLO).unwrap();
    writeln!(printer, "{}", MESSAGE).unwrap();

    printer.str_counter = 1000;
    writeln!(printer, "1 + 1 = {}", 1 + 1).unwrap();

    println!();
    println!("above is new_line from `println!` macro");
    println!("writing from `println!` macro + some numbers {}", 3.14 + 45435.0);
    print!("Hello from `print!` macro  {}", 5);
    println!();

    // panic!("panic was caused");
    

    loop{}
}

struct Printer {
    str_counter: usize,
}

impl Printer {
    fn new() -> Printer {
        Self {str_counter: 0}
    }

    fn print(&mut self, str: &str) {
        let vga_buffer = 0xb8000 as *mut u8;

        for byte in str.bytes() {
            self.print_byte(byte);
        }
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

    fn panic(msg: &str) -> usize {
        let mut printer = Self {str_counter: 900};
        printer.print(msg);
        return 0;
    }
}

impl fmt::Write for Printer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.print(s);
        Ok(())
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
        _ => Printer::panic("illegal argument at num2VGA"),
    };

    result as u8
}



// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop{}
}

