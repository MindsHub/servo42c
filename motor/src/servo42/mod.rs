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
    pub microstep: u8,
}

impl<T: Serial> Servo42C<T> {
    pub fn empty_new(s: T) -> Servo42C<T> {
        Servo42C::<T> {
            address: 0xe0,
            s,
            kp: 1616,
            ki: 288,
            kd: 1616,
            acc: 286,
            microstep: 16,
        }
    }

    pub fn new(s: T) -> Result<Servo42C<T>, MotorError> {
        let mut t = Servo42C::empty_new(s);
        t.stop()?;
        t.set_kp(t.kp)?;
        t.set_ki(t.ki)?;
        t.set_kd(t.kd)?;
        t.set_acc(t.acc)?;
        t.set_microstep(t.microstep)?;
        Ok(t)
    }
}

#[derive(Debug)]
pub enum MotorError {
    SerialError(SerialError),
    Stuck,
    NegativeResponse,
}

impl From<SerialError> for MotorError {
    fn from(value: SerialError) -> Self {
        MotorError::SerialError(value)
    }
}
