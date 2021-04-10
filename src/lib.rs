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

extern crate alloc;

use buddy_system_allocator::LockedHeap;

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
pub extern "C" fn _start() -> ! {
    unsafe {
        HEAP.lock()
            .init(HEAP_SPACE.as_ptr() as usize, USER_HEAP_SIZE);
    }
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
pub fn close(fd: usize) -> isize { sys_close(fd) }
pub fn pipe(pipe_fd: &mut [usize]) -> isize { sys_pipe(pipe_fd) }
pub fn exit(exit_code: i32) -> ! { sys_exit(exit_code); }
pub fn get_time_ms() -> isize { sys_get_time() }
// yield is key word, so...
pub fn _yield() -> isize { sys_yield() }
pub fn getpid() -> isize { sys_getpid() }
pub fn fork() -> isize { sys_fork() }
pub fn exec(path: &str) -> isize { sys_exec(path) }
pub fn wait(exit_code: &mut i32) -> isize {
    sys_waitpid(-1, exit_code as *mut _)
}
pub fn waitpid(pid: isize, exit_code: &mut i32) -> isize {
    sys_waitpid(pid, exit_code as *mut _)
}