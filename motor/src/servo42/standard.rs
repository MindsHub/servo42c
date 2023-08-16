use serial::serialtrait::{MySize, SerialError};
use serial::serialtrait::{Sendable, Serial};

use super::MotorError::{self, *};
use super::{ActiveOn, BaudRate, Dir, MotType, Protection, Servo42CTrait, WorkMode};
pub struct Servo42C<S: Serial> {
    s: S,
    address: u8,
    pub microstep: u8,
    pub kp: u16,
    pub ki: u16,
    pub kd: u16,
    pub acc: u16,
}

impl<T: Serial> Servo42CTrait<T> for Servo42C<T> {
    fn empty_new(s: T) -> Servo42C<T> {
        Servo42C::<T> {
            address: 0xe0,
            s,
            kp: 1616, //1616,
            ki: 288,  //288,
            kd: 1616, //1616,
            acc: 286, //286,
            microstep: 16,
        }
    }

    fn new(s: T) -> Result<Servo42C<T>, MotorError> {
        let mut t = Servo42C::empty_new(s);
        t.stop()?;
        t.set_kp(t.kp)?;
        t.set_ki(t.ki)?;
        t.set_kd(t.kd)?;
        t.set_acc(t.acc)?;
        t.set_microstep(t.microstep)?;
        t.set_maxt(Some(2000))?;
        Ok(t)
    }
    
    fn send<Data: Sendable>(&mut self, code: u8, data: Data) -> Result<(), MotorError>
    where
        [(); <((u8, u8), (Data, u8))>::SIZE]:,
        [(); Data::SIZE]:,
        [(); <(Data, u8)>::SIZE]:,
    {
        let mut to_send = ((self.address, code), (data, 0u8)).into_byte();
        to_send[to_send.len() - 1] = to_send[..(to_send.len() - 1)]
            .iter()
            .fold(0u8, |x, y| x.overflowing_add(*y).0);
        self.s.write(&to_send)?;
        Ok(())
    }

    fn read<Res: Sendable>(&mut self) -> Result<Res, MotorError>
    where
        [(); Res::SIZE]:,
        [(); <(u8, Res)>::SIZE]:,
        [(); <((u8, Res), u8)>::SIZE]:,
    {
        let mut readen = [0u8; <((u8, Res), u8)>::SIZE];
        self.s.read(&mut readen)?;
        let chcksm = readen[..(readen.len() - 1)]
            .iter()
            .fold(0u8, |x, y| x.overflowing_add(*y).0);
        if *readen.last().unwrap() != chcksm {
            return Err(SerialError::Undefined.into());
        }
        let result = <((u8, Res), u8)>::from_byte(readen);
        if result.0 .0 != self.address {
            return Err(SerialError::Undefined.into());
        }
        Ok(result.0 .1)
    }

    fn send_cmd<Data: Sendable, Res: Sendable>(
        &mut self,
        code: u8,
        data: Data,
    ) -> Result<Res, MotorError>
    where
        [(); Data::SIZE]:,
        [(); <(Data, u8)>::SIZE]:,
        [(); <((u8, u8), (Data, u8))>::SIZE]:,

        [(); Res::SIZE]:,
        [(); <(u8, Res)>::SIZE]:,
        [(); <((u8, Res), u8)>::SIZE]:,
    {
        self.send(code, data)?;
        let r: Res = self.read()?;
        Ok(r)
    }

    /**
     * this implementation returns an f64 that rappresent the number of full rotations
    read the encoder value (the motor should be calibrated)
    returns (carry, value)  where
    - carry is the value of the encoder (giri?)
    - current value of the encoder (fase)
     */
    fn read_encoder_value(&mut self) -> Result<f64, MotorError> {
        //even if we read an int is easyer to manage an f64 (64 bit so we manage even big numbers losless)
        //we retourn
        let (rotations, phase): (i32, u16) = self.send_cmd(0x30, ())?;
        let output: f64 = rotations as f64 + (phase as f64) / 65536.;
        //let tot = ((rotations as i64).shl(16) + phase as i64)/182i64;
        Ok(output)
    }

    /**
     Read the number of pulses received.
    */
    fn read_recived_pulses(&mut self) -> Result<f64, MotorError> {
        let val: i32 = self.send_cmd(0x33, ())?;
        Ok(val as f64 / 200.)
    }

