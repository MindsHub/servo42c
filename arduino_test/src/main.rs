#![no_std]
#![no_main]
use panic_halt as _;
use motor::servo42::{self, Servo42C};
#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let serial = arduino_hal::default_serial!(dp, pins, 57600);
    let mut t: Servo42C<_> = servo42::Servo42C::new(serial).unwrap();
    // Wait for a character and print current time once it is received
    loop {
        //t.calibrate();
        let _ =t.read_encoder_value();
        //let s=[0, 0, 0];
        //let _ =serial.write(s.as_slice());
        //t.se
        //serial.write(&mut send);
        //serial.read(&mut buff);
        //serial.write(&mut buff);
        //serial.write(b'w');
        //let _ = t.read_encoder_value();
        //while z.is_ok(){}
        //arduino_hal::delay_ms(1000);
    }
}