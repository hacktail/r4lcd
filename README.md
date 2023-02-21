# r4lcd
A rust library for writing to hd44780 or hd44780 compatible dot matrix lcds, like the 1602.

>Current functions:  
  >>writing strings  
  >>moving cursor  
  >>clear screen  
  >>changing settings  
  >>writing directly to the lcd using binary
  >>Full 2-line support
  >>change begin settings
  >>moving cursor using coordinates like (4,1)

    
>Upcomming functions:  
  >>remaking bwrite  
  >>adding error handling
  >>use whatever gpio pins you want  
  >>shift display

>Funtions that I may add later:
  >>4-bit mode  
  >>multi-display support  
  >>suppport for screen with 4 lines ( I'll need an lcd with 4 lines first )
  
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

This project will be added to `crates.io` when all the `"Upcomming functions"` have been added.


### Instructions
upcoming instructions
will be added tomorrow

## Please use this with love <3 :3
