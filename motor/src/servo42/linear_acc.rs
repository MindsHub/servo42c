use core::time::Duration;

use serial::serialtrait::{Serial, SerialError};

use super::{Motor, MotorBuilder};

use super::{MotorError, Servo42C};

pub struct Servo42LinearAcc<T: Serial> {
    pub m: Servo42C<T>,
    pub obbiettivo: f64,
    pub cur_speed: f64,
    pub pos: f64,
    pub max_speed: f64,
    pub acc: f64,
}
pub struct Servo42LinearAccBuilder<T: Serial> {
    s: T,
    max_speed: f64,
    acc: f64,
}

impl<T: Serial> Motor for Servo42LinearAcc<T> {
    type PosUnit = f64;
    type Info = MotorError;
    type Builder = Servo42LinearAccBuilder<T>;

    fn goto(&mut self, pos: Self::PosUnit) -> Result<(), ()> {
        self.obbiettivo = pos;
        //self.m.goto(10, 1000);
        //let _ =self.m.set_speed(10);
        Ok(())
    }

    fn update(&mut self, time_from_last: Duration) -> Result<(), MotorError> {
        self.pos = self.m.read_recived_pulses().unwrap() / self.m.microstep as f64;

        //calcolo lo spazio di frenata s=V^2/2a
        let d_stop: f64 = self.cur_speed * self.cur_speed / 2. / self.acc;
        let distanza_rimanente = self.obbiettivo - self.pos as f64;
        let speed_dif = self.acc * time_from_last.as_secs_f64();

        let change_speed = |speed: &mut f64, quantity: f64| {
            if *speed > 0. {
                *speed += quantity;
            } else {
                *speed -= quantity;
            }
        };
        let abs = |val: f64| {
            if val > 0. {
                val
            } else {
                -val
            }
        };

        if distanza_rimanente * self.cur_speed >= 0. {
            //change_speed(&mut self.cur_speed, speed_dif);

            //se vado nella direzione corretta
            if abs(distanza_rimanente) > d_stop {
                //e ho spazio accelero
                change_speed(&mut self.cur_speed, speed_dif);
            } else {
                //se non ho spazio rallento
                change_speed(&mut self.cur_speed, -speed_dif);
            }
        } else {
            //rallento se vado nella direzione sbagliata
            change_speed(&mut self.cur_speed, -speed_dif);
        }
        if self.cur_speed > self.max_speed {
            self.cur_speed = self.max_speed;
        }
        if self.cur_speed < -self.max_speed {
            self.cur_speed = -self.max_speed;
        }
        let to_set = 200. * self.m.microstep as f64 / 500. * self.cur_speed;
        let _ = self.m.set_speed(to_set as i8);

        Ok(())
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

    fn build(self) -> Result<Self::M, SerialError> {
        Ok(Self::M {
            m: Servo42C::new(self.s)?,
            obbiettivo: 0.,
            cur_speed: 0.,
            pos: 0.,
            max_speed: self.max_speed,
            acc: self.acc,
        })
    }
}

impl<T: Serial> Servo42LinearAccBuilder<T> {
    pub fn new(s: T) -> Servo42LinearAccBuilder<T> {
        Servo42LinearAccBuilder {
            s: s,
            max_speed: 1.,
            acc: 0.25,
        }
    }
}
