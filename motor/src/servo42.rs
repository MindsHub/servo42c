use serial::serialtrait::{Serial, Sendable, SerialError};
use serial::serialtrait::MySize;
pub struct Servo42C<T: Serial>{
    pub address: u8,
    pub s: T,
}

impl<T: Serial> Servo42C<T>{
    pub fn send_cmd<Data: Sendable, Res: Sendable>(&mut self, code: u8, data: Data)->Result<Res, SerialError>
        where 
        [(); <((u8, u8), Data)>::SIZE]:,
        [(); Data::SIZE]:,
        [(); Data::SIZE+3]:,
        [(); Res::SIZE+2]:,
        [(); <(u8, Res)>::SIZE]:,
        [(); <((u8, Res), u8)>::SIZE]:,
        {
        let data_buf =((self.address, code), data).as_u8() ;
        let chksm: u8 = (data_buf.iter().fold(0u32, |acc, x|acc+*x as u32)%256) as u8;
        let mut to_send = [0u8; Data::SIZE+3];
        to_send[..Data::SIZE+2].copy_from_slice(&data_buf[..{Data::SIZE+2}]);
        to_send[Data::SIZE+2]=chksm;
        self.s.write(&to_send)?;
        let mut readen=[0u8; <((u8, Res), u8)>::SIZE];
        self.s.read(&mut readen)?;
        let chksm = (readen[..(Res::SIZE+1)].iter().fold(0u32, |acc, x|acc+*x as u32)%256) as u8;
        if *readen.last().unwrap()!=chksm{
            println!("invalid checksum {} {chksm}", *readen.last().unwrap());
            return Err(SerialError::Undefined);
        }
        let result =<((u8, Res), u8)>::from_u8(readen);
        if result.0.0!=self.address{
            println!("invalid address");
            return Err(SerialError::Undefined);
        } 
        Ok(result.0.1)
    }
}

//read impl block
impl<T: Serial> Servo42C<T>{
    /**
    read the encoder value (the motor should be calibrated)
    returns (carry, value)  where
    - carry is the value of the encoder (giri?)
    - current value of the encoder (fase)
     */
    pub fn read_encoder_value(&mut self)->(i32, u16){
        self.send_cmd(0x30, ()).unwrap()
    }

    /**
     Read the number of pulses received.
    */
    pub fn read_recived_pulses(&mut self)->i32{
        self.send_cmd(0x33, ()).unwrap()
    }

    /**recived_pulses
     * read the error of the motor shaft angle
     The error is the difference between the angle you want to control 
     minus  the  real-time  angle  of  the  motor,  0~FFFF  corresponds  to 
     0~360°. 
     for  example,  when  the  angle  error  is  1°,  the  return  error  is 
     65536/360= 182.444, and so on.
     */
    pub fn read_error(&mut self)->i16{
        self.send_cmd(0x39, ()).unwrap()
    }

    /**
     read the En pins status
     */
    pub fn read_en_pin(&mut self)->bool{
        let ret: u8=self.send_cmd(0x3A, ()).unwrap();
        /*
        enable =1  Enabled   
        enable =2 Disabled
         */
        ret==1
    }

    /**
     Release the motor shaft locked-rotor protection state
    status =1 release success.    
    status =0 release fail
     */
    pub fn release_lock(&mut self)->Result<(), ()>{
        let ret: u8=self.send_cmd(0x3D, ()).unwrap();
        if ret==1{
            Ok(())
        }else{
            Err(())
        }
    }
    /**
    Read the motor shaft protection state.
    status =1  protected.   
    status =2  no protected
     */
    pub fn read_lock(&mut self)->u8{
        self.send_cmd(0x3E, ()).unwrap()
    }
}

///set impl block
impl<T: Serial> Servo42C<T>{
    /**
     Calibrate the encoder
     （Same as the "Cal" option on screen）
     - status =1  Calibrated success.  
     - status =2  Calibrating fail. 
     Note : The motor must be unloaded.
     */
    pub fn calibrate(&mut self)->Result<(), ()>{
        let ret: u8= self.send_cmd(0x80, 0x00u8).unwrap();
        if ret==1{
            Ok(())
        }else{
            Err(())
        }
    }

