use crate::motortrait::{Motor, MotorBuilder};
use ::serial::serialtrait::{Sendable, Serial, SerialError};
use core::fmt::Debug;
use serial::serialtrait::MySize;
pub mod linear_acc;
pub mod test;
//pub mod serial;
pub mod standard;

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

pub trait Servo42CTrait<T> where Self: Sized,
    T: Serial{
    fn empty_new(t: T)->Self;
    fn new(s: T) -> Result<Self, MotorError>;

    fn send<Data: Sendable>(&mut self, code: u8, data: Data) -> Result<(), MotorError>
    where
        [(); <((u8, u8), (Data, u8))>::SIZE]:,
        [(); Data::SIZE]:,
        [(); <(Data, u8)>::SIZE]:;

    fn read<Res: Sendable>(&mut self) -> Result<Res, MotorError>
    where
        [(); Res::SIZE]:,
        [(); <(u8, Res)>::SIZE]:,
        [(); <((u8, Res), u8)>::SIZE]:;
    fn send_cmd<Data: Sendable, Res: Sendable>(
        &mut self,
        code: u8,
        data: Data,
    ) -> Result<Res, MotorError>
    where
        [(); Data::SIZE]:,
        [(); <(Data, u8)>::SIZE]:,
        [(); <((u8, u8), (Data, u8))>::SIZE]:,

        [(); Res::SIZE]:,
        [(); <(u8, Res)>::SIZE]:,
        [(); <((u8, Res), u8)>::SIZE]:;

    fn read_encoder_value(&mut self) -> Result<f64, MotorError>;
    fn read_recived_pulses(&mut self) -> Result<f64, MotorError>;
    fn read_error(&mut self) -> Result<f64, MotorError>;
    fn read_en_pin(&mut self) -> Result<bool, MotorError>;
    fn release_lock(&mut self) -> Result<(), MotorError>;
    fn read_lock(&mut self) -> Result<Protection, MotorError>;

    fn calibrate(&mut self) -> Result<(), MotorError>;
    fn set_mot_type(&mut self, mot_type: MotType) -> Result<(), MotorError>;
    fn set_mode(&mut self, work_mode: WorkMode) -> Result<(), MotorError>;
    fn set_current(&mut self, t: u8) -> Result<(), MotorError>;
    fn set_microstep(&mut self, mstep: u8) -> Result<(), MotorError>;
    fn set_en_active(&mut self, active_on: ActiveOn) -> Result<(), MotorError>;
    fn set_direction(&mut self, dir: Dir) -> Result<(), MotorError>;
    fn set_autossd(&mut self, active: bool) -> Result<(), MotorError>;
    fn set_lock(&mut self, protection: Protection) -> Result<(), MotorError>;
    fn set_subdivision_interpolation(&mut self, active: bool) -> Result<(), MotorError>;
    fn set_baudrate(&mut self, baud_rate: BaudRate) -> Result<(), MotorError>;
    fn set_slave_address(&mut self, addr: u8) -> Result<(), MotorError>;
    fn reset(&mut self) -> Result<(), MotorError>;

    fn set_zero_mode(&mut self, mode: u8) -> Result<(), MotorError>;
    fn set_zero(&mut self) -> Result<(), MotorError>;
    fn set_zero_speed(&mut self, speed: u8) -> Result<(), MotorError>;
    fn set_zero_dir(&mut self, dir: u8) -> Result<(), MotorError>;
    fn goto_zero(&mut self) -> Result<(), MotorError>;

    fn set_kp(&mut self, kp: u16) -> Result<(), MotorError>;
    fn set_ki(&mut self, ki: u16) -> Result<(), MotorError>;
    fn set_kd(&mut self, kd: u16) -> Result<(), MotorError>;
    fn set_acc(&mut self, acc: u16) -> Result<(), MotorError>;
    fn set_maxt(&mut self, kp: Option<u16>) -> Result<(), MotorError>;
    /**
    Set the En pin status in CR_UART mode.
    */
    fn set_enable(&mut self, en: bool) -> Result<(), MotorError>;
    fn set_speed(&mut self, speed: i8) -> Result<u8, MotorError>;
    fn stop(&mut self) -> Result<u8, MotorError>;
    fn goto(&mut self, speed: u8, dist: u32) -> u8;
}

#[derive(PartialEq, Eq, Debug)]
pub enum Protection {
    Protected,
    UnProtected,
}
pub enum MotType {
    Deg1_8,
    Deg0_9,
}

pub enum WorkMode {
    CrOpen,
    CrVFoc,
    CrUART,
}

pub enum ActiveOn {
    Low,
    High,
    Hold,
}
pub enum Dir {
    ClockWise,
    CounterClockWise,
}
#[derive(Debug, PartialEq, Clone)]
pub enum BaudRate {
    B9600,
    B19200,
    B25000,
    B38400,
    B57600,
    B115200,
}
impl From<BaudRate> for u32 {
    fn from(value: BaudRate) -> Self {
        match value {
            BaudRate::B9600 => 9600,
            BaudRate::B19200 => 19200,
            BaudRate::B25000 => 25000,
            BaudRate::B38400 => 38400,
            BaudRate::B57600 => 57600,
            BaudRate::B115200 => 115200,
        }
    }
}
