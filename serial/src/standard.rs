pub use std::io::Read;

use super::serialtrait::*;
pub use serialport;
pub use serialport::*;

impl Serial for Box<dyn SerialPort + 'static> {
    fn read(&mut self, buf: &mut [u8]) -> std::result::Result<(), SerialError> {
        let mut y = 0usize;
        let mut failed=0usize;
        loop {
            if let Ok(x) = std::io::Read::read(self, &mut buf[y..]) {
                y += x;
            }else{
                failed+=1;
            }
            println!("readen {buf:?} {y}/{}", buf.len());
            if failed>10 {
                return Err(SerialError::Undefined);
            }
            if y >= buf.len() {
                break;
            }
        }
        //self.read_to_end(buf).map_err(|_|SerialError::Undefined)?;
        //self.read(buf).map_err(|_|SerialError::Undefined)?;
        //println!("readen {buf:?}");
        Ok(())
    }

    fn write(&mut self, buf: &[u8]) -> std::result::Result<(), SerialError> {
        self.write_all(buf).map_err(|_| SerialError::Undefined)?;
        println!("written {buf:?}");
        Ok(())
    }
}
