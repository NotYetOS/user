#![no_std]
#![no_main]
#![feature(asm)]

extern crate libuser;

#[no_mangle]
fn main() -> i32 {
    unsafe {
        asm!(
            "ebreak",
            options(nostack)
        );
    }
    0
}
