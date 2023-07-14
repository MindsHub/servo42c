use crate::{motortrait::{Motor, MotorBuilder}, servo42::MotorError};

//struttura di test, qua dichiaro tutto quello che mi serve
pub struct MotorTest{
    obbiettivo: f64,
    cur_speed: f64,
    pos: f64,
    max_speed: f64,
    acc: f64,
    spazio_di_frenata: f64,
}

// builder, tutti i parametri che voglio siano configurabili vanno dichiarati qui
pub struct MotorTestBuilder{
    max_speed: f64,
    acc: f64,
}

// implementazione dell'interfaccia motore
impl Motor for MotorTest{
    //che unità mi aspetto per la posizione
    type PosUnit=f64;

    //unità per le info(deve implementare debug)
    type Info=MotorError;

    //chi è il builder?
    type Builder=MotorTestBuilder;

    //funzione per impostare l'obbiettivo
    fn goto(&mut self, obj: Self::PosUnit) -> Result<(), ()> {
        self.obbiettivo=obj;
        Ok(())
    }

    //(non implementato)
    fn get_info(&mut self) -> Self::Info {
        todo!()
    }

    //funzione di aggiornamento, viene chiamata di continuo
    fn update(&mut self, time_from_last: core::time::Duration) -> Result<(), crate::servo42::MotorError> {
        //leggiamo la posizione corrente... (nel caso simulato siamo sempre perfetti)
        self.pos+=self.cur_speed*time_from_last.as_secs_f64();

        //calcolo lo spazio di frenata s=V^2/2a 
        let d_stop: f64 = self.cur_speed*self.cur_speed/2./self.acc;
        let distanza_rimanente=self.obbiettivo-self.pos;
        if distanza_rimanente>=d_stop{
            //se mi manca ancora tanto accelero
            //se la distanza che ci manca ha lo stesso segno della velocità stiamo andando nella direzione corretta
            if distanza_rimanente*self.cur_speed>=0.{
                let da_calare = time_from_last.as_secs_f64()*self.acc;
                self.s
            }else{
                //direzione sbagliata

            }
        }else{
            //freno
        }
        
        todo!()
    }

    // resettiamo, troviamo finecorsa ecc
    fn reset(&mut self) {
        //cerchiamo il finecorsa e ci fermiamo....
        self.set_zero();
    }

    //
    fn set_zero(&mut self) {
        self.pos=0.;
        self.cur_speed=0.;
    }

    fn new() -> Self::Builder {
        Self::Builder::new()
    }
}

impl MotorBuilder<MotorTest> for MotorTestBuilder{
    fn build(self) -> MotorTest {
        MotorTest {
            obbiettivo: 0.0,
            cur_speed: 0.0,
            pos: 0.0,
            max_speed: self.max_speed,
            acc: self.acc,
            
        }
    }
}

impl MotorTestBuilder{
    pub fn new()->MotorTestBuilder{
        MotorTestBuilder { 
            max_speed: 10.,
            acc: 10.,
        }
    }
    pub fn set_max_speed(mut self, max_speed: f64)->Self{
        self.max_speed=max_speed;
        self
    }
    pub fn set_acc(mut self, max_acc: f64)->Self{
        self.acc=max_acc;
        self
    }
}

#[cfg(test)]
mod test{
    use crate::motortrait::{Motor, MotorBuilder};

    use super::MotorTest;


    #[test]
    fn test_build(){
        let _m = MotorTest::new()
            .set_acc(11.0)
            .set_max_speed(11.0)
            .build();
        
    }
}