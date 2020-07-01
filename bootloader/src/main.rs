#![feature(rustc_private, asm)]
#![no_std]
#![no_main]

mod core_reqs;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// TODO: what is inline
// TODO: how to use asm!
#[inline]
unsafe fn out8(addr: u16, val: u8) {
    asm!("out dx,al" :: "{dx}"(addr), "{al}"(val):: "volatile","intel");
}

#[inline]
unsafe fn in8(addr: u16) -> u8 {
    let val: u8;
    asm!("in al,dx" : "={al}"(val): "{dx}"(addr):: "volatile","intel");
    val
}


fn serial_init() {
    unsafe {
        for com_id in 0..4 {
            // Get the COM port I/O address from the BIOS data area (BDA)
            let port = unsafe { *(0x400 as *const u16).offset(com_id) };
            // If the port address is zero, it is not present as reported by
            // the BIOS
            if port == 0 {
                continue;
            }
            unsafe {
                out8(port + 1, 0x00); // Disable all interrupt
                out8(port + 3, 0x80); // Enable DLAB
                out8(port + 0, 0x01); // Low byte divisor (115200 baud)
                out8(port + 1, 0x00); // High byte divisor
                out8(port + 3, 0x03); // 8n1, no parity
                out8(port + 4, 0x03); // RTS/DSR set
                loop {
                    while (in8(port + 5) & 0x20) == 0 {}
                    out8(port, b'A');
                }
            }
        }
    }
}

#[no_mangle]
extern fn entry() {
    serial_init();
    unsafe {
        loop {
            asm!(r#"
            cli
            hlt
        "# :::: "volatile", "intel");
        }
    }
}

