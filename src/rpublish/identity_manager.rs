use std::fmt;
use std::fs::{File};
use std::io::{BufReader, Error, ErrorKind, Write, stdin, stdout};
use std::path::Path;
use std::collections::HashMap;
use chrono::prelude::*;
use rand_core::OsRng;
use serde::{Serialize, Deserialize};
use serde_json;
use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2
};
extern crate termion;
use termion::{color, style};

use crate::helpers::write_json;

pub struct IdentityManager
{
    users: Users,
    sessions: HashMap<String, String>
}

impl IdentityManager {
    pub fn new() -> Self {
        match Users::load_users() {
            Ok(users) => {
                Self {
                    users: users,
                    sessions: HashMap::new()
                }
            },
            Err(_) => panic!("{}Failed loading users!", color::Fg(color::Red))
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Users
{
    users: Vec<User>
}

impl Users {
    pub fn load_users() -> Result<Self, std::io::Error> {
        let path = Path::new("data/auth/users.json");
        match File::open(path) {
            Ok(file) => {
                let reader = BufReader::new(file);
                match serde_json::from_reader::<BufReader<File>, Self>(reader) {
                    Ok(users) => {
                        println!("{}Users loaded", color::Fg(color::Green));
                        Ok(users)
                    },
                    Err(error) => {
                        Err(Error::new(
                            ErrorKind::InvalidData, 
                            format!("{}Error deserializing file", color::Fg(color::Red))
                        ))
                    }
                }
            },
            Err(error) => {
                match error.kind() {
                    ErrorKind::NotFound => {
                        println!("{}Users file Not Found, Creating one", color::Fg(color::Cyan));
                        let mut new_users = Self{
                            users: Vec::new()
                        };
                        let user_name = Self::read_input("Admin username");

                        loop {
                            
                            let user_password = Self::read_input("Admin password");
                            let user_password_repeat = Self::read_input("Admin password (repeat)");

                            if user_password == user_password_repeat {
                                new_users.users.push(User::new(user_name.as_str(), user_password.as_str()));
                                break;
                            }
                        }

                        new_users.save();

                        Ok(new_users)
                    },
                    ErrorKind::PermissionDenied => {
                        println!("{}Error reading users file: Permision Denied", color::Fg(color::Red));
                        Err(Error::new(ErrorKind::InvalidData, "File read error: Permision Denied"))
                    },
                    _ => {
                        println!("{}Error reading users file {}", color::Fg(color::Red), error.to_string());
                        Err(Error::new(ErrorKind::InvalidData, "File read error not expected"))
                    }
                }
            }
        }
    }

    fn read_input(text: &str) -> String {
        let mut data : String = String::new();
        print!("    {}{}{}{}{}", style::Bold, color::Fg(color::Yellow), text, style::Reset, style::Bold);
        stdout().flush().ok().expect("Could not flush stdout");
        stdin().read_line(&mut data).expect("Error: unable to read user input");
        data
    }

    fn save(&self) {
        match serde_json::to_string(self) {
            Ok(json) => {
                match write_json("data/auth/users.json", json) {
                    Ok(_) => println!("{}Users file saved", color::Fg(color::Cyan)),
                    Err(_) => println!("{}Failed to save users file", color::Fg(color::Red)),
                }
            },
            Err(_) => println!("{}Failed to serialize users file", color::Fg(color::Red))
        }
    }

    pub fn get(username: &str) -> Result<User, IdentityError> {
        todo!()
    }
    
    pub fn create(username: &str, password: &str) -> Result<User, IdentityError> {
        todo!()
    }

    pub fn delete(username: &str, password: &str) {

    }
}

#[derive(Serialize, Deserialize)]
pub struct User
{
    user_name: String,
    first_name: String,
    last_name: String,
    show_name: String,
    password_hash: String,
    password_update_date: DateTime<Utc>,
    created_date: DateTime<Utc>,
    last_login_date: DateTime<Utc>
}

impl User {
    pub fn new(user: &str, password: &str) -> Self {
        Self {
            user_name: user.to_string(),
            first_name: String::new(),
            last_name: String::new(),
            show_name: String::new(),
            password_hash: Self::hash_password(password),
            password_update_date: chrono::offset::Utc::now(),
            created_date: chrono::offset::Utc::now(),
            last_login_date: chrono::offset::Utc::now(),
        }
    }

    pub fn hash_password(password: &str) -> String {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        argon2.hash_password_simple(
            password.as_bytes(), 
            salt.as_ref()
        ).unwrap().to_string()
    }

    pub fn authenticate(&self, password: &str) -> Result<(), IdentityError> {
        let parsed_hash = PasswordHash::new(&self.password_hash).unwrap();
        match Argon2::default().verify_password(password.as_bytes(), &parsed_hash) {
            Ok(_) => Ok(()),
            Err(_) => {
                Err(IdentityError{
                    kind: IdentityErrorKind::AuthFailed
                })
            },
        }
    }
}

#[derive(Debug, Clone)]
pub enum IdentityErrorKind
{
    UserAlreadyExist,
    UserNotFound,
    AuthFailed
}

#[derive(Debug, Clone)]
pub struct IdentityError {
    kind: IdentityErrorKind
}

impl fmt::Display for IdentityError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}