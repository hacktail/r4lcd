use crate::*;

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

