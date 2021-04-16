#![no_std]
#![no_main]

#[macro_use]
extern crate libuser;
extern crate alloc;

use alloc::string::String;
use libuser::console::getchar;
use libuser::{
    exec, 
    exit, 
    fork, 
    block_waitpid,
};

const LF: u8 = 0x0au8;
const CR: u8 = 0x0du8;
const DL: u8 = 0x7fu8;
const BS: u8 = 0x08u8;

// emm, laji codes, but it works
#[no_mangle]
fn main() -> ! {
    print!("-> ~ ");
    let mut line: String = String::new();
    
    loop {
        let ch = getchar();
        if ch.is_ascii() {
            match ch as u8 {
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
                            let pid = block_waitpid(pid, &mut exit_code);
                            println!(
                                "Process {} exited with code {}", 
                                pid, 
                                exit_code
                            );
                        } 
                        line.clear();
                    }

                    print!("-> ~ ");
                },
                BS | DL => {
                    if !line.is_empty() {
                        let ch = line.pop().unwrap();
                        if ch.is_ascii() {
                            print!("{}", BS as char);
                            print!(" ");
                            print!("{}", BS as char);
                        } else {
                            print!("{}", BS as char);
                            print!("{}", BS as char);
                            print!(" ");
                            print!(" ");
                            print!("{}", BS as char);
                            print!("{}", BS as char);
                        }
                    }
                }
                _ => {
                    if !ch.is_ascii_control() {
                        print!("{}", ch);
                        line.push(ch);
                    }
                }
            }
        } else {
            print!("{}", ch);
            line.push(ch);
        }
    }
}
