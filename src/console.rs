use core::fmt::{
    self, 
    Write
};

const STDIN: usize = 0;
const STDOUT: usize = 1;

struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        super::write(STDOUT, s.as_bytes());
        Ok(())
    }
}

pub fn print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!($fmt $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}

// utf8, 4 bytes
pub fn getchar() -> char {
    let mut bytes = [0; 4];
    
    let len = loop {
        let ret = super::read(
            STDIN, 
            &mut bytes
        );
        if ret != -1 && ret != -2 {
            break ret as usize;
        }
    };

    let ch = core::str::from_utf8(
        &bytes[0..len]
    ).unwrap();
    ch.chars().nth(0).unwrap()
}
