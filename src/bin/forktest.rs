#![no_std]
#![no_main]

#[macro_use]
extern crate libuser;

use libuser::{
    fork, 
    block_wait, 
    exit,
};

const MAX_CHILD: usize = 25;

#[no_mangle]
pub fn main() -> i32 {
    for i in 0..MAX_CHILD {
        let pid = fork();
        if pid == 0 {
            println!("I am child {}", i);
            exit(0);
        } else {
            println!("forked child pid = {}", pid);
        }
        assert!(pid > 0);
    }
    let mut exit_code: i32 = 0;
    for _ in 0..MAX_CHILD {
        if block_wait(&mut exit_code) <= 0 {
            panic!("wait stopped early");
        }
    }
    if block_wait(&mut exit_code) > 0 {
        panic!("wait got too many");
    }
    println!("forktest pass.");
    0
}