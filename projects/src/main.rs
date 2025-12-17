mod simple_program; 
use crate::simple_program::{guess_game, temperature_conversion};



fn main() {
    let f = temperature_conversion::fah_to_cel(54.0);
    println!("Temperature : Fahrenheit to Celsius {}", f);
    guess_game::play_game();
    
    
}
