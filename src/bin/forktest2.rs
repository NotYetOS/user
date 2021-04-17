#![no_std]
#![no_main]

#[macro_use]
extern crate libuser;

use libuser::{
    fork, 
    block_wait, 
    getpid, 
    exit, 
    sleep, 
    get_time_ms
};

static NUM: usize = 25;

#[no_mangle]
pub fn main() -> i32 {
    for _ in 0..NUM {
        let pid = fork();
        if pid == 0 {
            let current_time = get_time_ms();
            let sleep_length = (current_time as i32 as isize) * (current_time as i32 as isize) % 1000 + 1000;
            println!("pid {} sleep for {} ms", getpid(), sleep_length);
            sleep(sleep_length as usize);
            println!("pid {} OK!", getpid());
            exit(0);
        }
    }

    let mut exit_code: i32 = 0;
    for _ in 0..NUM {
        assert!(block_wait(&mut exit_code) > 0);
        assert_eq!(exit_code, 0);
    }
    assert!(block_wait(&mut exit_code) < 0);
    println!("forktest2 test passed!");
    0
}