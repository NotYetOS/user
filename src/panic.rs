use core::panic::PanicInfo;
use super::exit;

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
    exit(-1)
}
