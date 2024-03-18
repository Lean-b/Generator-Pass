use rand::seq::IteratorRandom;
use std::{char, env};

#[derive(Debug)]
enum Types {
    SYMBOL(Vec<char>),
    NUMBER(Vec<char>),
    LETTER(Vec<char>),
}

#[derive(Debug)]
struct Password {
    typ: Types,
}

impl Password {
    fn generator() {
        let mut rng = rand::thread_rng();
        
        let symbol = Password {
            typ: Types::SYMBOL("!@#$%^&*()_+-=[]{}|;':,./<>?".chars().collect()),
        };
        let number = Password {
            typ: Types::NUMBER("123456789".chars().collect()),
        };
        let letter = Password {
            typ: Types::LETTER(
                "ABCDEFGHIJKLMNÑOPQRSTUVWXYZabcdefghijklmnñopqrstuvwxyz"
                    .chars()
                    .collect(),
            ),
        };

        let combinator = match  {
            
        };
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let pass = Password::generator();

    for argument in args.iter() {
        match argument.as_str() {
            "-n" | "--numbers" => {
                let password = numbers();
                println!("{}", password);
            }
            "-l" | "--letter" => {
                let password = letter();
                println!("{}", password);
            }
            "-s" | "--symbol" => {
                let password = symbol();
                println!("{}", password);
            }
            "p" | "--password" => {
                let password = combination();
                println!("{}", password);
            }
            _ => {}
        }
    }
}
