#![no_std]
#![no_main]
#![feature(asm)]

use libuser::{fork, get_pid};

#[macro_use]
extern crate libuser;

#[no_mangle]
fn main() {
    println!("ready to fork");
    if fork() == 0 {
        println!("it's child process, pid is {}", get_pid());
    } else {
        println!("it's parent process, pid is {}", get_pid());
    }
}