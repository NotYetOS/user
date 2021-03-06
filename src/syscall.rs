#![allow(unused)]

const SYSCALL_DUP: usize = 24;
const SYSCALL_OPEN: usize = 56;
const SYSCALL_CLOSE: usize = 57;
const SYSCALL_PIPE: usize = 59;
const SYSCALL_READ: usize = 63;
const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;
const SYSCALL_YIELD: usize = 124;
const SYSCALL_GET_TIME: usize = 169;
const SYSCALL_GETPID: usize = 172;
const SYSCALL_FORK: usize = 220;
const SYSCALL_EXEC: usize = 221;
const SYSCALL_WAITPID: usize = 260;

use alloc::vec::Vec;
use alloc::string::String;

fn syscall(id: usize, args: [usize; 3]) -> isize {
    let ret: isize;

    // ecall to SMode
    unsafe {
        asm! {
            "ecall",
            lateout("x10") ret, 
            in("x10") args[0], in("x11") args[1], in("x12") args[2], 
            in("x17") id,
            options(nostack)
        }
    }

    ret
}

pub fn sys_dup(fd: usize) -> isize {
    syscall(SYSCALL_DUP, [fd, 0, 0])
}

pub fn sys_open(path: &str, len: usize, flags: u32) -> isize {
    syscall(SYSCALL_OPEN, [path.as_ptr() as usize, len, flags as usize])
}

pub fn sys_close(fd: usize) -> isize {
    syscall(SYSCALL_CLOSE, [fd, 0, 0])
}

pub fn sys_pipe(pipe: &mut [usize]) -> isize {
    syscall(SYSCALL_PIPE, [pipe.as_mut_ptr() as usize, 0, 0])
}

pub fn sys_read(fd: usize, buffer: &mut [u8]) -> isize {
    syscall(SYSCALL_READ, [fd, buffer.as_mut_ptr() as usize, buffer.len()])
}

pub fn sys_write(fd: usize, buffer: &[u8]) -> isize {
    syscall(SYSCALL_WRITE, [fd, buffer.as_ptr() as usize, buffer.len()])
}

pub fn sys_exit(exit_code: i32) -> ! {
    syscall(SYSCALL_EXIT, [exit_code as usize, 0, 0]);
    unreachable!("sys_exit never returns!")
}

pub fn sys_yield() -> isize {
    syscall(SYSCALL_YIELD, [0; 3])
}

pub fn sys_get_time() -> isize {
    syscall(SYSCALL_GET_TIME, [0; 3])
}

pub fn sys_getpid() -> isize {
    syscall(SYSCALL_GETPID, [0; 3])
}

pub fn sys_fork() -> isize {
    syscall(SYSCALL_FORK, [0; 3])
}

pub fn sys_exec(args: &str) -> isize {
    syscall(SYSCALL_EXEC, [
        args.as_ptr() as usize, 
        args.len(),
        0
    ])
}

pub fn sys_waitpid(pid: isize, exit_code: &mut i32) -> isize {
    syscall(SYSCALL_WAITPID, [pid as usize, exit_code as *mut i32 as usize, 0])
}
