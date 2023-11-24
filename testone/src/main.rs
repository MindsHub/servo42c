use std::time::Duration;

use motor::servo42::{standard::Servo42C, Servo42CTrait};
use serial::standard::{serialport, Parity, DataBits};

fn main() {
    let s = serialport::new("/dev/ttyACM1", 115200)
        .timeout(Duration::from_millis(10))
        .parity(Parity::None)
        .stop_bits(serial::standard::StopBits::One)
        .data_bits(DataBits::Eight)
        .flow_control(serial::standard::FlowControl::None)
        .open().unwrap();
    let mut z = Servo42C::new(s).unwrap();
    println!("wtf");
    //z.goto()
    z.read_lock();
    z.release_lock();
    //let _ = z.set_microstep(16);
    let _ =z.goto(10, 200*16*100);
    //println!("Hello, world!");
}

/*
[0xe0][0x3e][0x1e]
E03E1E
E03D1D
[0xe0][0xfd][0xa][0x0][0x4][0xe2][0x0][0xcd]
E0FD0A0004E200CD */