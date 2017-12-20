#![no_std]
#![no_main]
#![feature(use_extern_macros)]

#[macro_use(print)]
extern crate teensy3;

use teensy3::bindings;
use teensy3::serial::Serial;

#[no_mangle]
pub unsafe extern fn main() {
    // Blink Loop
    bindings::pinMode(13, bindings::OUTPUT as u8);
    bindings::digitalWrite(13, bindings::LOW as u8);
    let ser = Serial{};
    let mut count: i32 = 0;

    loop {
        // Show we are alive
        alive();

        // If the serial write fails, we will halt (no more alive blinks)
        // send_waiting(count).unwrap();

        match ser.try_read_byte() {
            Some(in_byte) => send_echo(in_byte),
            None => Ok(())
        }.unwrap();

        count += 1;
        // Don't spam the console
        bindings::delay(100);
    }
}

/// Blink the light twice to know we're alive
pub unsafe fn alive() {
    for _ in 0..2 {
        bindings::digitalWrite(13, bindings::LOW as u8);
        bindings::delay(100);
        bindings::digitalWrite(13, bindings::HIGH as u8);
        bindings::delay(100);
        bindings::digitalWrite(13, bindings::LOW as u8);
        bindings::delay(100);
    }
}

/// Send a message over the USB Serial port
pub fn send_waiting(count: i32) -> Result<(),()> {
    print!("Waiting {}\n\r", count);
    Ok(())
}

pub fn send_echo(msg : u8) -> Result<(),()> {
    print!("Received: \"{}\"\n\r", msg as char);
    Ok(())
}
