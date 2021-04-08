#![no_std]
#![no_main]

#[macro_use]
extern crate libuser;

use libuser::{
    _yield, 
    close, 
    fork, 
    pipe, 
    read, 
    wait, 
    write
};

#[no_mangle]
pub fn main() -> i32 {
    let test_str = "停留牛逼，冲啊";
    let mut pipe_fd = [0; 2];
    pipe(&mut pipe_fd);

    if fork() == 0 {
        println!("it's child process");
        close(pipe_fd[1]);
        let mut buf = [0; 32];
        let len = read(pipe_fd[0], &mut buf) as usize;
        close(pipe_fd[0]);
        let pipe_str = core::str::from_utf8(&buf[0..len]).unwrap();
        assert_eq!(test_str, pipe_str);
        println!("{}", pipe_str);
        println!("read ok, child process exited!");
        0
    } else {
        close(pipe_fd[0]);
        assert_eq!(write(pipe_fd[1], test_str.as_bytes()), test_str.len() as isize);
        close(pipe_fd[1]);
        let mut exit_code: i32 = 0;
        loop {
            match wait(&mut exit_code) {
                -2 => { _yield(); },
                 _ => { 
                    break;
                }
            };
        }
        assert_eq!(exit_code, 0);
        println!("pipetest passed!");
        0
    }
}
