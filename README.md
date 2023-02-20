# r4lcd
An unfinished rust library for writing to hd44780 or hd44780 compatible dot matrix lcds, like the 1602.

>Current functions:  
  >>writing strings  
  >>moving cursor  
  >>clear screen  
  >>changing settings  
  >>writing directly to the lcd using binary
  >>Full 2-line support
  >>change begin settings
  >>moving cursor using coordinates like (2,4) THIS IS CURRENTLY BEING REMADE TO INCREASE SPEED GREATLY. I was just incrementing the cursor position which was really ineffectivte, I am now remaking it so that it just send one command with an address for where the cursor should move

    
>Upcomming functions:
  >>use whatever gpio pins you want

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


## Please use this with love <3 :3
