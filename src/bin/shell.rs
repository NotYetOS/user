#![no_std]
#![no_main]

#[macro_use]
extern crate libuser;
extern crate alloc;

use alloc::vec::Vec;
use alloc::string::String;
use libuser::console::get_byte;
use libuser::{
    _yield, 
    exec, 
    exit, 
    fork, 
    waitpid,
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
    let mut char_buf = CharBuffer::new();
    
    loop {
        let ch = char_buf.char();
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
                            block_wait(pid, &mut exit_code);
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

pub struct CharBuffer {
    inner: Vec<u8>,
    str_data: String,
}

impl CharBuffer {
    pub fn new() -> Self {
        Self {
            inner: Vec::new(),
            str_data: String::new()
        }
    }

    pub fn get_bytes(&mut self) {
        loop {
            let byte = get_byte();
            if byte as i8 != -1 {
                self.inner.push(byte);
                break;
            }
        }

        loop {
            let byte = get_byte();
            if byte as i8 == -1 {
                break;
            } else {
                self.inner.push(byte);
            }
        }
    }

    pub fn char(&mut self) -> char {
        if !self.str_data.is_empty() {
            self.str_data.remove(0)
        } else {
            self.get_bytes();
            let len = self.inner.len();
            self.str_data = core::str::from_utf8(&self.inner[0..len]).unwrap().into();
            self.inner.clear();
            self.str_data.remove(0)
        }
    }
}
