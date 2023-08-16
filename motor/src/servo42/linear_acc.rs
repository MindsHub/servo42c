use core::time::Duration;

use serial::serialtrait::Serial;

use crate::motortrait::UpdateStatus;

use super::standard::Servo42C;
use super::{Motor, MotorBuilder};
use crate::servo42::Servo42CTrait;
use super::MotorError;
use libm;
///Helper function
impl<T: Serial> Servo42LinearAcc<T> {
    fn change_speed(&mut self, quantity: f64) {
        if self.cur_speed > 0. {
            self.cur_speed += quantity;
        } else {
            self.cur_speed -= quantity;
        }
        self.normalize_speed(self.max_speed);
    }
    fn normalize_speed(&mut self, quantity: f64) {
        if self.cur_speed > quantity {
            self.cur_speed = quantity;
            return;
        }
        if self.cur_speed < -quantity {
            self.cur_speed = -quantity;
        }
    }
}

fn abs(val: f64) -> f64 {
    if val > 0. {
        val
    } else {
        -val
    }
}

pub struct Servo42LinearAcc<T: Serial> {
    //motore incapsulato
    pub m: Servo42C<T>,

    //dati aggiuntivi
    pub obbiettivo: f64,
    pub cur_speed: f64,
    pub pos: f64,

    //configurazione
    pub max_speed: f64,
    pub acc: f64,
    pub max_err: f64,
    pub precision: f64,
}
pub struct Servo42LinearAccBuilder<T: Serial> {
    pub s: T,
    pub max_speed: f64,
    pub acc: f64,
    pub max_err: f64,
    pub precision: f64,
}

impl<T: Serial> Motor for Servo42LinearAcc<T> {
    type PosUnit = f64;
    type Info = MotorError;
    type Builder = Servo42LinearAccBuilder<T>;

    fn goto(&mut self, pos: Self::PosUnit) -> Result<(), MotorError> {
        self.obbiettivo = pos;
        Ok(())
    }

    fn update(&mut self, time_from_last: Duration) -> Result<UpdateStatus, MotorError> {
        if abs(self.m.read_error()?) > self.max_err {
            let _ = self.m.stop();
            return Err(MotorError::Stuck);
        }
        self.pos = self.m.read_recived_pulses()? / self.m.microstep as f64;
        let speed_dif = self.acc * time_from_last.as_secs_f64();
        let distanza_rimanente =
            self.obbiettivo - self.pos - self.cur_speed * time_from_last.as_secs_f64();
        if abs(self.obbiettivo - self.pos) < self.precision {
            self.m.stop()?;
            return Ok(UpdateStatus::GetThere);
        }
        if distanza_rimanente * self.cur_speed <= 0. {
            //direzione sbagliata:
            //rallenta
            self.change_speed(-speed_dif);
        } else {
            //direzione giusta:
            let max_speed = libm::sqrt(
                abs(distanza_rimanente) * self.acc + self.cur_speed * self.cur_speed / 2.,
            );
            let d_to_max =
                abs(distanza_rimanente) / 2. - self.cur_speed * self.cur_speed / (4.0 * self.acc);
            if d_to_max > 0. {
                //accelero
                self.change_speed(speed_dif);
                self.normalize_speed(max_speed);
                //self.normalize_speed(abs(d_to_max)/time_from_last.as_secs_f64());
            } else {
                //decelero
                self.change_speed(-speed_dif);
            }
        }

        let to_set = 200. * self.m.microstep as f64 / 500. * self.cur_speed;
        let _ = self.m.set_speed(to_set as i8);

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

impl<T: Serial> MotorBuilder for Servo42LinearAccBuilder<T> {
    type M = Servo42LinearAcc<T>;

    fn build(self) -> Result<Self::M, MotorError> {
        Ok(Self::M {
            m: Servo42C::new(self.s)?,
            obbiettivo: 0.,
            cur_speed: 0.,
            pos: 0.,
            max_speed: self.max_speed,
            acc: self.acc,
            max_err: self.max_err,
            precision: self.precision,
        })
    }
}

impl<T: Serial> Servo42LinearAccBuilder<T> {
    pub fn new(s: T) -> Servo42LinearAccBuilder<T> {
        Servo42LinearAccBuilder {
            s,
            max_speed: 10.,
            acc: 10.,
            max_err: 0.045,
            precision: 0.005,
        }
    }
}
