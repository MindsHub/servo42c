#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use arduino_hal::{Usart, hal::{Atmega, port::{PD0, PD1}}, port::{mode::{Input, Floating, Output, AnyInput}, Pin}, simple_pwm::{Prescaler, Timer2Pwm}};
use avr_device::atmega328p::{USART0, self};
use embedded_hal::{serial::Write, timer};
use embedded_hal::blocking::delay::DelayUs;
use millis::millis_init;
use motor::prelude::*;
use panic_halt as _;
pub mod millis;
pub mod serial_timer;
type Serial=arduino_hal::Usart<atmega328p::USART0, Pin<Input<AnyInput>, PD0>, Pin<Output, PD1>>;
#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    
    //let z = pins.d1.into_output();
    let serial:  Serial = arduino_hal::default_serial!(dp, pins, 57600);

    //let t = bitbang_hal::serial::Serial::new(pins.d2.into_output(), pins.d3, dp.TC2);
    arduino_hal::delay_us(10);
    /*let w = arduino_hal::Usart::new(
        dp,
        pins.d0,
        pins.d1.into_output(),
        arduino_hal::hal::usart::BaudrateArduinoExt::into_baudrate(57600),
    );*/
    //let mut z  =bitbang_hal::serial::Serial::new(pins.d10., pins.d11, dp.TC1.into());
    //z.write(b'x');
    millis_init(dp.TC0);
    let m: Servo42LinearAcc<Serial, Servo42C<Serial>>=Servo42LinearAccBuilder::new(serial).build().unwrap();
    // Wait for a character and print current time once it is received
    loop {
        //m.update(time_from_last);
    }
}