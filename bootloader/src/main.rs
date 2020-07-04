#![feature(rustc_private)]
#![no_std]
#![no_main]

mod core_reqs;

use core::panic::PanicInfo;

use serial::print;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}



#[no_mangle]
extern fn entry() {
    serial::init();
    print!("Welcome to the chocolate milk! {:?}\n", 2);
    cpu::halt();
}

