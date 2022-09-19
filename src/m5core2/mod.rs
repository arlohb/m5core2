use embedded_hal::prelude::*;
use esp_idf_hal::{gpio, i2c};

pub struct M5<
    I2C: i2c::I2c,
    SDA: gpio::OutputPin + gpio::InputPin,
    SCL: gpio::OutputPin + gpio::InputPin,
> {
    i2c: i2c::Master<I2C, SDA, SCL>,
}

impl Default for M5<i2c::I2C0, gpio::Gpio21<gpio::Unknown>, gpio::Gpio22<gpio::Unknown>> {
    fn default() -> Self {
        Self {
            i2c: esp_idf_hal::i2c::Master::new(
                unsafe { i2c::I2C0::new() },
                i2c::MasterPins {
                    sda: unsafe { gpio::Gpio21::<gpio::Unknown>::new() },
                    scl: unsafe { gpio::Gpio22::<gpio::Unknown>::new() },
                },
                i2c::config::MasterConfig {
                    baudrate: esp_idf_hal::units::Hertz(1_000_000),
                    timeout: Some(std::time::Duration::from_millis(100)),
                    sda_pullup_enabled: true,
                    scl_pullup_enabled: true,
                },
            )
            .unwrap(),
        }
    }
}

impl<
        I2C: i2c::I2c,
        SDA: gpio::OutputPin + gpio::InputPin,
        SCL: gpio::OutputPin + gpio::InputPin,
    > M5<I2C, SDA, SCL>
{
    pub fn set_vibration(&mut self, state: bool) {
        fn write_1_byte<
            I2C: i2c::I2c,
            SDA: gpio::OutputPin + gpio::InputPin,
            SCL: gpio::OutputPin + gpio::InputPin,
        >(
            i2c: &mut i2c::Master<I2C, SDA, SCL>,
            addr: u8,
            data: u8,
        ) {
            let _ = i2c
                .write(0x34, &[addr, data])
                .map_err(|e| eprintln!("Error: {:?}", e));
        }

        fn read_1_byte<
            I2C: i2c::I2c,
            SDA: gpio::OutputPin + gpio::InputPin,
            SCL: gpio::OutputPin + gpio::InputPin,
        >(
            i2c: &mut i2c::Master<I2C, SDA, SCL>,
            addr: u8,
        ) -> u8 {
            let mut buf = [0u8; 1];

            let _ = i2c
                .write_read(0x34, &[addr], &mut buf)
                .map_err(|e| eprintln!("Error: {:?}", e));

            buf[0]
        }

        if state {
            let data = read_1_byte(&mut self.i2c, 0x12) | 0b1000;
            write_1_byte(&mut self.i2c, 0x12, data);
        } else {
            let data = read_1_byte(&mut self.i2c, 0x12) & !0b1000;
            write_1_byte(&mut self.i2c, 0x12, data);
        }
    }
}
