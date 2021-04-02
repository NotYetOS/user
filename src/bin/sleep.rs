#![no_std]
#![no_main]

#[macro_use]
extern crate libuser;

use libuser::{
    _yield, 
    get_pid, 
    get_time_ms
};

#[no_mangle]
fn main() -> i32 {
    let current_timer = get_time_ms();
    let wait_for = current_timer + 500;
    while get_time_ms() < wait_for {
        _yield();
    }
    println!("pid = {}", get_pid());
    println!("Test sleep OK!");
    0
}