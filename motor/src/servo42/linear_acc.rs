use core::marker::PhantomData;
use core::time::Duration;

use serial::serialtrait::Serial;

use crate::motortrait::UpdateStatus;

use super::{Motor, MotorBuilder};
use crate::servo42::Servo42CTrait;
use super::MotorError;
use libm;
///Helper function
impl<T: Serial, S: Servo42CTrait<T>> Servo42LinearAcc<T, S> {
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
#[derive(Debug)]
pub struct Servo42LinearAcc<T: Serial, S: Servo42CTrait<T>> {
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
pub struct Servo42LinearAccBuilder<T: Serial> {
    pub s: T,
    pub max_speed: f64,
    pub acc: f64,
    pub max_err: f64,
    pub precision: f64,
}

impl<T: Serial, S: Servo42CTrait<T>> Motor for Servo42LinearAcc<T, S>{
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
        self.pos = self.m.read_recived_pulses()? / self.m.get_microstep() as f64;
        let speed_dif = self.acc * time_from_last.as_secs_f64();
        let distanza_rimanente =
            self.obbiettivo - self.pos - self.cur_speed * time_from_last.as_secs_f64();
        if abs(self.obbiettivo - self.pos) <= self.precision {
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

        let to_set = 200. * self.m.get_microstep() as f64 / 500. * self.cur_speed;
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

impl<T: Serial, S: Servo42CTrait<T>> MotorBuilder<Servo42LinearAcc<T, S>> for Servo42LinearAccBuilder<T> {
    fn build(self) -> Result<Servo42LinearAcc<T, S>, MotorError> {
        Ok( Servo42LinearAcc{
            m: Servo42CTrait::new(self.s)?,
            obbiettivo: 0.,
            cur_speed: 0.,
            pos: 0.,
            max_speed: self.max_speed,
            acc: self.acc,
            max_err: self.max_err,
            precision: self.precision,
            ph: PhantomData,
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

#[cfg(test)]
#[cfg(feature = "std")]
mod tests {
    use core::time::Duration;

    use serial::test::SerialTest;

    use crate::{motortrait::{MotorBuilder, Motor, UpdateStatus}, servo42::test::Servo42CTest};

    use super::{Servo42LinearAccBuilder, Servo42LinearAcc};

    fn muovi(acc: f64, speed: f64, dist: f64){
        let s=SerialTest::default();
        let mut z = Servo42LinearAccBuilder::new(s);
        z.acc=acc;
        z.max_speed=speed;
        z.precision=0.007;
        let mut m: Servo42LinearAcc<SerialTest, Servo42CTest<SerialTest>> = z.build().unwrap();
        let time = if dist*acc<=speed*speed{
            (dist/acc).sqrt()*2.
        }else{
            2.*speed/acc+(dist-speed*speed/acc-m.precision)/speed
        };
        let mut iter=0;
        let max_iter=(time*1120.)as i32;
        let min_iter=(time*900.)as i32;

        m.goto(dist).unwrap();
        while UpdateStatus::Working== m.update(Duration::from_millis(1)).unwrap() && iter<max_iter{
            iter+=1;
        }
        //println!("{}<{}<{} {}", min_iter, iter, max_iter, m.obbiettivo-m.pos);
        //assert_eq!(m.obbiettivo, m.pos);
        assert!(min_iter<iter && iter<max_iter)
    }
    #[test]
    fn arrivo(){
        for x in 2..=10{
            for y in 1..=10{
                for z in 1..=10{
                    //println!("{x} {y} {z}");
                    muovi(x as f64, y as f64, z as f64);
                }
            }
        }
        
    
        
       
        
    }

    /*#[bench]
    fn arrivo(b: &mut Bencher){
        b.iter(||{
            let s=SerialTest::default();
            let mut z = Servo42LinearAccBuilder::new(s);
            z.acc=10.0;
            z.max_speed=10.0;
            z.precision=0.005;
            let mut m: Servo42LinearAcc<SerialTest, Servo42CTest<SerialTest>> = z.build().unwrap();
            m.goto(10.0).unwrap();
            let mut iter=0;
            while UpdateStatus::Working== m.update(Duration::from_millis(1)).unwrap() && iter<10000{
                iter+=1;
            }
        
            assert!(1950<iter && iter<2050)
        });
        
    }*/
}