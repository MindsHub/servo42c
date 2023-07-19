
use ::serial::serialtrait::{Serial, SerialError};
use core::fmt::Debug;

use crate::motortrait::{Motor, MotorBuilder};
pub mod linear_acc;
pub mod serial;
pub struct Servo42C<T: Serial> {
    pub address: u8,
    pub s: T,
    pub kp: u16,
    pub ki: u16,
    pub kd: u16,
    pub acc: u16,
}

impl<T: Serial> Servo42C<T> {
    pub fn new(s: T) -> Result<Servo42C<T>, SerialError> {
        let t = Servo42C::<T> {
            address: 0xe0,
            s,
            kp: 1616,
            ki: 288,
            kd: 1616,
            acc: 286,
        };
        Ok(t)
    }
}

#[derive(Debug)]
pub enum MotorError {
    SerialError,
    Stuck,
}




/*
impl<T: Serial> Motor<i64> for Servo42C<T>{
    fn goto(&mut self, pos: i64, ) -> Result<(), ()> {
        dt = obj - PrevPos;
        if dt < 0 {
            self.set_speed(false, speeed);
        }else {
            self.set_speed(true, speeed);
        }
        todo!()
    }

    fn get_info(&mut self) {
        todo!()
    }

    fn update(&mut self, time_from_last: Duration) {
        todo!()
    }
}*/
