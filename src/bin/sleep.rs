#![no_std]
#![no_main]

#[macro_use]
extern crate libuser;

use libuser::{
    block_waitpid, 
    exit, 
    fork, 
    get_time_ms, 
    sleep
};

fn sleepy() {
    let time: usize = 1000;
    for i in 0..5 {
        sleep(time);
        println!("sleep {} x {} msecs.", i + 1, time);
    }
    exit(0);
}

#[no_mangle]
pub fn main() -> i32 {
    let current_time = get_time_ms();
    let pid = fork();
    let mut exit_code: i32 = 0;
    if pid == 0 {
        sleepy();
    }
    assert!(block_waitpid(pid, &mut exit_code) == pid && exit_code == 0);
    println!("use {} msecs.", get_time_ms() - current_time);
    println!("sleep pass.");
    0
}