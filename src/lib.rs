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
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CursorModes {
    On,
    Blink,
    Off,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Settings {
    Cursor(CursorModes),
    Power(bool),
    DisplayLines(i8),
}

pub static mut CURSOR_POSITION: (i16, i16) = (0, 0); // (x,y)
static mut SETTINGS: (Settings, Settings, Settings) = (
    Settings::Cursor(CursorModes::Off),
    Settings::Power(false),
    Settings::DisplayLines(1),
);

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

pub fn home(pins: &mut Pins) {
    pins.rs.set_low();
    bwrite(pins, "00000010");
    sleep(Duration::from_millis(2));
}

pub fn mvc(pins: &mut Pins, x: i16, y: i16) {
    if x < 0 || y < 0 {
        panic!("the coordinates are lower then 0: x={}, y={}", x, y);
    } else {
        unsafe {
            // moving x
            pins.rs.set_low();

            while CURSOR_POSITION.0 < x {
                bwrite(pins, "00010100");
                CURSOR_POSITION.0 += 1;
                println!(
                    "cursor position = ({},{})",
                    CURSOR_POSITION.0, CURSOR_POSITION.1
                );
            }
            while CURSOR_POSITION.0 > x {
                bwrite(pins, "00010000");
                CURSOR_POSITION.0 -= 1;
                println!(
                    "cursor position = ({},{})",
                    CURSOR_POSITION.0, CURSOR_POSITION.1
                );
            }
            // moving y
            if SETTINGS.2 != Settings::DisplayLines(1) {
                while CURSOR_POSITION.1 < y {
                    bwrite(pins, "000111100");
                    CURSOR_POSITION.1 += 1;
                    println!(
                        "cursor position = ({},{})",
                        CURSOR_POSITION.0, CURSOR_POSITION.1
                    );
                }
                while CURSOR_POSITION.1 > y {
                    bwrite(pins, "00011000");
                    CURSOR_POSITION.1 -= 1;
                    println!(
                        "cursor position = ({},{})",
                        CURSOR_POSITION.0, CURSOR_POSITION.1
                    );
                }
            }
        }
    }
}

pub fn settings(pins: &mut Pins, cursor: CursorModes, screen: Settings) {
    pins.rs.set_low();

    if screen == Settings::Power(true) {
        match cursor {
            CursorModes::On => bwrite(pins, "00001110"),
            CursorModes::Blink => bwrite(pins, "00001111"),
            CursorModes::Off => bwrite(pins, "00001100"),
        }
    } else if screen == Settings::Power(false) {
        match cursor {
            CursorModes::On => bwrite(pins, "00001010"),
            CursorModes::Blink => bwrite(pins, "00001011"),
            CursorModes::Off => bwrite(pins, "00001000"),
        }
    } else {
        println!("'{screen:?}' is an invalid option")
    }
    unsafe {
        SETTINGS.0 = Settings::Cursor(cursor);
        SETTINGS.1 = screen;
    };
}

pub fn begin(pins: &mut Pins, display_lines: i8 /*, bits: i8 */) {
    pins.rs.set_low();

    Settings::DisplayLines(display_lines);
    match display_lines {
        1 => {
            bwrite(pins, "00001000");
        }
        2 => {
            bwrite(pins, "00001010");
        }
        _ => {
            panic!("'{}' is an invalid amount of display_lines", display_lines);
        }
    }
    clear(pins);
    home(pins);
    settings(pins, CursorModes::Off, Settings::Power(true));
    println!("finished setting up lcd");
}

pub fn clear(pins: &mut Pins) {
    pins.rs.set_low();
    bwrite(pins, "00000001");
}

fn pulse(pins: &mut Pins) {
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
    if pins.rs.is_set_high() {
        unsafe {
            CURSOR_POSITION.0 += 1;
        }
    }
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
