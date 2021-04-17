#![no_std]
#![no_main]

#[macro_use]
extern crate libuser;

static TESTS: &[&str] = &[
    "hello",
    "matrix",
    "sleep",
    "pid",
    "forktest",
    "forktest2",
    "forktest_simple",
    "yield",
    "panic",
    "pipe",
    "ebreak",
    "power_3",
    "power_5",
    "power_7",
    "fork",
    "stack_overflow",
    "i_can_eat_glass",
    "cat /tlnb/test",
    "fantastic_text"
];

use libuser::{
    exec, 
    fork, 
    block_waitpid,
};

#[no_mangle]
pub fn main() -> i32 {
    for test in TESTS {
        println!("tests: Running {}", test);
        let pid = fork();
        if pid == 0 {
            exec(*test);
            panic!("unreachable!");
        } else {
            let mut exit_code: i32 = 0;
            let pid = block_waitpid(pid, &mut exit_code);
            println!(
                "\x1b[32mtests: Test {} in Process {} exited with code {}\x1b[0m", 
                test, 
                pid, 
                exit_code
            );
        }
    }
    println!("tests passed!");
    0
}
