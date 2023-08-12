use serialport;

pub struct HavenCntrlr {
    serial_port: Box<dyn serialport::SerialPort>, 
    degree: u8,
}

impl HavenCntrlr {
    pub fn new() -> Self {
        let serial_port = serialport::new("/dev/ttyUSB0", 9600)
            .timeout(std::time::Duration::from_millis(10))
            .open()
            .expect("Failed to open serial port");

        HavenCntrlr {
            serial_port,
            degree: 0,
        }

    }

    pub fn is_locked(self) -> bool {
        self.degree == 90
    }

    pub fn lock(mut self) -> Result<(), serialport::Error> {
        self.serial_port.write(&[90])?;
        Ok(())
    }

    pub fn unlock(mut self) -> Result<(), serialport::Error> {
        self.serial_port.write(&[0])?;
        Ok(())
    }
}