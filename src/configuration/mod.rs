use crate::*;

// This is for choosing, we can't always use bools, because sometimes I might need an "in-the-middle" option. So I made this one enum to controll multiple choice stuff
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Options {
    On,
    Off,
    Blink,
}

// setts different settings for the lcd
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

// sets up the lcd with some basic things
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

