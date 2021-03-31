#![no_std]
#![no_main]

#[macro_use]
extern crate libuser;

#[no_mangle]
fn main() -> i32 {
    println!("hello from user mode");
    println!("来自用户模式的问候");
    println!("顺便测试一下高级的中文能否显示");
    0
}
