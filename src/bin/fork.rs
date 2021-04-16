#![no_std]
#![no_main]
#![feature(asm)]

use libuser::{
    fork, 
    getpid, 
    block_waitpid,
};

#[macro_use]
extern crate libuser;

#[no_mangle]
fn main() -> i32 {
    println!("ready to fork");
    let pid = fork();
    if pid == 0 {
        println!("it's child process, pid is {}", getpid());
        0
    } else {
        println!("it's parent process, pid is {}", getpid());
        let mut exit_code: i32 = 0;
        block_waitpid(pid, &mut exit_code);
        assert_eq!(exit_code, 0);
        0
    }
}
