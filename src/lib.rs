use rppal;
use rppal::gpio::OutputPin;
use rppal::gpio::Level;

use ascii_converter::string_to_decimals;
use std::thread::sleep;
use std::time::Duration;
pub mod pin_lib;
use pin_lib::*;
pub mod configuration;
use configuration::*;
pub mod write;
use write::*;




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





pub fn clear(pins: &mut Pins) {
    pins.rs.set_low();
    bwrite(pins, 0b00000001);
}

pub fn pulse(pins: &mut Pins) {
    pins.en.set_high();
    sleep(Duration::from_nanos(340));
    pins.en.set_low();
}



