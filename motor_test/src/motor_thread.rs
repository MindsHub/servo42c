use std::{
    sync::mpsc::{channel, Receiver, Sender},
    thread,
    time::{Duration, SystemTime},
};

use motor::{
    motortrait::{Motor, MotorBuilder, UpdateStatus},
    servo42::{linear_acc::{Servo42LinearAccBuilder, Servo42LinearAcc}, Servo42CTrait, test::Servo42CTest},
};
use serial::standard::{serialport, DataBits, Parity, SerialPort};
pub struct MotorState {
    pub pos: f64,
    pub obbiettivo: f64,
    pub error: f64,
    pub timing: Duration,
    pub cmd_rate: f64,
    pub reached: bool,
}

pub enum MotorComand {
    KillThread,
}

pub fn new_thread(
    name: &str,
    baudrate: u32,
    builder: &Servo42LinearAccBuilder<Box<dyn SerialPort>>,
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
    let mut cur_builder = Servo42LinearAccBuilder::new(s);
    cur_builder.max_speed = builder.max_speed;
    cur_builder.acc = builder.acc;
    //builder.s=s;
    let mut m: Servo42LinearAcc<Box<dyn SerialPort>, Servo42CTest<Box<dyn SerialPort>>> = cur_builder.build().map_err(|_| "Impossibile comunicare!")?;
    thread::spawn(move || {
        let mut time = SystemTime::now();
        let mut update_obj_timer = SystemTime::now() - Duration::from_secs(100);
        let mut state = false;
        let start = SystemTime::now();
        let mut cmd_sent = 0.;
        let mut elapsed;
        loop {
            thread::sleep(Duration::from_micros(930));
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
            if update_obj_timer.elapsed().unwrap() > Duration::from_secs(10) {
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
            let z = m.update(elapsed).unwrap();
            println!("{}", elapsed.as_micros());
            cmd_sent += 3.;

            //let error = (m.pos-m.obbiettivo)*360.;
            let error = m.m.read_error().unwrap() as f64;
            /*println!(
                "{}            {error}",
                (error + m.obbiettivo - m.pos) * 360.
            );*/
            //let error=m.m.read_encoder_value().unwrap()-zero;
            //println!("{error}");
            //send data
            let to_send = MotorState {
                pos: m.pos,
                obbiettivo: m.obbiettivo,
                timing: start.elapsed().unwrap(),
                cmd_rate: cmd_sent / start.elapsed().unwrap().as_secs_f64(),
                error: error * 200.,
                reached: z == UpdateStatus::GetThere,
            };
            
            data_sender.send(to_send).unwrap(); //if can't sent it's ok to crash
        }
    });

    Ok((data_receiver, cmd_sender))
}

pub struct EmptySerial {}
impl std::io::Write for EmptySerial {
    fn write(&mut self, _buf: &[u8]) -> std::io::Result<usize> {
        todo!()
    }

    fn flush(&mut self) -> std::io::Result<()> {
        todo!()
    }
}
impl std::io::Read for EmptySerial {
    fn read(&mut self, _buf: &mut [u8]) -> std::io::Result<usize> {
        todo!()
    }
}
impl SerialPort for EmptySerial {
    fn name(&self) -> Option<String> {
        todo!()
    }

    fn baud_rate(&self) -> serial::standard::Result<u32> {
        todo!()
    }

    fn data_bits(&self) -> serial::standard::Result<DataBits> {
        todo!()
    }

    fn flow_control(&self) -> serial::standard::Result<serial::standard::FlowControl> {
        todo!()
    }

    fn parity(&self) -> serial::standard::Result<Parity> {
        todo!()
    }

    fn stop_bits(&self) -> serial::standard::Result<serial::standard::StopBits> {
        todo!()
    }

    fn timeout(&self) -> Duration {
        todo!()
    }

    fn set_baud_rate(&mut self, _baud_rate: u32) -> serial::standard::Result<()> {
        todo!()
    }

    fn set_data_bits(&mut self, _data_bits: DataBits) -> serial::standard::Result<()> {
        todo!()
    }

    fn set_flow_control(
        &mut self,
        _flow_control: serial::standard::FlowControl,
    ) -> serial::standard::Result<()> {
        todo!()
    }

    fn set_parity(&mut self, _parity: Parity) -> serial::standard::Result<()> {
        todo!()
    }

    fn set_stop_bits(
        &mut self,
        _stop_bits: serial::standard::StopBits,
    ) -> serial::standard::Result<()> {
        todo!()
    }

    fn set_timeout(&mut self, _timeout: Duration) -> serial::standard::Result<()> {
        todo!()
    }

    fn write_request_to_send(&mut self, _level: bool) -> serial::standard::Result<()> {
        todo!()
    }

    fn write_data_terminal_ready(&mut self, _level: bool) -> serial::standard::Result<()> {
        todo!()
    }

    fn read_clear_to_send(&mut self) -> serial::standard::Result<bool> {
        todo!()
    }

    fn read_data_set_ready(&mut self) -> serial::standard::Result<bool> {
        todo!()
    }

    fn read_ring_indicator(&mut self) -> serial::standard::Result<bool> {
        todo!()
    }

    fn read_carrier_detect(&mut self) -> serial::standard::Result<bool> {
        todo!()
    }

    fn bytes_to_read(&self) -> serial::standard::Result<u32> {
        todo!()
    }

    fn bytes_to_write(&self) -> serial::standard::Result<u32> {
        todo!()
    }

    fn clear(
        &self,
        _buffer_to_clear: serial::standard::ClearBuffer,
    ) -> serial::standard::Result<()> {
        todo!()
    }

    fn try_clone(&self) -> serial::standard::Result<Box<dyn SerialPort>> {
        todo!()
    }

    fn set_break(&self) -> serial::standard::Result<()> {
        todo!()
    }

    fn clear_break(&self) -> serial::standard::Result<()> {
        todo!()
    }
}
