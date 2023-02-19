use rppal;
use rppal::gpio::OutputPin;

use ascii_converter::string_to_binary;
use std::thread::sleep;
use std::time::Duration;

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
    pub fn new() -> Self {
        let gpio = rppal::gpio::Gpio::new().expect("death");
        Self {
            d0: gpio.get(26).expect("couldn't get pin").into_output_low(),
            d1: gpio.get(19).expect("couldn't get pin").into_output_low(),
            d2: gpio.get(13).expect("couldn't get pin").into_output_low(),
            d3: gpio.get(6).expect("couldn't get pin").into_output_low(),
            d4: gpio.get(5).expect("couldn't get pin").into_output_low(),
            d5: gpio.get(0).expect("couldn't get pin").into_output_low(),
            d6: gpio.get(11).expect("couldn't get pin").into_output_low(),
            d7: gpio.get(9).expect("couldn't get pin").into_output_low(),
            rs: gpio.get(21).expect("couldn't get pin").into_output_low(),
            en: gpio.get(20).expect("couldn't get pin").into_output_low(),
        }
    }
}
pub fn mv_cursor(pins: &mut Pins, direction: &str, line: &str) {
    pins.rs.set_low();
    match direction {
        "next" => {
            bwrite(pins, "00010100");
        }
        "prev" => {
            bwrite(pins, "00010000");
        }
        _ => {
            println!("Invalid options");
        }
    }
}

pub fn settings(pins: &mut Pins, cursor: &str, screen: bool) {
    pins.rs.set_low();

    match cursor {
        "off" => {
            bwrite(pins, "00001100");
        }
        "on" => {
            bwrite(pins, "00001110");
        }
        "blink" => {
            bwrite(pins, "00001111");
        }
        _ => {
            println!("Invalid cursor option: '{}'", screen);
            println!("Valid cursor options: 'on', 'blink' and 'off' ");
        }
    }

    if !screen {
        bwrite(pins, "00001000");
    }
}

pub fn begin(pins: &mut Pins) {
    pins.rs.set_low();

    bwrite(pins, "00001010");
    clear(pins);
    bwrite(pins, "00000010");
    bwrite(pins, "00001100");

    println!("finished setting up lcd <begin(&mut pins);> ");
}

pub fn clear(pins: &mut Pins) {
    pins.rs.set_low();
    bwrite(pins, "00000001");
}

pub fn pulse(pins: &mut Pins) {
    pins.en.set_high();
    sleep(Duration::from_nanos(340));
    pins.en.set_low();
}

pub fn write(pins: &mut Pins, text: &str) {
    pins.rs.set_high();
    let binary_text = string_to_binary(text).unwrap();
    for c in binary_text {
        if c.to_string().len() < 7 {
            bwrite(pins, format!("00{}", c).as_str());
        } else if c.to_string().len() < 8 {
            bwrite(pins, format!("0{}", c).as_str());
        }
    }
}

pub fn bwrite(pins: &mut Pins, bits: &str) {
    println!("output: {}", bits);
    if bits.chars().nth(7).unwrap() == '1' {
        pins.d0.set_high();
    } else {
        pins.d0.set_low();
    }

    if bits.chars().nth(6).unwrap() == '1' {
        pins.d1.set_high();
    } else {
        pins.d1.set_low();
    }
    if bits.chars().nth(5).unwrap() == '1' {
        pins.d2.set_high();
    } else {
        pins.d2.set_low();
    }
    if bits.chars().nth(4).unwrap() == '1' {
        pins.d3.set_high();
    } else {
        pins.d3.set_low();
    }
    if bits.chars().nth(3).unwrap() == '1' {
        pins.d4.set_high();
    } else {
        pins.d4.set_low();
    }
    if bits.chars().nth(2).unwrap() == '1' {
        pins.d5.set_high();
    } else {
        pins.d5.set_low();
    }
    if bits.chars().nth(1).unwrap() == '1' {
        pins.d6.set_high();
    } else {
        pins.d6.set_low();
    }
    if bits.chars().nth(0).unwrap() == '1' {
        pins.d7.set_high();
    } else {
        pins.d7.set_low();
    }
    sleep(Duration::from_millis(30));
    pulse(pins);
}
