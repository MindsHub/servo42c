use core::marker::PhantomData;

use serial::serialtrait::Serial;

use super::{Servo42CTrait, MotorError};
use serial::serialtrait::MySize;

struct Servo42CTest<S: Serial> {
    microstep: u8,
    kp: u16,
    ki: u16,
    kd: u16,
    acc: u16,
    ph: PhantomData<S>,
}

impl<T: Serial> Servo42CTrait<T> for Servo42CTest<T>{
    fn empty_new(_s: T) -> Self {
        Servo42CTest{
            kp: 1616, //1616,
            ki: 288,  //288,
            kd: 1616, //1616,
            acc: 286, //286,
            microstep: 16,
            ph: PhantomData,
        }
    }

    fn new(s: T) -> Result<Self, MotorError> {
        let mut t = Servo42CTest::empty_new(s);
        t.stop()?;
        t.set_kp(t.kp)?;
        t.set_ki(t.ki)?;
        t.set_kd(t.kd)?;
        t.set_acc(t.acc)?;
        t.set_microstep(t.microstep)?;
        t.set_maxt(Some(2000))?;
        Ok(t)
    }

    fn send<Data: serial::serialtrait::Sendable>(&mut self, code: u8, data: Data) -> Result<(), super::MotorError>
    where
        [(); <((u8, u8), (Data, u8))>::SIZE]:,
        [(); Data::SIZE]:,
        [(); <(Data, u8)>::SIZE]: {
        todo!()
    }

    fn read<Res: serial::serialtrait::Sendable>(&mut self) -> Result<Res, super::MotorError>
    where
        [(); Res::SIZE]:,
        [(); <(u8, Res)>::SIZE]:,
        [(); <((u8, Res), u8)>::SIZE]: {
            self.stop();
        todo!()
    }

    fn send_cmd<Data: serial::serialtrait::Sendable, Res: serial::serialtrait::Sendable>(
        &mut self,
        code: u8,
        data: Data,
    ) -> Result<Res, super::MotorError>
    where
        [(); Data::SIZE]:,
        [(); <(Data, u8)>::SIZE]:,
        [(); <((u8, u8), (Data, u8))>::SIZE]:,

        [(); Res::SIZE]:,
        [(); <(u8, Res)>::SIZE]:,
        [(); <((u8, Res), u8)>::SIZE]: {
        todo!()
    }

    fn read_encoder_value(&mut self) -> Result<f64, super::MotorError> {
        todo!()
    }

    fn read_recived_pulses(&mut self) -> Result<f64, super::MotorError> {
        todo!()
    }

    fn read_error(&mut self) -> Result<f64, super::MotorError> {
        todo!()
    }

    fn read_en_pin(&mut self) -> Result<bool, super::MotorError> {
        todo!()
    }

    fn release_lock(&mut self) -> Result<(), super::MotorError> {
        todo!()
    }

    fn read_lock(&mut self) -> Result<super::Protection, super::MotorError> {
        todo!()
    }

    fn calibrate(&mut self) -> Result<(), super::MotorError> {
        todo!()
    }

    fn set_mot_type(&mut self, mot_type: super::MotType) -> Result<(), super::MotorError> {
        todo!()
    }

    fn set_mode(&mut self, work_mode: super::WorkMode) -> Result<(), super::MotorError> {
        todo!()
    }

    fn set_current(&mut self, t: u8) -> Result<(), super::MotorError> {
        todo!()
    }

    fn set_microstep(&mut self, mstep: u8) -> Result<(), super::MotorError> {
        todo!()
    }

    fn set_en_active(&mut self, active_on: super::ActiveOn) -> Result<(), super::MotorError> {
        todo!()
    }

    fn set_direction(&mut self, dir: super::Dir) -> Result<(), super::MotorError> {
        todo!()
    }

    fn set_autossd(&mut self, active: bool) -> Result<(), super::MotorError> {
        todo!()
    }

    fn set_lock(&mut self, protection: super::Protection) -> Result<(), super::MotorError> {
        todo!()
    }

    fn set_subdivision_interpolation(&mut self, active: bool) -> Result<(), super::MotorError> {
        todo!()
    }

    fn set_baudrate(&mut self, baud_rate: super::BaudRate) -> Result<(), super::MotorError> {
        todo!()
    }

    fn set_slave_address(&mut self, addr: u8) -> Result<(), super::MotorError> {
        todo!()
    }

    fn reset(&mut self) -> Result<(), super::MotorError> {
        todo!()
    }

    fn set_zero_mode(&mut self, mode: u8) -> Result<(), super::MotorError> {
        todo!()
    }

    fn set_zero(&mut self) -> Result<(), super::MotorError> {
        todo!()
    }

    fn set_zero_speed(&mut self, speed: u8) -> Result<(), super::MotorError> {
        todo!()
    }

    fn set_zero_dir(&mut self, dir: u8) -> Result<(), super::MotorError> {
        todo!()
    }

    fn goto_zero(&mut self) -> Result<(), super::MotorError> {
        todo!()
    }

    fn set_kp(&mut self, kp: u16) -> Result<(), super::MotorError> {
        todo!()
    }

    fn set_ki(&mut self, ki: u16) -> Result<(), super::MotorError> {
        todo!()
    }

    fn set_kd(&mut self, kd: u16) -> Result<(), super::MotorError> {
        todo!()
    }

    fn set_acc(&mut self, acc: u16) -> Result<(), super::MotorError> {
        todo!()
    }

    fn set_maxt(&mut self, kp: Option<u16>) -> Result<(), super::MotorError> {
        todo!()
    }

    fn set_enable(&mut self, en: bool) -> Result<(), MotorError> {
        todo!()
    }

    fn set_speed(&mut self, speed: i8) -> Result<u8, MotorError> {
        todo!()
    }

    fn stop(&mut self) -> Result<u8, MotorError> {
        todo!()
    }

    fn goto(&mut self, speed: u8, dist: u32) -> u8 {
        todo!()
    }
}