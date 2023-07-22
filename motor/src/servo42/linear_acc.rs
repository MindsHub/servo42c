use core::time::Duration;

use serial::serialtrait::Serial;

use super::{Motor, MotorBuilder};

use super::{Servo42C, MotorError};

pub struct Servo42LinearAcc<T: Serial> {
    pub m: Servo42C<T>,
    pub obbiettivo: f64,
    cur_speed: f64,
    pub pos: i64,
    max_speed: f64,
    acc: f64,
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
        self.obbiettivo=pos;
        //self.m.goto(10, 1000);
        //let _ =self.m.set_speed(10);
        Ok(())
    }

    fn update(&mut self, _time_from_last: Duration) -> Result<(), MotorError> {
        self.pos=self.m.read_encoder_value().unwrap() ;
        
        //calcolo lo spazio di frenata s=V^2/2a 
        //let d_stop: f64 = self.cur_speed*self.cur_speed/2./self.acc;
        let distanza_rimanente=self.obbiettivo-self.pos as f64;
        if distanza_rimanente<0.{
            self.m.set_speed(1).unwrap();
        }else{
            self.m.set_speed(-1).unwrap();
        }
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
    type M=Servo42LinearAcc<T>;

    fn build(self) -> Self::M {
        Self::M{
            m: Servo42C::new(self.s).unwrap(),
            obbiettivo: 0.,
            cur_speed: 0.,
            pos: 0,
            max_speed: self.max_speed,
            acc: self.acc,
            
        }
    }

}

impl <T: Serial> Servo42LinearAccBuilder<T>{
    pub fn new(s: T)->Servo42LinearAccBuilder<T>{
        Servo42LinearAccBuilder{
            s: s,
            max_speed: 10.,
            acc: 10.,        }
    }
}