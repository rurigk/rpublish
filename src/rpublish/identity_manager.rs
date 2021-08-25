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
use termion::input::TermRead;
use termion::{color, style};

use crate::helpers::write_json;

pub struct IdentityManager
{
    pub users: Users,
    pub sessions: Sessions
}

impl IdentityManager {
    pub fn new() -> Self {
        match Users::load_users() {
            Ok(users) => {
                match Sessions::load_sessions() {
                    Ok(sessions) => {
                        Self {
                            users,
                            sessions
                        }
                    },
                    Err(_) => panic!("{}Failed loading sessions!", color::Fg(color::Red))
                }
            },
            Err(_) => panic!("{}Failed loading users!", color::Fg(color::Red))
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Sessions {
    sessions: HashMap<String, Session>
}

impl Sessions {
    pub fn load_sessions() -> Result<Self, std::io::Error> {
        let path = Path::new("data/auth/sessions.json");
        match File::open(path) {
            Ok(file) => {
                let reader = BufReader::new(file);
                match serde_json::from_reader::<BufReader<File>, Self>(reader) {
                    Ok(users) => {
                        println!("{}Sessions loaded", color::Fg(color::Green));
                        Ok(users)
                    },
                    Err(_) => {
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
                        println!("{}Sessions file Not Found, Creating one", color::Fg(color::Cyan));
                        let new_sessions = Self{
                            sessions: HashMap::new()
                        };
                        new_sessions.save();
                        Ok(new_sessions)
                    },
                    ErrorKind::PermissionDenied => {
                        println!("{}Error reading sessions file: Permision Denied", color::Fg(color::Red));
                        Err(Error::new(ErrorKind::InvalidData, "File read error: Permision Denied"))
                    },
                    _ => {
                        println!("{}Error reading sessions file {}", color::Fg(color::Red), error.to_string());
                        Err(Error::new(ErrorKind::InvalidData, "File read error not expected"))
                    }
                }
            }
        }
    }

    pub fn validate(&self, sessid: &str) -> bool {
        self.sessions.get(sessid).is_some()
    }

    pub fn invalidate(&mut self, sessid: &str)
    {
        self.sessions.remove(sessid);
        self.save();
    }

    pub fn create(&mut self, sessid: String, username: String, ip: String)
    {
        self.sessions.insert(sessid, Session{
            username,
            ip,
            date: chrono::offset::Utc::now(),
        });
        self.save();
    }

    pub fn get_user(&self, sessid: &String) -> Option<String> {
        match self.sessions.get(sessid) {
            Some(session) => {
                Some(session.username.to_owned())
            },
            None => None,
        }
    }

    fn save(&self) {
        match serde_json::to_string(self) {
            Ok(json) => {
                match write_json("data/auth/sessions.json", json) {
                    Ok(_) => println!("{}Sessions file saved", color::Fg(color::Cyan)),
                    Err(_) => println!("{}Failed to save sessions file", color::Fg(color::Red)),
                }
            },
            Err(_) => println!("{}Failed to serialize sessions file", color::Fg(color::Red))
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Session {
    username: String,
    ip: String,
    date: DateTime<Utc>
}

impl Session {
    
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
                    Err(_) => {
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
                        let user_name = Self::read_input("Admin username: ");

                        loop {
                            
                            let user_password = Self::read_input_hidden("Admin password: ")
                                .ok_or_else(|| std::io::Error::from(ErrorKind::InvalidInput))?;
                            let user_password_repeat = Self::read_input_hidden("Admin password (repeat): ")
                                .ok_or_else(|| std::io::Error::from(ErrorKind::InvalidInput))?;

                            if user_password == user_password_repeat {
                                let mut new_user = User::new(user_name.trim(), user_password.trim());
                                new_user.set_permission(UserPermissions::Admin);
                                new_user.set_permission(UserPermissions::Editor);
                                new_users.users.push(new_user);
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
        print!("    {}{}{}{}{}{}", style::Reset, style::Bold, color::Fg(color::Yellow), text, style::Reset, style::Bold);
        stdout().flush().expect("Could not flush stdout");
        stdin().read_line(&mut data).expect("Error: unable to read user input");
        data.trim().to_string()
    }

    fn read_input_hidden(text: &str) -> Option<String> {
        let mut stdout = stdout();
        print!("\n    {}{}{}{}{}{}", style::Reset, style::Bold, color::Fg(color::Yellow), text, style::Reset, style::Bold);
        stdout.flush().expect("Could not flush stdout");
        stdin().read_passwd(&mut stdout).expect("Error: unable to read user input")
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

    pub fn get(&self, username: &str) -> Result<&User, IdentityError> {
        let user_iter = self.users.iter();
        for user in user_iter {
            if user.user_name.as_str() == username {
                return Ok(user)
            }
        }
        Err(IdentityError{
            kind: IdentityErrorKind::UserNotFound
        })
    }
    
    #[allow(dead_code)]
    pub fn create(&mut self, username: &str, password: &str) -> Result<&User, IdentityError> {
        match self.get(username) {
            Ok(_) => {
                Err(IdentityError{
                    kind: IdentityErrorKind::UserAlreadyExist
                })
            },
            Err(_) => {
                let user = User::new(username.trim(), password.trim());
                self.users.push(user);
                self.save();
                Ok(self.users.last().unwrap())
            },
        }
    }

    #[allow(dead_code)]
    pub fn delete(&mut self, username: &str, password: &str) -> Result<(), IdentityError> {
        match self.get(username) {
            Ok(user) => {
                match user.authenticate(password) {
                    Ok(_) => todo!(),
                    Err(_) => todo!(),
                }
            },
            Err(_) => {
                Err(IdentityError{
                    kind: IdentityErrorKind::UserNotFound
                })
            },
        }
    }
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, PartialEq, Eq)]
pub enum UserPermissions
{
    Admin,
    Editor
}

#[derive(Serialize, Deserialize)]
pub struct User
{
    user_name: String,
    password_hash: String,
    password_update_date: DateTime<Utc>,
    created_date: DateTime<Utc>,
    last_login_date: DateTime<Utc>,
    enabled: bool,
    permissions: Vec<UserPermissions>
}

impl User {
    pub fn new(user: &str, password: &str) -> Self {
        Self {
            user_name: user.to_string(),
            password_hash: Self::hash_password(password),
            password_update_date: chrono::offset::Utc::now(),
            created_date: chrono::offset::Utc::now(),
            last_login_date: chrono::offset::Utc::now(),
            enabled: true,
            permissions: Vec::new(),
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

    pub fn set_permission(&mut self, permission: UserPermissions) {
        if !self.permissions.contains(&permission) {
            self.permissions.push(permission);
        }
    }
}

#[allow(dead_code)]
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
        write!(f, "Indentity Error")
    }
}