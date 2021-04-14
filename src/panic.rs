use core::panic::PanicInfo;
use crate::sys_exit;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    match info.location() {
        Some(location) => {
            println!(
                "Panicked at '{}', {}:{}:{}", 
                info.message().unwrap(),
                location.file(), 
                location.line(),
                location.column()
            );
        }
        None => println!("Panicked at '{}'", info.message().unwrap())
    }
    sys_exit(-1)
}
