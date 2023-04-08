use crate::*;


/// moves the cursor the the wanted position
pub fn mvc(pins: &mut Pins, x: u8, mut y: u8) -> Result<(), &str>
{
    pins.rs.set_low();

    // because there isn't an actual real y coordinate in the lcd, you have to just add 64 to skip two lines on the display
    // the reason we're skipping two lines is because the HD44780 is designed for 4 lines, and the two line displays bascially skip the two in the middle
    if x == 1 {
        y+=64;
    }

    if y > 128 {return Err("wow")};
    y+=128;
    bwrite(pins, y);
Ok(())
}

/// Shifts the scren left or right.
/// 0 = left, 1 = right
pub fn shiftd(pins: &mut Pins, dir: u8) -> Result<(), &str>{
    pins.rs.set_low();
    match dir {
        0 => {
            bwrite(pins, 0b11000);
        }
        1 => {
            bwrite(pins, 0b11100);
        }
        _ => {
            return Err("Invalid direction");
        }
    }
Ok(())
}



/// sets the cursor position to 0
pub fn home(pins: &mut Pins) {
    pins.rs.set_low();
    bwrite(pins, 0b00000010);
    sleep(Duration::from_millis(2));
}


