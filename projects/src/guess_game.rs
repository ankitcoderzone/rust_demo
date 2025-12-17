use std::io;
use std::cmp::Ordering;
use rand::Rng;

use crate::guess_game;


pub fn play_game(){

    println!("Play guess game! ");
    println!("Enter Your Number: ");

    let secret_number = rand::thread_rng().gen_range(1..100);

    loop{
        let mut guess_number = String::new();

        io::stdin().read_line(&mut guess_number).expect("Invalid Input!");
        let guess_number : u32 = match guess_number.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess_number.cmp(&secret_number){
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }

        }

    }
}