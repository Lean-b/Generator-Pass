use std::{env, char};
use rand::seq::SliceRandom;

const DEFAULT_LENGTH: usize = 20;

fn generate_password(length: usize, chars: &[char]) -> String {
    let random: String = (0..length).map(|_| {
        *chars.choose(&mut rand::thread_rng()).unwrap()
    }).collect();
    random
}

fn main() {
    let args: Vec<String> = env::args().collect(); 
    let mut password_chars: Vec<char> = Vec::new();
    let mut length = DEFAULT_LENGTH;

    for argument in args.iter() {
        match argument.as_str() {
            "-n" | "--numbers" => {
                password_chars.extend("123456789".chars());
            },
            "-l" | "--letters" => {
                password_chars.extend("ABCDEFGHIJKLMNÑOPQRSTUVWXYZabcdefghijklmnñopqrstuvwxyz".chars());
            },
            "-s" | "--symbols" => {
                password_chars.extend("!@#$%^&*()_+-=[]{}|;':,./<>?".chars());
            },
            "-p" | "--password" => {
                // Here you can adjust the length if desired
                // For example, if the user types "-p 15", it will generate a password of length 15
                if let Some(arg) = args.iter().position(|x| x == argument) {
                    if let Some(len) = args.get(arg + 1) {
                        if let Ok(num) = len.parse::<usize>() {
                            length = num;
                        }
                    }
                }
            },
            _ => {}
        }
    }

    if password_chars.is_empty() {
        // Default to combining all character sets
        password_chars.extend("!@#$%^&*()_+-=[]{}|;':,./<>?123456789ABCDEFGHIJKLMNÑOPQRSTUVWXYZabcdefghijklmnñopqrstuvwxyz".chars());
    }

    let password = generate_password(length, &password_chars);
    println!("{}", password);
}

