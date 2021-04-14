#![no_std]
#![feature(asm)]
#![feature(linkage)]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]
#![feature(assoc_char_funcs)]

#[macro_use]
pub mod console;
mod panic;
mod syscall;

#[macro_use]
extern crate bitflags;
extern crate alloc;

use buddy_system_allocator::LockedHeap;
use alloc::vec::Vec;

const USER_HEAP_SIZE: usize = 32768;
static mut HEAP_SPACE: [u8; USER_HEAP_SIZE] = [0; USER_HEAP_SIZE];

#[global_allocator]
static HEAP: LockedHeap = LockedHeap::empty();


#[alloc_error_handler]
pub fn handle_alloc_error(layout: core::alloc::Layout) -> ! {
    panic!("Heap allocation error, layout = {:?}", layout);
}

#[no_mangle]
#[link_section = ".text.entry"]
pub extern "C" fn _start(argc: usize, argv: usize) -> ! {
    unsafe {
        HEAP.lock()
            .init(HEAP_SPACE.as_ptr() as usize, USER_HEAP_SIZE);
    }
    let mut args = Vec::new();
    let mut ptr = argv;

    for _ in 0..argc {
        let str_start = ptr;
        let mut len = 0;

        for idx in 0.. {
            let value = ptr as *const u8;
            ptr += 1;
            if unsafe { *value == '\0' as u8 } {
                len = idx;
                break;
            }
        }

        let slice = unsafe { 
            core::slice::from_raw_parts(
                str_start as *const u8, 
                len
            )
        };

        args.push(
            core::str::from_utf8(
                slice
            ).unwrap()
        );
    }

    exit(main(argc, args.as_slice()))
}

#[no_mangle]
#[linkage = "weak"]
#[allow(unused)]
fn main(argc: usize, argv: &[&str]) -> i32 {
    panic!("Cannot find main!");
}

use syscall::*;

bitflags! {
    pub struct OpenFlags: u32 {
        const RDONLY = 0;
        const WRONLY = 1 << 0;
        const RDWR = 1 << 1;
        const CREATE = 1 << 9;
        const TRUNC = 1 << 10;
    }
}

pub fn read(fd: usize, buf: &mut [u8]) -> isize { sys_read(fd, buf) }
pub fn write(fd: usize, buf: &[u8]) -> isize { sys_write(fd, buf) }
pub fn open(path: &str, flags: OpenFlags) -> isize { sys_open(path, path.len(), flags.bits)}
pub fn close(fd: usize) -> isize { sys_close(fd) }
pub fn pipe(pipe_fd: &mut [usize]) -> isize { sys_pipe(pipe_fd) }
pub fn exit(exit_code: i32) -> ! { sys_exit(exit_code); }
pub fn get_time_ms() -> isize { sys_get_time() }
// yield is key word, so...
pub fn _yield() -> isize { sys_yield() }
pub fn getpid() -> isize { sys_getpid() }
pub fn fork() -> isize { sys_fork() }
pub fn exec(args: &str) -> isize { sys_exec(&args) }
pub fn wait(exit_code: &mut i32) -> isize {
    sys_waitpid(-1, exit_code)
}
pub fn waitpid(pid: isize, exit_code: &mut i32) -> isize {
    sys_waitpid(pid, exit_code)
}