#![no_std]
#![feature(asm)]
#![feature(linkage)]
#![feature(panic_info_message)]

#[macro_use]
pub mod console;
mod panic;
mod syscall;

#[no_mangle]
#[link_section = ".text.entry"]
pub extern "C" fn _start() -> ! {
    exit(main())
}

#[no_mangle]
#[linkage = "weak"]
fn main() -> i32 {
    panic!("Cannot find main!");
}

use syscall::*;

pub fn read(fd: usize, buf: &mut [u8]) -> isize { sys_read(fd, buf) }
pub fn write(fd: usize, buf: &[u8]) -> isize { sys_write(fd, buf) }
pub fn exit(exit_code: i32) -> ! { sys_exit(exit_code); }
pub fn get_time_ms() -> isize { sys_get_time() }
// yield is key word, so...
pub fn _yield() -> isize { sys_yield() }
pub fn get_pid() -> isize { sys_getpid() }
pub fn fork() -> isize { sys_fork() }
