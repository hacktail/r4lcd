use crate::*;

// takes in an u8 and checks if pin should be on or not by comparing the u8 and PIN_FLAGS using a bitwise and
// write(Level::from(<u8>)) turns on the pin if the u8 is bigger than 0, otherwise it turns the pin off
pub fn bwrite(pins: &mut Pins, bits: u8)// "Binary Write"
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



// Converts the &str into ascii character codes
pub fn write(pins: &mut Pins, text: &str)
{

    pins.rs.set_high();
    let binary_text = string_to_decimals(text).unwrap();

    for bits in binary_text{
        println!("writing {:b}", bits);
        bwrite(pins, bits);
    }
}


// pulses the lcd enable pin, basically single stepping a clock
pub fn pulse(pins: &mut Pins) {
    pins.en.set_high();
    sleep(Duration::from_nanos(340));
    pins.en.set_low();
}

// clears the lcd
pub fn clear(pins: &mut Pins) {
    pins.rs.set_low();
    bwrite(pins, 0b00000001);
}