    /**recived_pulses
    * read the error of the motor shaft angle
    The error is the difference between the angle you want to control
    minus  the  real-time  angle  of  the  motor,  0~FFFF  corresponds  to
    0~360°.
    for  example,  when  the  angle  error  is  1°,  the  return  error  is
    65536/360= 182.444, and so on.
    */
    fn read_error(&mut self) -> Result<f64, MotorError> {
        let err: i16 = self.send_cmd(0x39, ())?;
        Ok(err as f64 / 65536.)
    }

    /**
    read the En pins status
    */
    fn read_en_pin(&mut self) -> Result<bool, MotorError> {
        let ret: u8 = self.send_cmd(0x3A, ())?;
        /*
        enable =1  Enabled
        enable =2 Disabled
         */
        Ok(ret == 1)
    }

    /**
     Release the motor shaft locked-rotor protection state
    status =1 release success.
    status =0 release fail
     */
    fn release_lock(&mut self) -> Result<(), MotorError> {
        let ret: u8 = self.send_cmd(0x3D, ())?;
        if ret == 1 {
            Ok(())
        } else {
            Err(NegativeResponse)
        }
    }

    /**
    Read the motor shaft protection state.
    status =1  protected.
    status =2  no protected
     */
    fn read_lock(&mut self) -> Result<Protection, MotorError> {
        let t: u8 = self.send_cmd(0x3E, ())?;
        Ok(if t == 1 {
            Protection::Protected
        } else {
            Protection::UnProtected
        })
    }

