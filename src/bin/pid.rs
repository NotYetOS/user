#![no_std]
#![no_main]

#[macro_use]
extern crate libuser;

use libuser::getpid;

#[no_mangle]
fn main() -> i32 {
    println!("pid: {}", getpid());
    0
}
