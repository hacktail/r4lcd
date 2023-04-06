use crate::*;

#[repr(u8)]
pub enum PowerMode {
    Off = 0,
    On = 1,

}

// We can't always use bools, because sometimes I might need an "in-the-middle" option. So I made this one enum to controll multiple choice stuff
#[repr(u8)]
pub enum CursorMode {
    Off = 0,
    On = 1,
    Blink = 2,
}
static CONFIGURATION_LUT: [u8; 6] = [
    0b0000_1000, // Power off, cursor off
    0b0000_1100, // Power on, cursor off
    0b0000_1010, // Power off, cursor on
    0b0000_1110, // Power on, cursor on
    0b0000_1011, // Power off, cursor blink
    0b0000_1111, // Power on, cursor blink
    ];



// setts Cursor mode and PowerMode for the lcd
pub fn settings(pins: &mut Pins, cursor_mode: CursorMode, power_mode: PowerMode) {
    pins.rs.set_low();

    bwrite(
        pins,
        CONFIGURATION_LUT[2*(cursor_mode as usize) + (power_mode as usize)]
            )
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