    /**
     Set the Motor Type 
     Same as the "MotType" option on screen
     - Type = 0 0.9 degree motor 
     - Type = 1 1.8 degree motor
     status =1  Set success.    
     status =0  Set fail.
     */
    pub fn set_mot_type(&mut self, t: u8)->Result<(), ()>{
        let ret: u8=self.send_cmd(0x81, t).unwrap();
        if ret==1{
            Ok(())
        }else{
            Err(())
        }
    }

    /**
     Set the work mode 
     Same as the "Mode" option on screen
     - mode = 0 CR_OPEN  
     - mode = 1 CR_vFOC 
     - mode = 2 CR_UART
     status =1  Set success.    
     status =0  Set fail
     */
    pub fn set_mode(&mut self, t: u8)->Result<(), ()>{
        let ret: u8 =self.send_cmd(0x82, t).unwrap();
        if ret==1{
            Ok(())
        }else{
            Err(())
        }
    }

    /**
     Set the current
     Same as the "Ma" option on screen
     The current = ma x 200 mA
     status =1  Set success.    
     status =0  Set fail.
    */
    pub fn set_current(&mut self, t: u8)->Result<(), ()>{
        let ret: u8=self.send_cmd(0x83, t).unwrap();
        if ret==1{
            Ok(())
        }else{
            Err(())
        }
    }

    /**
     Set microstep
     Note:the new micstep will show in the screen of MStep option.
     status =1  Set success.    
     status =0  Set fail. 
     */
    pub fn set_microstep(&mut self, t: u8)->Result<(), ()>{
        let ret: u8=self.send_cmd(0x84, t).unwrap();
        if ret==1{
            Ok(())
        }else{
            Err(())
        }
    }

    /**
     Set the active of the En pin
     （Same as the "En" option on screen）
     - enable = 00   active low  (L) 
     - enable = 01   active high   (H) 
     - enable = 02   active always (Hold) 
     */
    pub fn set_en_active(&mut self, t: u8)->Result<(), ()>{
        let ret: u8=self.send_cmd(0x85, t).unwrap();
        if ret==1{
            Ok(())
        }else{
            Err(())
        }
    }

    /**
     Set the direction of motor rotation 
     Same as the "Dir" option on screen
     - dir = 00   CW 
     - dir = 01   CCW
     */
    pub fn set_direction(&mut self, t: u8)->Result<(), ()>{
        let ret: u8=self.send_cmd(0x86, t).unwrap();
        if ret==1{
            Ok(())
        }else{
            Err(())
        }
    }

    /**
     Set automatic turn off the screen
     Same as the "AutoSDD" option on screen
     */
    pub fn set_autossd(&mut self, active: bool)->Result<(), ()>{
        let mut t:u8=0;
        if active{
            t+=1;
        }
        let ret: u8=self.send_cmd(0x87, t).unwrap();
        if ret==1{
            Ok(())
        }else{
            Err(())
        }
    }

    /**
     Set the motor shaft locked-rotor protection function
     Same as the "Protect" option on screen
     */
    pub fn set_lock(&mut self, active: bool)->Result<(), ()>{
        let mut t:u8=0;
        if active{
            t+=1;
        }
        let ret: u8=self.send_cmd(0x88, t).unwrap();
        if ret==1{
            Ok(())
        }else{
            Err(())
        }
    }

    /**
     Set the subdivision interpolation function 
    （Same as the "Mplyer" option on screen）
    enabled interpolation function.
     */
    pub fn set_subdivision_interpolation(&mut self, active: bool)->Result<(), ()>{
        let mut t:u8=0;
        if active{
            t+=1;
        }
        let ret: u8=self.send_cmd(0x89, t).unwrap();
        if ret==1{
            Ok(())
        }else{
            Err(())
        }
    }

    /**
     Set the baud rate 
    Same as the "UartBaud" option on screen
     - baud = 01   9600. 
     - baud = 02   19200. 
     - baud = 03   25000. 
     - baud = 04   38400. 
     - baud = 05   57600. 
     - baud = 06   115200
     */
    pub fn set_baudrate(&mut self, t: u8)->Result<(), ()>{
        let ret: u8=self.send_cmd(0x8A, t).unwrap();
        if ret==1{
            Ok(())
        }else{
            Err(())
        }
    }

