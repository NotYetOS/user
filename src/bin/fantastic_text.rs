#![no_std]
#![no_main]

#[macro_use]
extern crate libuser;

#[no_mangle]
pub fn main() -> i32 {
    println!(
        "{}{}{}{}{} {}{}{}{}{}{}{}{}{} {}{}",
        color_text!("H", 31),
        color_text!("e", 32),
        color_text!("l", 33),
        color_text!("l", 34),
        color_text!("o", 35),
        color_text!("N", 36),
        color_text!("o", 37),
        color_text!("t", 90),
        color_text!("Y", 91),
        color_text!("e", 92),
        color_text!("t", 93),
        color_text!("O", 94),
        color_text!("S", 95),
        color_text!("!", 96),
        color_text!("你", 31),
        color_text!("好", 32),
    );

    let text =
        "reguler \x1b[4munderline\x1b[24m \x1b[7mreverse\x1b[27m \x1b[9mstrikethrough\x1b[29m";
    println!("\x1b[47m{}\x1b[0m", color_text!(text, 30));
    for i in 31..38 {
        println!("{}", color_text!(text, i));
    }
    for i in 90..98 {
        println!("{}", color_text!(text, i));
    }
    0
}