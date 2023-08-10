use serialport;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub struct HavenCntrlr {
    serial_port: Box<dyn serialport::SerialPort>, 
}

impl HavenCntrlr {
    pub fn new() {
        let serial_port = serialport::new("/dev/ttyUSB0", 9600)
            .timeout(std::time::Duration::from_millis(10))
            .open()
            .expect("Failed to open serial port");

        let ret = HavenCntrlr {
            serial_port,
        };

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