    /**
     Set the slave address 
    （Same as the "UautAddr" option on screen）
     Slave address = addr + 0xe0 
     addr from 0-9
     */
    pub fn set_slave_address(&mut self, t: u8)->Result<(), ()>{
        let ret: u8=self.send_cmd(0x8B, t).unwrap();
        if ret==1{
            Ok(())
        }else{
            Err(())
        }
    }

    /**
     Restore the default parameter 
    （Same as the "Restore" option on screen）
    NOTE: after resetting it must be rebooted, and the serial comunication baudrate must be manually set
     */
    pub fn reset(&mut self)->Result<(), ()>{
        let ret: u8=self.send_cmd(0x3F, ()).unwrap();
        if ret==1{
            Ok(())
        }else{
            Err(())
        }
    }

}

///set zero mode(how to return to zero on poweron)
impl<T: Serial> Servo42C<T>{
    /**
     Set the mode of zeroMode 
    （Same as the " 0_Mode " option on screen）
     - mode = 00  Disable 
     - mode = 01  DirMode 
     - mode = 02  NearMode
     */
    pub fn set_zero_mode(&mut self, mode: u8)->Result<(), ()>{
        let ret: u8= self.send_cmd(0x90, mode).unwrap();
        if ret==1{
            Ok(())
        }else{
            Err(())
        }
    }

    /**
     Set zero of zeroMode 
    （Same as the " set 0 " option on screen）
    NOTE: The mode of "0_Mode" needs to be set first. 
     */
    pub fn set_zero(&mut self)->Result<(), ()>{
        let ret: u8= self.send_cmd(0x91, 0u8).unwrap();
        if ret==1{
            Ok(())
        }else{
            Err(())
        }
    }

    /**
     Set zero speed 
    （Same as the " 0_Speed" option on screen）
    NOTE: The mode of "0_Mode" needs to be set first.
    (speed = 0~4, the smaller the value, the faster the speed)
     */
    pub fn set_zero_speed(&mut self, speed: u8)->Result<(), ()>{
        let ret: u8= self.send_cmd(0x92, speed).unwrap();
        if ret==1{
            Ok(())
        }else{
            Err(())
        }
    }

    /**
     Set dir of zero mode
    （Same as the " 0_Dir" option on screen）
    NOTE: The mode of "0_Mode" needs to be set first.
     - dir = 00 CW 
     - dir = 01 CCW
     Note: For NearMode, the setting of 0_Dir should be consistent 
    with the actual running direction of the motor, otherwise it will 
    fail to return to zero
     */
    pub fn set_zero_dir(&mut self, dir: u8)->Result<(), ()>{
        let ret: u8= self.send_cmd(0x93, dir).unwrap();
        if ret==1{
            Ok(())
        }else{
            Err(())
        }
    }

    /**
     Set dir of zero mode
    （Same as the " Goto 0" option on screen）
     */
    pub fn goto_zero(&mut self)->Result<(), ()>{
        let ret: u8= self.send_cmd(0x94, 0u8).unwrap();
        if ret==1{
            Ok(())
        }else{
            Err(())
        }
    }
}

//Set PID/ACC/Torque command
impl<T: Serial> Servo42C<T>{
    /**
     Set the position Kp parameter
     */
    pub fn set_kp(&mut self, kp: u16)->Result<(), ()>{
        let ret: u8= self.send_cmd(0xA1, kp).unwrap();
        if ret==1{
            Ok(())
        }else{
            Err(())
        }
    }
}
//Serial control comands
impl<T: Serial> Servo42C<T>{
    /**
     Set the En pin status in CR_UART mode.
     */
    pub fn set_enable(&mut self, en: bool)->Result<u8, ()>{
        let mut b=0u8;
        if en{
            b+=1;
        }
        let ret: u8= self.send_cmd(0xF3, b).unwrap();
        Ok(ret)
    }

    /**
     Set 0_Speed
     run the motor forward / reverse in a Constant speed.
     Direction : The highest 1bit of VAL. 
     Speed : The lowest 7bit of VAL. 
     for example：
     The Vrpm calculation formula is: 
    Vrpm = (Speed × 30000)/(Mstep × 200)(RPM)   (1.8 degree motor) 
    Vrpm = (Speed × 30000)/(Mstep × 400)(RPM)   (0.9 degree motor) 
     */
    pub fn set_speed(&mut self, dir: bool, mut speed: u8)->Result<u8, ()>{
        if dir{
            speed=speed|0x80;
        }
        let ret: u8= self.send_cmd(0xF6, speed).unwrap();
        Ok(ret)
    }

