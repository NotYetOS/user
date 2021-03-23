#![no_std]
#![no_main]

#[macro_use]
extern crate libuser;

#[no_mangle]
fn main() -> i32 {
    println!("hello from user mode");
    0
}
