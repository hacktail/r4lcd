use crate::*;
pub(crate) static PIN_FLAGS: [u8; 8] = [0b00000001,0b00000010,0b00000100,0b00001000,0b00010000,0b00100000,0b01000000,0b10000000];

/// A struct which defines what pins you want to
pub struct DefinePins{
    pub d0: u8,
    pub d1: u8,
    pub d2: u8,
    pub d3: u8,
    pub d4: u8,
    pub d5: u8,
    pub d6: u8,
    pub d7: u8,
    pub rs: u8,
    pub en: u8,
}

/// This struct is used to specify which Pins you want a function to use. This way you can also use multiple lcds
pub struct Pins {
    pub d0: OutputPin,
    pub d1: OutputPin,
    pub d2: OutputPin,
    pub d3: OutputPin,
    pub d4: OutputPin,
    pub d5: OutputPin,
    pub d6: OutputPin,
    pub d7: OutputPin,
    pub rs: OutputPin,
    pub en: OutputPin,
}
impl Pins {
    /// returns a struct with all the pins use to control the lcd
    /// you can toggle the pins on or off manually, but do not do that unless you really really know what you're doing
    pub fn new(udf: DefinePins) -> Self {
        let gpio = rppal::gpio::Gpio::new().expect("death");
        Self {
            d0: gpio.get(udf.d0).expect("couldn't get pin").into_output_low(),
            d1: gpio.get(udf.d1).expect("couldn't get pin").into_output_low(),
            d2: gpio.get(udf.d2).expect("couldn't get pin").into_output_low(),
            d3: gpio.get(udf.d3).expect("couldn't get pin").into_output_low(),
            d4: gpio.get(udf.d4).expect("couldn't get pin").into_output_low(),
            d5: gpio.get(udf.d5).expect("couldn't get pin").into_output_low(),
            d6: gpio.get(udf.d6).expect("couldn't get pin").into_output_low(),
            d7: gpio.get(udf.d7).expect("couldn't get pin").into_output_low(),
            rs: gpio.get(udf.rs).expect("couldn't get pin").into_output_low(),
            en: gpio.get(udf.en).expect("couldn't get pin").into_output_low(),
        }
    }
}