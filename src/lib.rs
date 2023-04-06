//! r4lcd is a library which makes it easier to use character lcds.
//! It supports HD44780 lcds or HD44780 compatible lcds, like the 1602, which means most character lcds will work.
//!
//! r4lcd currently supports: Writing to the lcd, changing the lcd configuration, moving the cursor, shifting the display and you can use multiple lcds if you would like.
//!
//! If you find any problems, please let me know: <https://github.com/hacktail/r4lcd>
//!
//! please use this library with love :3


// pub(crate) makes the items only public to the crate
// I do this to make it make a bit cleaner for the user when they use the library, but easier to write in the library.
pub(crate) use rppal;
pub(crate) use rppal::gpio::OutputPin;
pub(crate) use rppal::gpio::Level;
pub(crate) use ascii_converter::string_to_decimals;
pub(crate) use std::thread::sleep;
pub(crate) use std::time::Duration;
pub(crate) use pins::*;
pub(crate) use cursor::*;
pub(crate) use write::*;
#[allow(unused_imports)]
pub(crate) use configuration::*;

pub mod write;
pub mod cursor;
pub mod configuration;
pub mod pins;






