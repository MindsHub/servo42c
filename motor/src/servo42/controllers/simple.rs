use core::marker::PhantomData;
use core::time::Duration;

use crate::prelude::*;
use serial::serialtrait::Serial;

use super::linear_acc::Servo42LinearAccBuilder;

#[derive(Debug)]
pub struct Simple<T: Serial, S: Servo42CTrait<T>> {
    //motore incapsulato
    pub m: S,

    //dati aggiuntivi
    pub obbiettivo: f64,
    pub cur_speed: f64,
    pub pos: f64,

    //configurazione
    pub max_speed: f64,
    pub acc: f64,
    pub max_err: f64,
    pub precision: f64,
    ph: PhantomData<T>,
}
pub struct SimpleBuilder<T: Serial> {
    pub s: T,
    pub max_speed: f64,
    pub acc: f64,
    pub max_err: f64,
    pub precision: f64,
}

impl<T: Serial, S: Servo42CTrait<T>> Motor for Simple<T, S> {
    type PosUnit = f64;
    type Info = MotorError;
    type Builder = Servo42LinearAccBuilder<T>; //TODO REMOVE

    fn goto(&mut self, pos: Self::PosUnit) -> Result<(), MotorError> {
        /*let mut speed=self.max_speed;
        let dist=self.obbiettivo-self.pos;
        if dist<0.{
            speed=-speed;
        }
        let to_set = (200. * self.m.get_microstep() as f64 / 500. * speed)as i8;
        let to_set = if to_set < 0 {
            -to_set as u8
        } else {
            to_set as u8 | 0x80
        };
        self.m.goto(to_set, (dist*20.*self.m.get_microstep() as f64) as u32);
        self.obbiettivo = pos;*/
        let _ = self.m.set_acc(200);

        //let _ = self.m.stop();
        //let _ = self.m.set_enable(true);
        self.m.goto(10, 200 * self.m.get_microstep() as u32 * 3);
        Ok(())
    }

    fn update(&mut self, _time_from_last: Duration) -> Result<UpdateStatus, MotorError> {
        Ok(UpdateStatus::Working)
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

impl<T: Serial, S: Servo42CTrait<T>> MotorBuilder<Simple<T, S>> for Servo42LinearAccBuilder<T> {
    fn build(self) -> Result<Simple<T, S>, MotorError> {
        let mut s: Simple<T, S> = Simple {
            m: Servo42CTrait::new(self.s)?,
            obbiettivo: 0.,
            cur_speed: 0.,
            pos: 0.,
            max_speed: self.max_speed,
            acc: self.acc,
            max_err: self.max_err,
            precision: self.precision,
            ph: PhantomData,
        };

        s.m.stop()?;
        s.m.set_kp(self.kp)?;
        s.m.set_ki(self.ki)?;
        s.m.set_kd(self.kd)?;
        s.m.set_acc(288)?;
        s.m.set_maxt(Some(2000))?;
        s.m.set_current(3)?;
        s.m.set_lock(Protection::Protected)?;
        let _ = s.m.release_lock();
        let _ = s.m.goto_zero();
        s.m.set_zero_mode(0)?;
        let _ = s.m.set_enable(true);
        s.m.set_lock(Protection::Protected)?;

        Ok(s)
    }
}

impl<T: Serial> SimpleBuilder<T> {
    pub fn new(s: T) -> SimpleBuilder<T> {
        SimpleBuilder {
            s,
            max_speed: 10.,
            acc: 10.,
            max_err: 0.045,
            precision: 0.005,
        }
    }
}
