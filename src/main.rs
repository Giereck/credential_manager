use colored::{self, Colorize};
use fancy_regex::Regex;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::io;

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

fn is_valid_email(email: &str) -> bool {
    let re = Regex::new(r"^([a-zA-Z0-9_\-\.]+)@([a-zA-Z0-9_\-\.]+)\.([a-zA-Z]{2,5})$").unwrap();
    match re.is_match(email) {
        Ok(true) => return true,
        _ => return false,
    }
}

fn is_valid_password(password: &str) -> bool {
    let re = Regex::new(r"^(?=.*[A-Za-z])(?=.*\d)(?=.*[@$!%*?&])[A-Za-z\d@$!%*?&]{8,20}$").unwrap();
    match re.is_match(password) {
        Ok(true) => return true,
        _ => return false,
    }
}

fn hash_string<S: AsRef<str>>(s: S) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    s.as_ref().hash(&mut hasher);
    hasher.finish()
}

struct Credential {
    user_name: UserName,
    password: Password,
}

impl Credential {
    fn new(user_name: UserName, password: Password) -> Credential {
        Credential {
            user_name: user_name,
            password: password,
        }
    }
}

pub struct UserName(String);

impl UserName {
    fn parse(user_name: &str) -> Result<UserName, String> {
        let user_name = user_name.trim();
        if is_valid_email(user_name) {
            Ok(UserName(String::from(user_name)))
        } else {
            Err("User name is not a valid e-mail address.".to_string())
        }
    }

    fn value(&self) -> &String {
        &self.0
    }
}

pub struct Password(u64);

impl Password {
    fn parse(password: &str) -> Result<Password, String> {
        let password = password.trim();
        if is_valid_password(password) {
            Ok(Password(hash_string(password)))
        } else {
            Err("Not a valid password!".to_string())
        }
    }

    fn value(&self) -> u64 {
        self.0
    }
}
