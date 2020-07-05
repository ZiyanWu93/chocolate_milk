#![feature(rustc_private, panic_info_message)]
#![no_std]
#![no_main]

mod core_reqs;
mod realmode;

use core::panic::PanicInfo;

use serial::print;
use crate::realmode::{invoke_realmode, RegisterState};

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
    // let mem= [0u8;10];
    // let mem= [0u8;10];
    // print!("{:?}\n",mem[0]);
    // print!("{:?}\n",mem[..][0]);
    // print!("{:?}\n",mem[..][50]);
    // let val=5;
    // print!("Welcome to the chocolate milk! {:p}\n", &val);
    unsafe {
        #[derive(Debug, Default)]
        #[repr(C)]
        struct E820Entry {
            base: u64,
            length: u64,
            typ: u32,
        }

        let mut cont = 0;
        let mut regs = RegisterState::default();
        let mut entry = E820Entry::default();

        regs.ebx = 0;
        loop {
            // Set up the arguments for E820, we use the previous
            // continuation code
            regs.eax = 0xe820;
            regs.edi = &mut entry as *mut E820Entry as u32;
            regs.ecx = core::mem::size_of_val(&entry) as u32;
            regs.edx = u32::from_be_bytes(*b"SMAP");

            // Invoke the BIOS for the E820 memory map
            invoke_realmode(0x15, &mut regs);

            if (regs.efl & 1) != 0 {
                // Check the CF for an error
                panic!("Error on E820");
            }
            print!("{:x?}\n",entry);

            if regs.ebx ==0{
                break;
            }
        }
    }
    cpu::halt();
}

