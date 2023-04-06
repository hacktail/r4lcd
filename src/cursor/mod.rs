use crate::*;


/// moves the cursor the the wanted position
pub fn mvc(pins: &mut Pins, mut x: u8, y: u8)
{
    pins.rs.set_low();

    // because there isn't an actual real y coordinate in the lcd, you have to just add 64 to skip two lines on the display
    // the reason we're skipping two lines is because the HD44780 is designed for 4 lines, and the two line displays bascially skip the two in the middle
    if y == 1 {
        x+=64;
    }

    if x > 128 {panic!("to big number");};
    x+=128;
    println!("{x}");


    bwrite(pins, x);

}

/// Shifts the scren left or right.
/// 0 = left, 1 = right
pub fn shiftd(pins: &mut Pins, dir: u8){
    pins.rs.set_low();
    match dir {
        0 => {
            bwrite(pins, 0b11000);
        }
        1 => {
            bwrite(pins, 0b11100);
        }
        _ => {
            panic!("Invalid direction '{}', 1 is right and 0 is left", dir);
        }
    }
}



/// sets the cursor position to 0
pub fn home(pins: &mut Pins) {
    pins.rs.set_low();
    bwrite(pins, 0b00000010);
    sleep(Duration::from_millis(2));
}


