use stm32f4::gpio;

pub const LD3: Led = Led {
    gpio: gpio::GPIO_D,
    pin: 13,
};
pub const LD4: Led = Led {
    gpio: gpio::GPIO_D,
    pin: 12,
};
pub const LD5: Led = Led {
    gpio: gpio::GPIO_D,
    pin: 14,
};
pub const LD6: Led = Led {
    gpio: gpio::GPIO_D,
    pin: 15,
};

pub struct Led {
    gpio: gpio::Gpio,
    pin: u32,
}

impl Led {
    pub fn init(&self) {
        self.gpio.enable(self.pin, gpio::GpioConfig {
            mode: gpio::GpioMode::OUTPUT,
            ospeed: gpio::GpioOSpeed::LOW_SPEED,
            otype: gpio::GpioOType::PUSH_PULL,
            pupd: gpio::GpioPuPd::NO,
            af: gpio::GpioAF::AF0, // not used
        });
    }

    pub fn turn_on(&self) {
        self.gpio.set_bit(self.pin);
    }

    #[allow(dead_code)]
    pub fn turn_off(&self) {
        self.gpio.clear_bit(self.pin);
    }
}