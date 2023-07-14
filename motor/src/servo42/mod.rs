use core::time::Duration;

use ::serial::serialtrait::{Serial, SerialError};
use core::fmt::Debug;

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

pub trait Motor
where
    Self: Sized,
{
    type PosUnit;
    type Info: Debug;
    type Builder: MotorBuilder<Self>;
    ///set a new objective
    fn goto(&mut self, pos: Self::PosUnit) -> Result<(), ()>;
    ///get printable info
    fn get_info(&mut self) -> Self::Info;
    ///function to call for an update
    fn update(&mut self, time_from_last: Duration) -> Result<(), MotorError>;
    ///find zero, and set
    fn reset(&mut self);
    ///set zero here
    fn set_zero(&mut self);
    ///Generic Function for set max speed, acceleration...
    fn new(&mut self) -> Self::Builder;
}
pub trait MotorBuilder<T: Motor>
where
    Self: Sized,
{
    fn build() -> T;
}

pub struct Servo42LinearAcc<T: Serial> {
    m: Servo42C<T>,
}
pub struct Servo42LinearAccBuilder<T: Serial> {
    s: T,
}

impl<T: Serial> Motor for Servo42LinearAcc<T> {
    fn goto(&mut self, pos: i64) -> Result<(), ()> {
        todo!()
    }

    fn update(&mut self, time_from_last: Duration) -> Result<(), MotorError> {
        todo!()
    }

    fn reset(&mut self) {
        todo!()
    }

    fn set_zero(&mut self) {
        todo!()
    }

    type PosUnit = i64;

    type Info = MotorError;

    type Builder = Servo42LinearAccBuilder<T>;

    fn new(&mut self) -> Self::Builder {
        todo!()
    }

    fn get_info(&mut self) -> Self::Info {
        todo!()
    }
}

impl<T: Serial> MotorBuilder<Servo42LinearAcc<T>> for Servo42LinearAccBuilder<T> {
    fn build() -> Servo42LinearAcc<T> {
        todo!()
    }
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
