pub use std::io::Read;

use super::serialtrait::*;
pub use serialport;
pub use serialport::*;

impl Serial for Box<dyn SerialPort + 'static> {
    fn read(&mut self, buf: &mut [u8]) -> std::result::Result<(), SerialError> {
        let mut readen = 0usize;
        let mut failed = 0usize;
        loop {
            if let Ok(x) = std::io::Read::read(self, &mut buf[readen..]) {
                readen += x;
            } else {
                failed += 1;
            }
            #[cfg(feature = "debug")]
            println!("readen {buf:?} {readen}/{}", buf.len());
            if failed > 10 {
                return Err(SerialError::ConnectionBreak);
            }
            if readen >= buf.len() {
                break;
            }
        }
        Ok(())
    }

    fn write(&mut self, buf: &[u8]) -> std::result::Result<(), SerialError> {
        self.write_all(buf)
            .map_err(|_| SerialError::ConnectionBreak)?;
        #[cfg(feature = "debug")]
        println!("written {buf:?}");
        Ok(())
    }
}
