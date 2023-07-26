use std::{
    sync::mpsc::{channel, Receiver, Sender},
    thread,
    time::{Duration, SystemTime},
};

use motor::{
    motortrait::{Motor, MotorBuilder},
    servo42::linear_acc::Servo42LinearAccBuilder,
};
use serial::standard::{serialport, DataBits, Parity};
pub struct MotorState {
    pub pos: f64,
    pub obbiettivo: f64,
    pub error: f64,
    pub timing: Duration,
    pub cmd_rate: f64,
}

pub enum MotorComand {
    KillThread,
}

pub fn new_thread(
    name: &str,
    baudrate: u32,
) -> Result<(Receiver<MotorState>, Sender<MotorComand>), Box<dyn std::error::Error>> {
    let (data_sender, data_receiver) = channel::<MotorState>();
    let (cmd_sender, cmd_receiver) = channel::<MotorComand>();
    let s = serialport::new(name, baudrate)
        .timeout(Duration::from_millis(10))
        .parity(Parity::None)
        .stop_bits(serial::standard::StopBits::One)
        .data_bits(DataBits::Eight)
        .flow_control(serial::standard::FlowControl::None)
        .open()?;
    let mut m = Servo42LinearAccBuilder::new(s)
        .build()
        .map_err(|_| "Impossibile comunicare!")?;
    thread::spawn(move || {
        let mut time = SystemTime::now();
        let mut update_obj_timer = SystemTime::now() - Duration::from_secs(100);
        let mut state = false;
        let start = SystemTime::now();
        let mut cmd_sent = 0.;
        let mut elapsed;
        loop {
            //if received a valid comand
            if let Ok(cmd) = cmd_receiver.try_recv() {
                match cmd {
                    MotorComand::KillThread => {
                        let _ = m.m.release_lock();
                        break;
                    }
                }
            }

            //change obj
            if update_obj_timer.elapsed().unwrap() > Duration::from_secs(30) {
                update_obj_timer = SystemTime::now();
                state = !state;
                if state {
                    let _ = m.goto(60.);
                } else {
                    let _ = m.goto(0.);
                }
            }

            //set variable
            elapsed = time.elapsed().unwrap();
            time = SystemTime::now();

            //update
            let _z = m.update(elapsed);
            //println!("{} {}", m.cur_speed, elapsed.as_secs_f64()*m.acc);
            cmd_sent += 3.;

            let error = m.m.read_error().unwrap() as f64 / 65536.;

            //send data
            let to_send = MotorState {
                pos: m.pos,
                obbiettivo: m.obbiettivo,
                timing: start.elapsed().unwrap(),
                cmd_rate: cmd_sent / start.elapsed().unwrap().as_secs_f64(),
                error,
            };
            data_sender.send(to_send).unwrap(); //if can't sent it's ok to crash
        }
    });

    Ok((data_receiver, cmd_sender))
}
