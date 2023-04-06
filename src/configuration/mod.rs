use crate::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PowerMode {
    On,
    Off,
}

// We can't always use bools, because sometimes I might need an "in-the-middle" option. So I made this one enum to controll multiple choice stuff

pub enum CursorMode {
    On,
    Off,
    Blink
}

// setts Cursor mode and PowerMode for the lcd
pub fn settings(pins: &mut Pins, cursor_mode: CursorMode, power: PowerMode) {
    pins.rs.set_low();

    if power == PowerMode::On {
        match cursor_mode {
            CursorMode::On => bwrite(pins, 0b00001110),
            CursorMode::Blink => bwrite(pins, 0b00001111),
            CursorMode::Off => bwrite(pins, 0b00001100),
        }
    } else if power == PowerMode::Off {
        match cursor_mode {
            CursorMode::On => bwrite(pins, 0b00001010),
            CursorMode::Blink => bwrite(pins, 0b00001011),
            CursorMode::Off => bwrite(pins, 0b00001000),
        }
    } else {
        println!("'{power:?}' is an invalid option")
    }
}

// sets up the lcd with basic configuration
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
            panic!("Invalid amount of display lines {}. Highest amount of display lines currently supported is 2", display_lines);
        }
    }
    clear(pins);
    home(pins);
    settings(pins, CursorMode::Off, PowerMode::On);
    println!("finished setting up lcd. Use 'lcd::Settings();' to change screen settings");
}
