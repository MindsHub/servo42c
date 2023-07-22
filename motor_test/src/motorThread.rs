use std::{sync::mpsc::{channel, Receiver}, thread, time::Duration};

use motor::servo42::linear_acc::Servo42LinearAccBuilder;
use serial::standard::{SerialPort, serialport, Parity, DataBits};

pub struct MotorState{

}
pub fn new_thread(name: &str)->Receiver<MotorState>{
    let (sender, receiver)=channel::<MotorState>();
    let name= name.to_string();
    thread::spawn(move || {
        let s = serialport::new(name, 38_400)
            .timeout(Duration::from_millis(10))
            .parity(Parity::None)
            //.baud_rate(115200)
            .stop_bits(serial::standard::StopBits::One)
            .data_bits(DataBits::Eight)
            .flow_control(serial::standard::FlowControl::None)
            .open().unwrap();
        Servo42LinearAccBuilder::new(s)
    });



    receiver
}