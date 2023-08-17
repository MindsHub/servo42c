use core::marker::PhantomData;

use serial::serialtrait::Serial;

use super::{Servo42CTrait, MotorError};
use serial::serialtrait::MySize;

pub struct Servo42CTest<S: Serial> {
    microstep: u8,
    kp: u16,
    ki: u16,
    kd: u16,
    acc: u16,
    ph: PhantomData<S>,
    cur_pos: f64,
    cur_speed: i8,
    error: f64,
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
            cur_pos: 0.,
            cur_speed: 0,
            error: 0.,
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

    fn send<Data: serial::serialtrait::Sendable>(&mut self, _code: u8, _data: Data) -> Result<(), super::MotorError>
    where
        [(); <((u8, u8), (Data, u8))>::SIZE]:,
        [(); Data::SIZE]:,
        [(); <(Data, u8)>::SIZE]: {
        unimplemented!()
    }

    fn read<Res: serial::serialtrait::Sendable>(&mut self) -> Result<Res, super::MotorError>
    where
        [(); Res::SIZE]:,
        [(); <(u8, Res)>::SIZE]:,
        [(); <((u8, Res), u8)>::SIZE]: {
        unimplemented!()
    }

    fn send_cmd<Data: serial::serialtrait::Sendable, Res: serial::serialtrait::Sendable>(
        &mut self,
        _code: u8,
        _data: Data,
    ) -> Result<Res, super::MotorError>
    where
        [(); Data::SIZE]:,
        [(); <(Data, u8)>::SIZE]:,
        [(); <((u8, u8), (Data, u8))>::SIZE]:,

        [(); Res::SIZE]:,
        [(); <(u8, Res)>::SIZE]:,
        [(); <((u8, Res), u8)>::SIZE]: {
        unimplemented!()
    }

    fn read_encoder_value(&mut self) -> Result<f64, super::MotorError> {
        let speed_rpm = (self.cur_speed as f64)*30000./(self.microstep as f64)/200.;
        //println!("{} {} {}", self.microstep, speed_rpm, speed_rpm/60./1000.);
        self.cur_pos+=speed_rpm/60./1000.*self.microstep as f64; //Vrpm = (Speed × 30000)/(Mstep × 200)(RPM)   (0.9 degree motor)
        //println!("{} {} {}", self.cur_pos, speed_rpm ,speed_rpm/60./1000.);
        Ok(self.cur_pos)
    }

    fn read_recived_pulses(&mut self) -> Result<f64, super::MotorError> {
        self.read_encoder_value()
    }

    fn read_error(&mut self) -> Result<f64, super::MotorError> {
        Ok(self.error)
    }

    fn read_en_pin(&mut self) -> Result<bool, super::MotorError> {
        Ok(true)
    }

    fn release_lock(&mut self) -> Result<(), super::MotorError> {
        Ok(())
    }

    fn read_lock(&mut self) -> Result<super::Protection, super::MotorError> {
        Ok(super::Protection::UnProtected)
    }

    fn calibrate(&mut self) -> Result<(), super::MotorError> {
        Ok(())
    }

    fn set_mot_type(&mut self, _mot_type: super::MotType) -> Result<(), super::MotorError> {
        Ok(())
    }

    fn set_mode(&mut self, _work_mode: super::WorkMode) -> Result<(), super::MotorError> {
        Ok(())
    }

    fn set_current(&mut self, _t: u8) -> Result<(), super::MotorError> {
        Ok(())
    }

    fn set_microstep(&mut self, mstep: u8) -> Result<(), super::MotorError> {
        self.microstep=mstep;
        Ok(())
    }

    fn set_en_active(&mut self, _active_on: super::ActiveOn) -> Result<(), super::MotorError> {
        Ok(())
    }

    fn set_direction(&mut self, _dir: super::Dir) -> Result<(), super::MotorError> {
        Ok(())
    }

    fn set_autossd(&mut self, _active: bool) -> Result<(), super::MotorError> {
        Ok(())
    }

    fn set_lock(&mut self, _protection: super::Protection) -> Result<(), super::MotorError> {
        Ok(())
    }

    fn set_subdivision_interpolation(&mut self, _active: bool) -> Result<(), super::MotorError> {
        Ok(())
    }

    fn set_baudrate(&mut self, _baud_rate: super::BaudRate) -> Result<(), super::MotorError> {
        Ok(())
    }

    fn set_slave_address(&mut self, _addr: u8) -> Result<(), super::MotorError> {
        Ok(())
    }

    fn reset(&mut self) -> Result<(), super::MotorError> {
        Ok(())
    }

    fn set_zero_mode(&mut self, _mode: u8) -> Result<(), super::MotorError> {
        Ok(())
    }

    fn set_zero(&mut self) -> Result<(), super::MotorError> {
        Ok(())
    }

    fn set_zero_speed(&mut self, _speed: u8) -> Result<(), super::MotorError> {
        Ok(())
    }

    fn set_zero_dir(&mut self, _dir: u8) -> Result<(), super::MotorError> {
        Ok(())
    }

    fn goto_zero(&mut self) -> Result<(), super::MotorError> {
        Ok(())
    }

    fn set_kp(&mut self, _kp: u16) -> Result<(), super::MotorError> {
        Ok(())
    }

    fn set_ki(&mut self, _ki: u16) -> Result<(), super::MotorError> {
        Ok(())
    }

    fn set_kd(&mut self, _kd: u16) -> Result<(), super::MotorError> {
        Ok(())
    }

    fn set_acc(&mut self, _acc: u16) -> Result<(), super::MotorError> {
        Ok(())
    }

    fn set_maxt(&mut self, _kp: Option<u16>) -> Result<(), super::MotorError> {
        Ok(())
    }

    fn set_enable(&mut self, _en: bool) -> Result<(), MotorError> {
        Ok(())
    }

    fn set_speed(&mut self, speed: i8) -> Result<(), MotorError> {
        self.cur_speed=speed;
        Ok(())
    }

    fn stop(&mut self) -> Result<(), MotorError> {
        self.cur_speed=0;
        Ok(())
    }

    fn goto(&mut self, _speed: u8, _dist: u32) -> u8 {
        unimplemented!()
    }
    fn get_microstep(&self)->u8 {
        self.microstep
    }
}