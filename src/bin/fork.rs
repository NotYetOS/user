#![no_std]
#![no_main]
#![feature(asm)]

use libuser::{
    fork, 
    getpid
};

#[macro_use]
extern crate libuser;

#[no_mangle]
fn main() -> i32 {
    println!("ready to fork");
    if fork() == 0 {
        println!("it's child process, pid is {}", getpid());
    } else {
        println!("it's parent process, pid is {}", getpid());
    }
    0
}
