use core::marker::PhantomData;
use core::time::Duration;

use serial::serialtrait::Serial;

use crate::prelude::*;

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

    pub kp: u16,
    pub ki: u16,
    pub kd: u16,
    pub cur: u8,
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
        if self.m.read_lock()?== Protection::Protected {
            //let _ = self.m.stop();
            let _ =self.m.set_enable(false);
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
            
            if (abs(self.cur_speed)*127./20.) <1.0{
                self.cur_speed = if self.cur_speed<0.{
                    -20.
                }else{
                    20.
                }/127.0;
            }
            //println!("{} speed={} {}", self.pos-self.obbiettivo, self.cur_speed, self.cur_speed*127./20.);
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
        let mut s: Servo42LinearAcc<T, S>= Servo42LinearAcc{
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
        let _ =s.m.release_lock();
        let _= s.m.goto_zero();
        s.m.set_zero_mode(0)?;
        let _=s.m.set_enable(true);
        s.m.set_lock(Protection::Protected)?;
        Ok(s)
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

            kp: 1616,
            ki: 288,
            kd: 1616,
            cur: 6,
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
        z.precision=0.002;
        let mut m: Servo42LinearAcc<SerialTest, Servo42CTest<SerialTest>> = z.build().unwrap();
        let time = if dist*acc<=speed*speed{
            (dist/acc).sqrt()*2.
        }else{
            2.*speed/acc+(dist-speed*speed/acc-m.precision)/speed
        };
        let mut iter=0;
        let max_iter=(time*1150.)as i32;
        let min_iter=(time*900.)as i32;

        m.goto(dist).unwrap();
        while UpdateStatus::Working== m.update(Duration::from_millis(1)).unwrap() && iter<max_iter{
            iter+=1;
        }
        //
        if !(min_iter<iter && iter<max_iter){
            println!("{} {} {}", acc, speed, dist);
            println!("{}<{}<{} {}", min_iter, iter, max_iter, m.obbiettivo-m.pos);
        }
        //assert_eq!(m.obbiettivo-m.pos, 0.);
        assert!(min_iter<iter && iter<max_iter)
    }
    #[test]
    fn arrivo(){
        for x in 1..=10{
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