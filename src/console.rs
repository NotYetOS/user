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
    super::read(STDIN, &mut bytes);
    let ch_value = u32::from_le_bytes(bytes);
    match char::from_u32(ch_value) {
        Some(ch) => ch,
        None => 0 as char
    }
}