    /**
    Calibrate the encoder
    （Same as the "Cal" option on screen）
    - status =1  Calibrated success.
    - status =2  Calibrating fail.
    Note : The motor must be unloaded.
    */
    fn calibrate(&mut self) -> Result<(), MotorError> {
        let ret: u8 = self.send_cmd(0x80, 0x00u8)?;
        if ret == 1 {
            Ok(())
        } else {
            Err(NegativeResponse)
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
    fn set_mot_type(&mut self, mot_type: MotType) -> Result<(), MotorError> {
        let to_send: u8 = match mot_type {
            MotType::Deg0_9 => 0,
            MotType::Deg1_8 => 1,
        };
        let ret: u8 = self.send_cmd(0x81, to_send)?;
        if ret == 1 {
            Ok(())
        } else {
            Err(NegativeResponse)
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
    fn set_mode(&mut self, work_mode: WorkMode) -> Result<(), MotorError> {
        let to_send: u8 = match work_mode {
            WorkMode::CrOpen => 0,
            WorkMode::CrVFoc => 1,
            WorkMode::CrUART => 2,
        };
        let ret: u8 = self.send_cmd(0x82, to_send)?;
        if ret == 1 {
            Ok(())
        } else {
            Err(NegativeResponse)
        }
    }

    /**
     Set the current
     Same as the "Ma" option on screen
     The current = ma x 200 mA
     status =1  Set success.
     status =0  Set fail.
    */
    fn set_current(&mut self, t: u8) -> Result<(), MotorError> {
        let ret: u8 = self.send_cmd(0x83, t)?;
        if ret == 1 {
            Ok(())
        } else {
            Err(NegativeResponse)
        }
    }

    /**
     * Supports subdivision from 1 to 256.
    (Default: 16)
    Set microstep
    Note:the new micstep will show in the screen of MStep option.
    status =1  Set success.
    status =0  Set fail.
    */
    fn set_microstep(&mut self, mstep: u8) -> Result<(), MotorError> {
        let ret: u8 = self.send_cmd(0x84, mstep)?;
        if ret == 1 {
            self.microstep = mstep;
            Ok(())
        } else {
            Err(NegativeResponse)
        }
    }

    /**
    Set the active of the En pin
    （Same as the "En" option on screen）
    - enable = 00   active low  (L)
    - enable = 01   active high   (H)
    - enable = 02   active always (Hold)
    */
    fn set_en_active(&mut self, active_on: ActiveOn) -> Result<(), MotorError> {
        let to_send: u8 = match active_on {
            ActiveOn::Low => 0,
            ActiveOn::High => 1,
            ActiveOn::Hold => 2,
        };
        let ret: u8 = self.send_cmd(0x85, to_send)?;
        if ret == 1 {
            Ok(())
        } else {
            Err(NegativeResponse)
        }
    }

    /**
    Set the direction of motor rotation
    Same as the "Dir" option on screen
    - dir = 00   CW
    - dir = 01   CCW
    */
    fn set_direction(&mut self, dir: Dir) -> Result<(), MotorError> {
        let to_send: u8 = match dir {
            Dir::ClockWise => 0,
            Dir::CounterClockWise => 1,
        };
        let ret: u8 = self.send_cmd(0x86, to_send)?;
        if ret == 1 {
            Ok(())
        } else {
            Err(NegativeResponse)
        }
    }

    /**
    Set automatic turn off the screen
    Same as the "AutoSDD" option on screen
    */
    fn set_autossd(&mut self, active: bool) -> Result<(), MotorError> {
        let mut t: u8 = 0;
        if active {
            t += 1;
        }
        let ret: u8 = self.send_cmd(0x87, t)?;
        if ret == 1 {
            Ok(())
        } else {
            Err(NegativeResponse)
        }
    }

    /**
    Set the motor shaft locked-rotor protection function
    Same as the "Protect" option on screen
    */
    fn set_lock(&mut self, protection: Protection) -> Result<(), MotorError> {
        let to_send: u8 = match protection {
            Protection::Protected => 0,
            Protection::UnProtected => 1,
        };
        let ret: u8 = self.send_cmd(0x88, to_send)?;
        if ret == 1 {
            Ok(())
        } else {
            Err(NegativeResponse)
        }
    }

    /**
     Set the subdivision interpolation function
    （Same as the "Mplyer" option on screen）
    enabled interpolation function.
     */
    fn set_subdivision_interpolation(&mut self, active: bool) -> Result<(), MotorError> {
        let mut t: u8 = 0;
        if active {
            t += 1;
        }
        let ret: u8 = self.send_cmd(0x89, t)?;
        if ret == 1 {
            Ok(())
        } else {
            Err(NegativeResponse)
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
    fn set_baudrate(&mut self, baud_rate: BaudRate) -> Result<(), MotorError> {
        let to_send: u8 = match baud_rate {
            BaudRate::B9600 => 0,
            BaudRate::B19200 => 1,
            BaudRate::B25000 => 2,
            BaudRate::B38400 => 3,
            BaudRate::B57600 => 4,
            BaudRate::B115200 => 5,
        };
        let ret: u8 = self.send_cmd(0x8A, to_send)?;
        if ret == 1 {
            Ok(())
        } else {
            Err(NegativeResponse)
        }
    }

    /**
     Set the slave address
    （Same as the "UautAddr" option on screen）
     Slave address = addr + 0xe0
     addr from 0-9
     */
    fn set_slave_address(&mut self, addr: u8) -> Result<(), MotorError> {
        //TODO enum?
        let ret: u8 = self.send_cmd(0x8B, addr)?;
        if ret == 1 {
            Ok(())
        } else {
            Err(NegativeResponse)
        }
    }

    /**
     Restore the default parameter
    （Same as the "Restore" option on screen）
    NOTE: after resetting it must be rebooted, and the serial comunication baudrate must be manually set
     */
    fn reset(&mut self) -> Result<(), MotorError> {
        let ret: u8 = self.send_cmd(0x3F, ())?;
        if ret == 1 {
            Ok(())
        } else {
            Err(NegativeResponse)
        }
    }

    /**
     Set the mode of zeroMode
    （Same as the " 0_Mode " option on screen）
     - mode = 00  Disable
     - mode = 01  DirMode
     - mode = 02  NearMode
     */
    fn set_zero_mode(&mut self, mode: u8) -> Result<(), MotorError> {
        let ret: u8 = self.send_cmd(0x90, mode)?;
        if ret == 1 {
            Ok(())
        } else {
            Err(NegativeResponse)
        }
    }

    /**
     Set zero of zeroMode
    （Same as the " set 0 " option on screen）
    NOTE: The mode of "0_Mode" needs to be set first.
     */
    fn set_zero(&mut self) -> Result<(), MotorError> {
        let ret: u8 = self.send_cmd(0x91, 0u8)?;
        if ret == 1 {
            Ok(())
        } else {
            Err(NegativeResponse)
        }
    }

    /**
     Set zero speed
    （Same as the " 0_Speed" option on screen）
    NOTE: The mode of "0_Mode" needs to be set first.
    (speed = 0~4, the smaller the value, the faster the speed)
     */
    fn set_zero_speed(&mut self, speed: u8) -> Result<(), MotorError> {
        let ret: u8 = self.send_cmd(0x92, speed)?;
        if ret == 1 {
            Ok(())
        } else {
            Err(NegativeResponse)
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
    fn set_zero_dir(&mut self, dir: u8) -> Result<(), MotorError> {
        let ret: u8 = self.send_cmd(0x93, dir)?;
        if ret == 1 {
            Ok(())
        } else {
            Err(NegativeResponse)
        }
    }

    /**
     Set dir of zero mode
    （Same as the " Goto 0" option on screen）
     */
    fn goto_zero(&mut self) -> Result<(), MotorError> {
        let ret: u8 = self.send_cmd(0x94, 0u8)?;
        if ret == 1 {
            Ok(())
        } else {
            Err(NegativeResponse)
        }
    }

    /**
    Set the position Kp parameter
    */
    fn set_kp(&mut self, kp: u16) -> Result<(), MotorError> {
        self.kp = kp;
        let ret: u8 = self.send_cmd(0xA1, kp)?;
        if ret == 1 {
            Ok(())
        } else {
            Err(NegativeResponse)
        }
    }
    /**
    Set the position Ki parameter
    */
    fn set_ki(&mut self, ki: u16) -> Result<(), MotorError> {
        self.ki = ki;
        let ret: u8 = self.send_cmd(0xA2, self.ki)?;
        if ret == 1 {
            Ok(())
        } else {
            Err(NegativeResponse)
        }
    }
    /**
    Set the position Ki parameter
    */
    fn set_kd(&mut self, kd: u16) -> Result<(), MotorError> {
        self.kd = kd;
        let ret: u8 = self.send_cmd(0xA3, kd)?;
        if ret == 1 {
            Ok(())
        } else {
            Err(NegativeResponse)
        }
    }

    /**
    Set the acceleration (ACC) parameter
    for unknown reasons it resets the position on the motor...
    */
    fn set_acc(&mut self, acc: u16) -> Result<(), MotorError> {
        self.acc = acc;
        let ret: u8 = self.send_cmd(0xA4, self.acc)?;
        if ret == 1 {
            Ok(())
        } else {
            Err(NegativeResponse)
        }
    }

    /**
    Set the maximum torque (MaxT) parameter
    */
    fn set_maxt(&mut self, kp: Option<u16>) -> Result<(), MotorError> {
        let ret: u8 = self.send_cmd(0xA5, kp.unwrap_or(0x4B0))?;
        if ret == 1 {
            Ok(())
        } else {
            Err(MotorError::NegativeResponse)
        }
    }

    /**
    Set the En pin status in CR_UART mode.
    */
    fn set_enable(&mut self, en: bool) -> Result<(), MotorError> {
        let mut b = 0u8;
        if en {
            b += 1;
        }
        let ret: u8 = self.send_cmd(0xF3, b)?;
        if ret == 1 {
            Ok(())
        } else {
            Err(NegativeResponse)
        }
    }

    /**
    Set Speed
    run the motor forward / reverse in a Constant speed.
    Direction : The highest 1bit of VAL.
    Speed : The lowest 7bit of VAL.
    for example：
    The Vrpm calculation formula is:
    Vrpm = (Speed × 30000)/(Mstep × 200)(RPM)   (1.8 degree motor)
    Vrpm = (Speed × 30000)/(Mstep × 400)(RPM)   (0.9 degree motor)
    Note: the Vrpm no great than 2000RPM.
     */
    fn set_speed(&mut self, speed: i8) -> Result<u8, MotorError> {
        if speed as f32 * 30000. / (self.microstep as f32 * 200.) > 2000. {
            return Err(NegativeResponse);
        }
        let to_send = if speed < 0 {
            -speed as u8
        } else {
            speed as u8 | 0x80
        };
        let ret: u8 = self.send_cmd(0xF6, to_send)?;
        Ok(ret)
    }

    /**
    Stop motor
    */
    fn stop(&mut self) -> Result<u8, MotorError> {
        self.send_cmd(0xF7, ())
    }
    /**
    DO NOT USE THIS FUNCTION, IT'S BLOCKING!!
    */
    fn goto(&mut self, speed: u8, dist: u32) -> u8 {
        let ret: u8 = self.send_cmd(0xFD, (speed, dist)).unwrap();
        //let stopped: u8= self.read().unwrap();
        //println!("WTF, received {}", stopped);
        ret
    }
    
}

#[cfg(test)]
#[cfg(feature = "std")]
mod tests {
    use super::*;

    use serial::test::SerialTest;
    macro_rules! test_motor {
        ($name:ident ($($arg:expr),*)->$res:expr, ($($val:literal) *)->($($ret:literal) *)) => {
            #[test]
            fn $name(){
                let mut servo: Servo42C<SerialTest>=Servo42C::empty_new(SerialTest::default());
                servo.s.add_response(vec![$($val),*], vec![$($ret),*]);

                assert_eq!(servo.$name($($arg),*).unwrap(), $res);
            }
        };
    }

    test_motor!(read_encoder_value()->0.25, (0xe0 0x30 0x10)->(0xe0 00 00 00 00 0x40 00 0x20));
    test_motor!(read_recived_pulses()->1.28, (0xe0 0x33 0x13)->(0xe0 00 00 0x01 00 0xe1));
    test_motor!(read_error()->183./65536., (0xe0 0x39 0x19)->(0xe0 00 0xB7 0x97));
    test_motor!(read_en_pin()->true, (0xe0 0x3a 0x1a)->(0xe0 0x01 0xe1));
    test_motor!(release_lock()->(), (0xe0 0x3d 0x1d)->(0xe0 0x01 0xe1));

    test_motor!(read_lock()->Protection::UnProtected, (0xe0 0x3e 0x1e)->(0xe0 0x02 0xe2));
    test_motor!(calibrate()->(), (0xe0 0x80 0x00 0x60)->(0xe0 0x01 0xe1));
    test_motor!(set_mot_type(MotType::Deg1_8)->(), (0xe0 0x81 0x01 0x62)->(0xe0 0x01 0xe1));
    test_motor!(set_mode(WorkMode::CrVFoc)->(), (0xe0 0x82 0x01 0x63)->(0xe0 0x01 0xe1));
    test_motor!(set_current(6)->(), (0xe0 0x83 0x06 0x69)->(0xe0 0x01 0xe1));
    test_motor!(set_microstep(26)->(), (0xe0 0x84 0x1a 0x7e)->(0xe0 0x01 0xe1));
    test_motor!(set_en_active(ActiveOn::Low)->(), (0xe0 0x85 0x00 0x65)->(0xe0 0x01 0xe1));
    test_motor!(set_direction(Dir::ClockWise)->(), (0xe0 0x86 0x00 0x66)->(0xe0 0x01 0xe1));
    test_motor!(set_autossd(false)->(), (0xe0 0x87 0x00 0x67)->(0xe0 0x01 0xe1));
    test_motor!(set_lock(Protection::Protected)->(), (0xe0 0x88 0x00 0x68)->(0xe0 0x01 0xe1));
    test_motor!(set_subdivision_interpolation(false)->(), (0xe0 0x89 0x00 0x69)->(0xe0 0x01 0xe1));
    test_motor!(set_baudrate(BaudRate::B57600)->(), (0xe0 0x8A 0x04 0x6e)->(0xe0 0x01 0xe1));
    test_motor!(set_slave_address(2)->(), (0xe0 0x8B 0x02 0x6d)->(0xe0 0x01 0xe1));
    test_motor!(reset()->(), (0xe0 0x3f 0x1f)->(0xe0 0x01 0xe1));

    test_motor!(set_zero_mode(1)->(), (0xe0 0x90 0x01 0x71)->(0xe0 0x01 0xe1));
    test_motor!(set_zero()->(), (0xe0 0x91 0x00 0x71)->(0xe0 0x01 0xe1));
    test_motor!(set_zero_speed(2)->(), (0xe0 0x92 0x02 0x74)->(0xe0 0x01 0xe1));
    test_motor!(set_zero_dir(0)->(), (0xe0 0x93 0x00 0x73)->(0xe0 0x01 0xe1));

    test_motor!(set_kp(0x120)->(), (0xe0 0xA1 0x01 0x20 0xA2)->(0xe0 0x01 0xe1));
    test_motor!(set_ki(0x02)->(), (0xe0 0xA2 0x00 0x02 0x84)->(0xe0 0x01 0xe1));
    test_motor!(set_kd(0x250)->(), (0xe0 0xA3 0x02 0x50 0xD5)->(0xe0 0x01 0xe1));
    test_motor!(set_acc(0x80)->(), (0xe0 0xA4 0x00 0x80 0x04)->(0xe0 0x01 0xe1));
    test_motor!(set_maxt(Some(0x258))->(), (0xe0 0xA5 0x02 0x58 0xDF)->(0xe0 0x01 0xe1));

    test_motor!(set_enable(true)->(), (0xe0 0xf3 0x01 0xd4)->(0xe0 0x01 0xe1));
    test_motor!(goto_zero()->(), (0xe0 0x94 0x00 0x74)->(0xe0 0x01 0xe1));
}
