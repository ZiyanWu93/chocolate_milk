#![feature(rustc_private,panic_info_message)]
#![no_std]
#![no_main]

mod core_reqs;

use core::panic::PanicInfo;

use serial::print;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    print!("PANIC:");

    if let Some(loc) = info.location() {
        print!(" {}:{}:{}", loc.file(), loc.line(),
               loc.column());
    }

    if let Some(msg) = info.message() {
        print!(" {}", msg);
    }

    print!("\n");

    cpu::halt();
}


#[no_mangle]
extern fn entry() {
    serial::init();
    let mem= [0u8;10];
    let mem= [0u8;10];
    print!("{:?}\n",mem[0]);
    print!("{:?}\n",mem[..][0]);
    print!("{:?}\n",mem[..][50]);
    // let val=5;
    // print!("Welcome to the chocolate milk! {:p}\n", &val);
    cpu::halt();
}

