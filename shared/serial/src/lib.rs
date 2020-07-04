//! A basic 8250A serial driver for x86

#![no_std]

use lockcell::LockCell;

/// A collection of 4 8250A serial ports, as seen on IBM PC systems. These are
/// the 4 serial ports which are identified by the BIOS, and thus it is limited
/// to just COM1-COM4.
struct SerialPort {
    devices: [Option<u16>; 4],
}

/// Global state for the serial ports on the system
static SERIAL: LockCell<SerialPort> = LockCell::new(SerialPort {
    devices: [None; 4],
});


pub fn init() {
// Go through each possible COM port
    let mut serial= SERIAL.lock();
    for (com_id, device) in serial.devices.iter_mut().enumerate() {
        // Get the COM port I/O address from the BIOS data area (BDA)
        let port =
            unsafe { *(0x400 as *const u16).offset(com_id as isize) };

        // If the port address is zero, it is not present as reported by
        // the BIOS
        if port == 0 {
            // Serial port is not present
            *device = None;
            continue;
        }

        // Initialize the serial port to a known state
        unsafe {
            cpu::out8(port + 1, 0x00); // Disable all interrupts
            cpu::out8(port + 3, 0x80); // Enable DLAB
            cpu::out8(port + 0, 0x01); // Low byte divisor (115200 baud)
            cpu::out8(port + 1, 0x00); // High byte divisor
            cpu::out8(port + 3, 0x03); // 8 bits, 1 stop bit, no parity
            cpu::out8(port + 4, 0x03); // RTS/DSR set
        }


        // Save that we found and initialized a serial port
        *device = Some(port);
    }
}

pub fn write(val:u8){
    // Get access to the serial ports
    let serial= SERIAL.lock();

    for device in &serial.devices{
        if let Some(port)= device{
            unsafe {
                cpu::out8(*port, val);
            }
        }
    }
}

pub fn write_bytes(val:u8){
    // Get access to the serial ports
    let serial= SERIAL.lock();

    for device in &serial.devices{
        if let Some(port)= device{
            unsafe {
                cpu::out8(*port, val);
            }
        }
    }
}
