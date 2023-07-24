use std::{sync::mpsc::{channel, Receiver, Sender}, thread, time::{Duration, SystemTime}, io::Error};

use motor::{servo42::linear_acc::Servo42LinearAccBuilder, motortrait::{MotorBuilder, Motor}};
use serial::standard::{serialport, Parity, DataBits};
pub struct MotorState{
    pub pos: f64,
    pub obbiettivo: f64,
    pub timing: Duration,
}

pub enum MotorComand{
    KillThread,
}

pub fn new_thread(name: &str, baudrate: u32)->Result<(Receiver<MotorState>, Sender<MotorComand>), Box<dyn std::error::Error>>{
    let (data_sender, data_receiver)=channel::<MotorState>();
    let (cmd_sender, cmd_receiver)=channel::<MotorComand>();
    let s = serialport::new(name, baudrate)
        .timeout(Duration::from_millis(10))
        .parity(Parity::None)
        .stop_bits(serial::standard::StopBits::One)
        .data_bits(DataBits::Eight)
        .flow_control(serial::standard::FlowControl::None)
        .open()?;
    let mut m = Servo42LinearAccBuilder::new(s).build().map_err(|_| {
        "Impossibile comunicare!"
    })?;
    thread::spawn(move || {
        let mut time=SystemTime::now();
        let mut update_obj_timer=SystemTime::now()-Duration::from_secs(100);
        let mut state=false;
        let start=SystemTime::now();
        loop{
            //if received a valid comand
            if let Ok(cmd) = cmd_receiver.try_recv(){
                match cmd{
                    MotorComand::KillThread => {
                        let _ = m.m.release_lock();
                        break},
                }
            }

            //change obj
            if update_obj_timer.elapsed().unwrap()>Duration::from_secs(10){
                update_obj_timer= SystemTime::now();
                state= !state;
                if state {
                    let _ = m.goto(1.);
                }else{
                    let _ = m.goto(0.);
                }
                
            }

            //update
            let _z = m.update(time.elapsed().unwrap());



            //send data
            let to_send=MotorState{
                pos: m.pos,
                obbiettivo: m.obbiettivo,
                timing: start.elapsed().unwrap(),
            };
            data_sender.send(to_send).unwrap();//if can't sent it's ok to crash

            //set variable
            time=SystemTime::now();
        }
    });


    Ok((data_receiver, cmd_sender))
}