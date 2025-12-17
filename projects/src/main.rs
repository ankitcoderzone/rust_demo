mod simple_program; 
// use projects::simple_program::fibonacci_series::nth_fibonacci;

use crate::simple_program::{guess_game, temperature_conversion, fibonacci_series};



fn main() {
    let _s  = fibonacci_series::nth_fibonacci(6);
    println!("fibonacci series : {}", _s);
    let f = temperature_conversion::fah_to_cel(54.0);
    println!("Temperature : Fahrenheit to Celsius {}", f);
    guess_game::play_game();
    
    
}
