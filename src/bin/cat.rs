#![no_std]
#![no_main]
#![feature(asm)]

use libuser::{
    OpenFlags,
    open, 
    read
};

#[macro_use]
extern crate libuser;

#[no_mangle]
fn main(argc: usize, argv: &[&str]) -> i32 {
    let mut buf = [0; 5120];
    if argc < 2 { 
        println!("please input arg");
        println!("format:");
        println!("cat <file-path>");
        return 0;
    }
    let fd = open(argv[1], OpenFlags::RDONLY);
    if fd == -1 {
        println!("cat: {}: No such file or directory", argv[1]);
        return -1;
    }
    let len = read(fd as usize, &mut buf) as usize;
    println!("{}", core::str::from_utf8(&buf[0..len]).unwrap());
    0
}
