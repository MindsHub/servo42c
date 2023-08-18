#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use arduino_hal::{Usart, hal::Atmega};
use avr_device::atmega328p::USART0;
use embedded_hal::serial::Write;
use millis::millis_init;
use motor::{servo42::linear_acc::{Servo42LinearAccBuilder, Servo42LinearAcc}, motortrait::{MotorBuilder, Motor}};
use panic_halt as _;
mod millis;
#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut tmr = dp.TC1;

    let serial = arduino_hal::default_serial!(dp, pins, 57600);
    //let mut z  =bitbang_hal::serial::Serial::new(pins.d10., pins.d11, dp.TC1.into());
    //z.write(b'x');
    millis_init(dp.TC0);
    //let m=Servo42LinearAccBuilder::new(serial).build().unwrap();
    // Wait for a character and print current time once it is received
    loop {
        //m.update(time_from_last);
    }
}