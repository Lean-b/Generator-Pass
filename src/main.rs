use std::{env, char};
use rand::seq::SliceRandom;

const LENGTH: usize = 20;

fn numbers() -> String {
    let number: Vec<char> = "123456789".chars().collect();
    let  random: String = (0..LENGTH).map(|_|{
        *number.choose(&mut rand::thread_rng()).unwrap()
        }
    ).collect();
    random
}


fn letter() -> String {
    let letters: Vec<char> = "ABCDEFGHIJKLMNÑOPQRSTUVWXYZabcdefghijklmnñopqrstuvwxyz".chars().collect();
    let  random: String = (0..LENGTH).map(|_|{
        *letters.choose(&mut rand::thread_rng()).unwrap()
        }
    ).collect();
    random
}


fn symbol() -> String {
    let symbols: Vec<char> = "!@#$%^&*()_+-=[]{}|;':,./<>?".chars().collect();
    let  random: String = (0..LENGTH).map(|_|{
        *symbols.choose(&mut rand::thread_rng()).unwrap()
        }
    ).collect();
    random
}



fn combination() -> String {
    let password: Vec<char> = "!@#$%^&*()_+-=[]{}|;':,./<>?123456789ABCDEFGHIJKLMNÑOPQRSTUVWXYZabcdefghijklmnñopqrstuvwxyz".chars().collect();
    let  random: String = (0..LENGTH).map(|_|{
        *password.choose(&mut rand::thread_rng()).unwrap()
        }
    ).collect();
    random
}

fn main() {
    let args: Vec<String> = env::args().collect(); 

   for argument in args.iter(){
        match argument.as_str() {
            "-n" | "--numbers" => {
                let password = numbers();
                println!("{}",password);    
            },
            "-l" | "--letter" => {
                let password = letter();
                println!("{}",password);
            },
            "-s" | "--symbol" => {
                let password = symbol();
                println!("{}",password);
            },
            "p" | "--password" => {
                let password = combination();
                println!("{}",password);  
            },
            _ => {
            // Acciones para otros argumentos
            }
        }
    }
}
