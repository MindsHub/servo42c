use core::time::Duration;

use serial::serialtrait::Serial;

use super::{Motor, MotorBuilder};

use super::{Servo42C, MotorError};

pub struct Servo42LinearAcc<T: Serial> {
    _m: Servo42C<T>,
}
pub struct Servo42LinearAccBuilder<T: Serial> {
    _s: T,
}

impl<T: Serial> Motor for Servo42LinearAcc<T> {
    type PosUnit = i64;
    type Info = MotorError;
    type Builder = Servo42LinearAccBuilder<T>;

    fn goto(&mut self, _pos: Self::PosUnit) -> Result<(), ()> {
        todo!()
    }

    fn update(&mut self, _time_from_last: Duration) -> Result<(), MotorError> {
        todo!()
    }

    fn reset(&mut self) {
        todo!()
    }

    fn set_zero(&mut self) {
        todo!()
    }

    fn get_info(&mut self) -> Self::Info {
        todo!()
    }

}

impl<T: Serial> MotorBuilder for Servo42LinearAccBuilder<T> {
    fn build(self) -> Servo42LinearAcc<T> {
        todo!()
    }

    type M=Servo42LinearAcc<T>;
}