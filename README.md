# r4lcd
## A rust library for writing to hd44780 or hd44780 compatible dot matrix lcds, like the 1602.


### Example

```rust
// Import r4lcd
use lcd::configuration::{CursorMode, PowerMode};
use r4lcd as lcd;

fn main() {
    // define the pins to be
    // dx = digital pin x on the lcd
    // rs = the registor select pin on the lcd
    // en = the enable pin on the lcd
    let pins = lcd::pins::DefinePins {
        d0: 26,
        d1: 19,
        d2: 13,
        d3: 6,
        d4: 5,
        d5: 0,
        d6: 11,
        d7: 9,
        rs: 21,
        en: 20,
    };
    
    // Pins::new take in the pins we defined, configures them
    // and sends us back a struct with the configured pins
    // this struct is used to tell which lcd the functions are going to use
    let mut my_lcd = lcd::pins::Pins::new(pins);

    // begin() sets up the lcd. It takes in two arguments, which lcd and how many display lines the lcd has
    lcd::configuration::begin(&mut my_lcd, 2);
    
    // this configures the lcd have blinking cursors and that the power to the screen should be on
    lcd::configuration::settings(&mut my_lcd, CursorMode::Blink, PowerMode::On);

    // here we write "Wowie!" to the scren
    lcd::write::write(&mut my_lcd, "Wowie!");
    
    // mvc, which means "move cursor", moves the cursor the the x and y coordinates on the screen.
    // here we're moving the cursor to column 3 row 2 
    lcd::cursor::mvc(&mut my_lcd, 2, 1); // the function uses index numbers
    
    // here we write "2 lines :0" on the second line
    lcd::write::write(&mut my_lcd, "2 lines :o");
}

```
  
### How do I add this to my project?
To add it to your project you'll have to run
```bash
cargo add --git https://github.com/hacktail/r4lcd.git
```
or add
```toml
lcd_lib = { git = "https://github.com/hacktail/r4lcd.git"}
```
to your `Cargo.toml`.

This project will soon be added to `crates.io`
I just want to polish r4lcd first :3


### Instructions
There is an unfinnished [wiki](https://github.com/hacktail/r4lcd/wiki)

## Please use this with love <3 :3