    /**
     Calibrate the encoder
     （Same as the "Cal" option on screen）
     - status =1  Calibrated success.  
     - status =2  Calibrating fail. 
     Note : The motor must be unloaded.
     */
    pub fn goto(&mut self, speed: u8, dist: u32)->Result<u8, ()>{
        let ret: u8= self.send_cmd(0xFD, (speed, dist)).unwrap();
        Ok(ret)
    }
}

#[cfg(test)]
mod tests{
    use serial::test::SerialTest;
    use super::*;
    macro_rules! test_motor {
        ($name:ident ($($arg:literal),*) ($($val:literal) *)->($($ret:literal) *)) => {
            #[test]
            fn $name(){
                let mut s = SerialTest::default();
                s.add_response(vec![$($val),*], vec![$($ret),*]);
                let mut servo=Servo42C{address: 0xe0, s};
                let _ =servo.$name($($arg),*);
            }
        };
    }
    
    test_motor!(read_encoder_value() (0xe0 0x30 0x10)->(0xe0 00 00 00 00 0x40 00 0x20));
    test_motor!(read_recived_pulses() (0xe0 0x33 0x13)->(0xe0 00 00 0x01 00 0xe1));
    test_motor!(read_error() (0xe0 0x39 0x19)->(0xe0 00 0xB7 0x97));
    test_motor!(read_en_pin() (0xe0 0x3a 0x1a)->(0xe0 0x01 0xe1));
    test_motor!(release_lock() (0xe0 0x3d 0x1d)->(0xe0 0x01 0xe1));
    test_motor!(read_lock() (0xe0 0x3e 0x1e)->(0xe0 0x02 0xe2));
    test_motor!(calibrate() (0xe0 0x80 0x00 0x60)->(0xe0 0x01 0xe1));
    test_motor!(set_mot_type(1) (0xe0 0x81 0x01 0x62)->(0xe0 0x01 0xe1));
    test_motor!(set_mode(1) (0xe0 0x82 0x01 0x63)->(0xe0 0x01 0xe1));
    test_motor!(set_current(6) (0xe0 0x83 0x06 0x69)->(0xe0 0x01 0xe1));
    test_motor!(set_microstep(26) (0xe0 0x84 0x1a 0x7e)->(0xe0 0x01 0xe1));
    test_motor!(set_en_active(0) (0xe0 0x85 0x00 0x65)->(0xe0 0x01 0xe1));
    test_motor!(set_direction(0) (0xe0 0x86 0x00 0x66)->(0xe0 0x01 0xe1));
    test_motor!(set_autossd(false) (0xe0 0x87 0x00 0x67)->(0xe0 0x01 0xe1));
    test_motor!(set_lock(false) (0xe0 0x88 0x00 0x68)->(0xe0 0x01 0xe1));
    test_motor!(set_subdivision_interpolation(false) (0xe0 0x89 0x00 0x69)->(0xe0 0x01 0xe1));
    test_motor!(set_baudrate(4) (0xe0 0x8A 0x04 0x6e)->(0xe0 0x01 0xe1));
    test_motor!(set_slave_address(2) (0xe0 0x8B 0x02 0x6d)->(0xe0 0x01 0xe1));
    test_motor!(reset() (0xe0 0x3f 0x1f)->(0xe0 0x01 0xe1));
    test_motor!(set_zero_mode(1) (0xe0 0x90 0x01 0x71)->(0xe0 0x01 0xe1));
    test_motor!(set_zero() (0xe0 0x91 0x00 0x71)->(0xe0 0x01 0xe1));
    test_motor!(set_zero_speed(2) (0xe0 0x92 0x02 0x74)->(0xe0 0x01 0xe1));
    test_motor!(set_zero_dir(0) (0xe0 0x93 0x00 0x73)->(0xe0 0x01 0xe1));
    test_motor!(goto_zero() (0xe0 0x94 0x00 0x74)->(0xe0 0x01 0xe1));
    
    test_motor!(set_enable(true) (0xe0 0xf3 0x01 0xd4)->(0xe0 0x01 0xe1));
    test_motor!(set_kp(8193) (0xe0 0xA1 0x01 0x20 0xA2)->(0xe0 0x01 0xe1));

}