#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;

// static HELLO: &[u8] = b"Hello World!";

#[no_mangle]
pub extern "C" fn _start() -> !{
    println!("Hello Clinio{}","!");
    print!("\n\n\n");
    println!("This is for the OS class");
    println!("\nLet's get those 400 points!!!!");
    panic!("My panic message");
    loop{}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}",_info);

    loop{}
}




