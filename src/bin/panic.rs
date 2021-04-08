#![no_std]
#![no_main]

extern crate libuser;

#[no_mangle]
fn main() {
    panic!("test panic");
}
