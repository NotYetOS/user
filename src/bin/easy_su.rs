#![no_std]
#![no_main]
#![feature(asm)]

#[macro_use]
extern crate libuser;

#[no_mangle]
fn main() -> i32 {
    println!("???");
    unsafe {
        asm!(
            "ebreak",
            options(nostack)
        );
    }
    0
}
