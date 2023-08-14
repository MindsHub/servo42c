use core::time::Duration;

use crate::{
    motortrait::{Motor, MotorBuilder, UpdateStatus},
    servo42::MotorError,
};

//struttura di test, qua dichiaro tutto quello che mi serveuse core::time::Duration;

use libm;
///Helper function
impl MotorTest{
    fn change_speed(&mut self, quantity: f64) {
        if self.cur_speed > 0. {
            self.cur_speed += quantity;
        } else {
            self.cur_speed -= quantity;
        }
        self.normalize_speed(self.max_speed);
    }
    fn normalize_speed(&mut self,  quantity: f64){
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

pub struct MotorTest {
    //dati aggiuntivi
    pub obbiettivo: f64,
    pub cur_speed: f64,
    pub pos: f64,

    //configurazione
    pub max_speed: f64,
    pub acc: f64,
}
pub struct MotorTestBuilder {
    pub max_speed: f64,
    pub acc: f64,
}

impl Motor for MotorTest {
    type PosUnit = f64;
    type Info = MotorError;
    type Builder = MotorTestBuilder;

    fn goto(&mut self, pos: Self::PosUnit) -> Result<(), MotorError> {
        self.obbiettivo = pos;
        Ok(())
    }
    fn update(&mut self, time_from_last: Duration) -> Result<UpdateStatus, MotorError> {
        self.pos += libm::round(self.cur_speed/self.max_speed*127.)*self.max_speed/127.*time_from_last.as_secs_f64();
        let speed_dif = self.acc * time_from_last.as_secs_f64();
        let distanza_rimanente = self.obbiettivo - self.pos-self.cur_speed*time_from_last.as_secs_f64();
        
        
        if distanza_rimanente * self.cur_speed <= 0. {
            //direzione sbagliata:
            //rallenta
            self.change_speed( -speed_dif);
        }else{
            //direzione giusta:
            let max_speed=libm::sqrt(abs(distanza_rimanente)*self.acc+self.cur_speed*self.cur_speed/2.);
            let d_to_max = abs(distanza_rimanente)/2.-self.cur_speed*self.cur_speed/(4.0*self.acc);
            if d_to_max>0.{
                //accelero
                self.change_speed(speed_dif);
                self.normalize_speed(max_speed);
                //self.normalize_speed(abs(d_to_max)/time_from_last.as_secs_f64());
                
            }else{
                //decelero
                self.change_speed(-speed_dif);
            }
        }
        Ok(UpdateStatus::GetThere)
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

impl MotorBuilder for MotorTestBuilder {
    type M = MotorTest;

    fn build(self) -> Result<Self::M, MotorError> {
        Ok(Self::M {
            obbiettivo: 0.,
            cur_speed: 0.,
            pos: 0.,
            max_speed: self.max_speed,
            acc: self.acc,
        })
    }
}

impl MotorTestBuilder{
    pub fn new() -> MotorTestBuilder{
        MotorTestBuilder {
            max_speed: 15.,
            acc: 10.,
        }
    }
}
