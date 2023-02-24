use rppal;
use rppal::gpio::OutputPin;
use rppal::gpio::Level;

use ascii_converter::string_to_decimals;
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
pub enum Options {
    On,
    Off,
    Blink,
}
static mut CURSOR_POSITION: (u8, u8) = (0, 0); // (x,y)


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
    bwrite(pins, 0b00000010);
    sleep(Duration::from_millis(2));
}

pub fn mvc(pins: &mut Pins, mut x: u8, y: u8)
{
    unsafe{CURSOR_POSITION = (x,y);};

    pins.rs.set_low();
    if y == 1 {
        x+=64;
    }
    if x > 128 {panic!("to big number");};
x+=128;
    println!("{x}");


    bwrite(pins, x);

}




pub fn settings(pins: &mut Pins, cursor_mode: Options, power: Options) {
    pins.rs.set_low();

    if power == Options::On {
        match cursor_mode {
            Options::On => bwrite(pins, 0b00001110),
            Options::Blink => bwrite(pins, 0b00001111),
            Options::Off => bwrite(pins, 0b00001100),
        }
    } else if power == Options::Off
    {
        match cursor_mode {
            Options::On => bwrite(pins, 0b00001010),
            Options::Blink => bwrite(pins, 0b00001011),
            Options::Off => bwrite(pins, 0b00001000),
        }
    } else {
        println!("'{power:?}' is an invalid option")
    }
}

pub fn begin(pins: &mut Pins, display_lines: i8 /*, bits: i8 */) {
    pins.rs.set_low();



    match display_lines {
        1 => {
            bwrite(pins, 0b00110000);
        }
        2 => {
            bwrite(pins, 0b00111000);
        }
        _ => {
            panic!("'{}' is an invalid amount of display_lines", display_lines);
        }
    }
    clear(pins);
    home(pins);
    settings(pins, Options::Off, Options::On);
    println!("finished setting up lcd. Use 'lcd::Settings();' to change screen settings");
}

pub fn clear(pins: &mut Pins) {
    pins.rs.set_low();
    bwrite(pins, 0b00000001);
}

pub fn pulse(pins: &mut Pins) {
    pins.en.set_high();
    sleep(Duration::from_nanos(340));
    pins.en.set_low();
}

pub fn write(pins: &mut Pins, text: &str)
{
    pins.rs.set_high();
    let binary_text = string_to_decimals(text).unwrap();

    for bits in binary_text{
        println!("writing {:b}", bits);
        bwrite(pins, bits);
    }
}

pub fn bwrite(pins: &mut Pins, bits: u8)
{

    pins.d0.write(Level::from((bits & PIN_FLAGS[0]) as u8));
    pins.d1.write(Level::from((bits & PIN_FLAGS[1]) as u8));
    pins.d2.write(Level::from((bits & PIN_FLAGS[2]) as u8));
    pins.d3.write(Level::from((bits & PIN_FLAGS[3]) as u8));
    pins.d4.write(Level::from((bits & PIN_FLAGS[4]) as u8));
    pins.d5.write(Level::from((bits & PIN_FLAGS[5]) as u8));
    pins.d6.write(Level::from((bits & PIN_FLAGS[6]) as u8));
    pins.d7.write(Level::from((bits & PIN_FLAGS[7]) as u8));


        println!("b{:b}",bits | 0b10000000);

    sleep(Duration::from_millis(40));
    pulse(pins);
}



