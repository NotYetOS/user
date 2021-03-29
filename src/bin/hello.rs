#![no_std]
#![no_main]

#[macro_use]
extern crate libuser;

#[no_mangle]
fn main() -> i32 {
    println!("hello from user mode");
    println!("你好你好");
    println!("1 + 1 = {}(二)", 1 + 1);
    0
}
