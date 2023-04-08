use crate::*;

// This look up table is used to make it easier to configure the display
static CONFIGURATION_LUT: [u8; 6] = [
    0b0000_1000, // Power off, cursor off
    0b0000_1100, // Power on, cursor off
    0b0000_1010, // Power off, cursor on
    0b0000_1110, // Power on, cursor on
    0b0000_1011, // Power off, cursor blink
    0b0000_1111, // Power on, cursor blink
    ];




/// This enum contains the available screen modes.
#[repr(u8)]
pub enum PowerMode {
    Off = 0,
    On = 1,

}

/// This enum contains the available cursor modes
#[repr(u8)]
pub enum CursorMode {
    Off = 0,
    On = 1,
    Blink = 2,
}



/// configures the cursor mode and the power mode
pub fn settings(pins: &mut Pins, cursor_mode: CursorMode, power_mode: PowerMode) {
    pins.rs.set_low();

    bwrite(
        pins,
        CONFIGURATION_LUT[2*(cursor_mode as usize) + (power_mode as usize)]
            )
}


/// This function has to be run before using an lcd, it configures the basic options for the lcd.
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
}
