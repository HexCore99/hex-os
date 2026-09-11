#![no_std]
#![no_main]
use core::panic::PanicInfo;
mod vga_buffer;

#[unsafe(no_mangle)] //tells compiler not to rename the function name.
pub extern "C" fn _start() -> ! {
    //Make this Rust function behave like a normal C function at the machine-code level

    println!("Hello World {}", "!");
    panic!("Some Panic message!");
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
