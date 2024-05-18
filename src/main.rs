use colored::{self, Colorize};

use crate::utils::{hash_string, Credential, Password, UserName};
use std::collections::HashMap;
use std::io;
mod utils;

fn main() {
    let user_name = loop {
        println!("Input user name:");

        let user_name_input = take_input();
        match UserName::parse(&user_name_input) {
            Ok(result) => break result,
            Err(err) => println!("Error: {}", err.as_str().red()),
        };
    };

    let password = loop {
        println!("Input password:");
        let password_input = take_input();
        match Password::parse(&password_input) {
            Ok(result) => break result,
            Err(err) => println!("Error: {}", err.as_str().red()),
        };
    };
    let cred1 = Credential::new(user_name, password);

    let mut map = HashMap::new();
    print_credentials_saved(&cred1);
    map.insert(String::from(cred1.user_name.value()), cred1);

    println!("Try to login :)");

    loop {
        println!("Enter username:");

        let login_user_input = take_input();

        let found_cred: &Credential;
        match map.get(&login_user_input) {
            Some(cred) => found_cred = cred,
            None => {
                println!(
                    "No credentials found with username = {}",
                    login_user_input.red()
                );
                continue;
            }
        }
        println!("Enter password:");

        let login_password_input = take_input();
        let hashed_password = hash_string(login_password_input);

        if hashed_password == found_cred.password.value() {
            println!("{}", "Welcome! You have been logged in!".green());
            break;
        } else {
            println!("{}", "Wrong password.".red());
        }
    }
}

fn print_credentials_saved(cred: &Credential) {
    println!(
        "Created credentials for: {} // {}",
        cred.user_name.value(),
        cred.password.value()
    );
}

fn take_input() -> String {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to take user input.");

    String::from(input.trim())
}
