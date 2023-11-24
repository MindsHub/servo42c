use crate::servo42::MotorError;
use core::fmt::Debug;
use core::time::Duration;

#[derive(PartialEq, Eq)]
pub enum UpdateStatus {
    Working,
    GetThere,
}
pub trait Motor
where
    Self: Sized,
{
    type PosUnit;
    type Info: Debug;
    type Builder: MotorBuilder<Self>;
    ///set a new objective
    fn goto(&mut self, pos: Self::PosUnit) -> Result<(), MotorError>;
    ///get printable info
    fn get_info(&mut self) -> Self::Info;
    ///function to call for an update
    fn update(&mut self, time_from_last: Duration) -> Result<UpdateStatus, MotorError>;
    ///find zero, and set
    fn reset(&mut self);
    ///set zero here
    fn set_zero(&mut self);
    //Generic Function for set max speed, acceleration...
    //fn new() -> Self::Builder;
}

pub trait MotorBuilder<M: Motor> {
    fn build(self) -> Result<M, MotorError>;
}
