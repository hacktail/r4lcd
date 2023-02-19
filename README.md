# r4lcd
An unfinished rust library for writing to hd44780 or hd44780 compatible dot matrix lcds, like the 1602.

>Current functions:  
  >>writing strings  
  >>moving cursor  
  >>clear screen  
  >>changing settings  
  >>writing directly to the lcd using binary  
    
>Upcomming functions:  
  >>use whatever gpio pins you want  
  >>multi-display support  
  >>Full 2-line support  
  >>moving cursor using coordinates like (2,4)  
  >>change begin settings  
    
>Funtions that I will add later:  
  >>4-bit mode  

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

Please use this with love <3 :3