#![no_std]
#![no_main]
#![feature(asm)]

use alloc::string::String;

#[macro_use]
extern crate libuser;
extern crate alloc;

#[no_mangle]
fn main(argc: usize, argv: &[&str]) -> i32 {
    if argc < 2 {
        println!("please input arg");
        return -1;
    }
    let mut echo_value = String::new();
    argv[1..].iter().for_each(|arg| {
        echo_value.push_str(arg);
        echo_value.push(' ');
    });
    echo_value.pop();
    println!("{}", echo_value);
    0
}
