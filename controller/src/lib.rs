use serialport;

pub struct HavenCntrlr {
    serial_port: Box<dyn serialport::SerialPort>, 
    #[allow(dead_code)]
    degree: u8,
    state: State,
}

#[derive(PartialEq, Debug)]
pub enum State {
    Locked,
    Unlocked,
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
            state: State::Unlocked,
        }

    }

    pub fn is_locked(&self) -> bool {
        self.state == State::Locked
    }

    pub fn lock(&mut self) -> Result<(), serialport::Error> {
        if self.is_locked() {
            return Ok(())
        }
        self.serial_port.write(&[90])?;
        self.state = State::Locked;
        Ok(())
    }

    pub fn unlock(&mut self) -> Result<(), serialport::Error> {
        if !self.is_locked() {
            return Ok(())
        }
        self.serial_port.write(&[0])?;
        self.state = State::Unlocked;
        Ok(())
    }
    
}