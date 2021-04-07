#![no_std]
#![no_main]

#[macro_use]
extern crate libuser;
extern crate alloc;

use alloc::string::String;
use libuser::{console::getchar, waitpid};
use libuser::{
    _yield, 
    exec, 
    exit, 
    fork, 
};

const LF: u8 = 0x0au8;
const CR: u8 = 0x0du8;
const DL: u8 = 0x7fu8;
const BS: u8 = 0x08u8;

#[no_mangle]
fn main() -> ! {
    print!("-> ~ ");
    let mut line: String = String::new();

    loop {
        let ch = getchar();
        match ch {
            LF | CR => {
                println!("");
                if line.eq("exit") {
                    exit(0);
                }

                if !line.is_empty() {
                    let pid = fork();
                    if pid == 0 {
                        let ret = exec(&line);
                        if ret == -1 {
                            println!("{}: command not found", line);
                            exit(-1);
                        }
                    } else {
                        let mut exit_code: i32 = 0;
                        block_wait(pid, &mut exit_code);
                    } 
                    line.clear();
                }
                print!("-> ~ ");
            },
            BS | DL => {
                if !line.is_empty() {
                    print!("{}", BS as char);
                    print!(" ");
                    print!("{}", BS as char);
                    line.pop();
                }
            }
            _ => {
                let ch = ch as char;
                if !ch.is_ascii_control() {
                    print!("{}", ch);
                    line.push(ch);
                }
            }
        }
    }
}

fn block_wait(pid: isize, exit_code: &mut i32) {
    loop {
        match waitpid(pid, exit_code) {
            -2 => { _yield(); },
            pid @ _ => { 
                println!(
                    "Process {} exited with code {}", 
                    pid, 
                    exit_code
                );
                break;
            }
        };
    }
}
