pub use rppal;
pub use rppal::gpio::OutputPin;
pub use rppal::gpio::Level;

pub use ascii_converter::string_to_decimals;
pub use std::thread::sleep;
pub use std::time::Duration;
pub mod pin_lib;
pub use pin_lib::*;
pub mod configuration;
pub use configuration::*;
pub mod write;
pub use write::*;




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



