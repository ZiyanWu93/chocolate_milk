//! x86 CPU routines

#![feature(asm)]
#![no_std]

/// Output an 8-bit `val` to I/O port `addr`
#[inline]
pub unsafe fn out8(addr: u16, val: u8) {
    asm!("out dx, al" :: "{dx}"(addr), "{al}"(val) :: "volatile", "intel");
}

/// Read an 8-bit value from I/O port `addr`
#[inline]
pub unsafe fn in8(addr: u16) -> u8 {
    let val: u8;
    asm!("in al, dx" : "={al}"(val) : "{dx}"(addr) :: "volatile", "intel");
    val
}

/// Disabel interrupts and halt forever
pub fn halt() ->!{
    unsafe {
        loop {
        asm ! (r#"
            cli
            hlt
        "#::::"volatile", "intel");
        }
    }
}
