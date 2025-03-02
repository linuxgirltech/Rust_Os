#![no_std] // don't link the Rust standard Library
#![no_main] // disable all Rust-level entry points
#![feature(asm)] // Inline Assembly

use core::panic::PanicInfo;

// Modules
mod vga_buffer;

// This function is called on panic
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

static HELLO: &[u8] = b"Hello World!";

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() ->! {
    // This funciton is the entry point, since the linker look for a function named
    // `_start` by default
    println!("Hello World{}", "!");
    panic!("Don't panic!");

    loop {}
}
