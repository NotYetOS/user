#![no_std]
#![no_main]

#[macro_use]
extern crate libuser;

use libuser::{
    getpid, 
    _yield
};

#[no_mangle]
pub fn main() -> i32 {
    println!("Hello, I am process {}.", getpid());
    for i in 0..5 {
        _yield();
        println!("Back in process {}, iteration {}.", getpid(), i);
    }
    println!("yield pass.");
    0
}