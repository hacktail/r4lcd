use rppal;
use rppal::gpio::OutputPin;
use rppal::gpio::Level;

use ascii_converter::*;
use std::thread::sleep;
use std::time::Duration;

static PIN_FLAGS: [u8; 8] = [0b00000001,0b00000010,0b00000100,0b00001000,0b00010000,0b00100000,0b01000000,0b10000000];

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
static mut CURSOR_POSITION: (u8, u8) = (0, 0); // (x,y)

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
pub fn cursor_position()-> (u8,u8){
    unsafe {CURSOR_POSITION}
}
pub fn home(pins: &mut Pins) {
    pins.rs.set_low();
    bwrite(pins, "00000010");
    sleep(Duration::from_millis(2));
}

pub fn mvc(pins: &mut Pins, mut x: u8, y: u8)
{
    unsafe{CURSOR_POSITION = (x,y);};

    pins.rs.set_low();
    if y == 1 {
        x+=64;
    }
    let x = decimals_to_binary(&vec![x]).unwrap();
    let mut x: String = x.iter().map(ToString::to_string).collect();
    println!("{x}");

    while x.len() < 7
            {
                x = format!("0{x}");
            }
    println!("{x}");
    x = format!("1{x}");
    bwrite(pins, x.as_str());

}




pub fn settings(pins: &mut Pins, cursor_mode: CursorModes, power: Settings) {
    pins.rs.set_low();

    if power == Settings::Power(true) {
        match cursor_mode {
            CursorModes::On => bwrite(pins, "00001110"),
            CursorModes::Blink => bwrite(pins, "00001111"),
            CursorModes::Off => bwrite(pins, "00001100"),
        }
    } else if power == Settings::Power(false) {
        match cursor_mode {
            CursorModes::On => bwrite(pins, "00001010"),
            CursorModes::Blink => bwrite(pins, "00001011"),
            CursorModes::Off => bwrite(pins, "00001000"),
        }
    } else {
        println!("'{power:?}' is an invalid option")
    }
    unsafe {
        SETTINGS.0 = Settings::Cursor(cursor_mode);
        SETTINGS.1 = power;
    };
}

pub fn begin(pins: &mut Pins, display_lines: i8 /*, bits: i8 */) {
    pins.rs.set_low();

    unsafe {SETTINGS.2 = Settings::DisplayLines(display_lines);} 

    match display_lines {
        1 => {
            bwrite(pins, "00110000");
        }
        2 => {
            bwrite(pins, "00111000");
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

pub fn write(pins: &mut Pins, text: &str)
{
    pins.rs.set_high();
    let binary_text = string_to_binary(text).unwrap();

    for bits in binary_text{
        println!("d{bits}");
        println!("dd{}", bits as u8);
        bbwrite(pins, bits as u8);
    }
}

/*pub fn write(pins: &mut Pins, text: &str) {
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
*/
pub fn bbwrite(pins: &mut Pins, bits: u8)
{

    pins.d0.write(Level::from((bits & PIN_FLAGS[0]) as u8));
    pins.d1.write(Level::from((bits & PIN_FLAGS[1]) as u8));
    pins.d2.write(Level::from((bits & PIN_FLAGS[2]) as u8));
    pins.d3.write(Level::from((bits & PIN_FLAGS[3]) as u8));
    pins.d4.write(Level::from((bits & PIN_FLAGS[4]) as u8));
    pins.d5.write(Level::from((bits & PIN_FLAGS[5]) as u8));
    pins.d6.write(Level::from((bits & PIN_FLAGS[6]) as u8));
    pins.d7.write(Level::from((bits & PIN_FLAGS[7]) as u8));

    println!("a{:b}", bits & PIN_FLAGS[0]);
        println!("a{:b}", bits & PIN_FLAGS[1]);
        println!("a{:b}", bits & PIN_FLAGS[2]);
        println!("a{:b}", bits & PIN_FLAGS[3]);
        println!("a{:b}", bits & PIN_FLAGS[4]);
        println!("a{:b}", bits & PIN_FLAGS[5]);
        println!("a{:b}", bits & PIN_FLAGS[6]);
        println!("a{:b}", bits & PIN_FLAGS[7]);
        println!("b{:b}",bits | 0b10000000);

    sleep(Duration::from_millis(30));
    pulse(pins);
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
