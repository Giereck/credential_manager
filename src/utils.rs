use fancy_regex::Regex;
use std::hash::{Hash, Hasher};

pub struct Credential {
    pub user_name: UserName,
    pub password: Password,
}

impl Credential {
    pub fn new(user_name: UserName, password: Password) -> Credential {
        Credential {
            user_name: user_name,
            password: password,
        }
    }
}

pub struct UserName(String);

impl UserName {
    pub fn parse(user_name: &str) -> Result<UserName, String> {
        let user_name = user_name.trim();
        if is_valid_email(user_name) {
            Ok(UserName(String::from(user_name)))
        } else {
            Err("User name is not a valid e-mail address.".to_string())
        }
    }

    pub fn value(&self) -> &String {
        &self.0
    }
}

pub struct Password(u64);

impl Password {
    pub fn parse(password: &str) -> Result<Password, String> {
        let password = password.trim();
        if is_valid_password(password) {
            Ok(Password(hash_string(password)))
        } else {
            Err("Not a valid password!".to_string())
        }
    }

    pub fn value(&self) -> u64 {
        self.0
    }
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

pub fn hash_string<S: AsRef<str>>(s: S) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    s.as_ref().hash(&mut hasher);
    hasher.finish()
}
