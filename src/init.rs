use std::fs;
use std::path::Path;
use std::io::{Result, Error, ErrorKind};

extern crate termion;
use termion::{color};

pub fn setup_system() -> Result<()> {
    let setup_dirs_complete = setup_directories_structure();
    if setup_dirs_complete {
        Ok(())
    }else{
        Err(Error::new(ErrorKind::Other, "Checks for directory structure failed"))
    }
}

fn setup_directories_structure() -> bool{
    // Root directory
    if !graceful_mkdir("data") {return false;}

    // Public directory for uploads
    if !graceful_mkdir("data/public") {return false;}
    if !graceful_mkdir("data/public/images") {return false;}
    if !graceful_mkdir("data/public/files") {return false;}

    // Articles
    if !graceful_mkdir("data/articles") {return false;}

    // System cache
    if !graceful_mkdir("data/cache") {return false;}
    if !graceful_mkdir("data/cache/search") {return false;}
    if !graceful_mkdir("data/cache/stats") {return false;}

    // System logs
    if !graceful_mkdir("data/logs") {return false;}

    true
}

fn graceful_mkdir(dir_path: &str) -> bool {
    let current_path = std::env::current_dir().unwrap();
    let current_path = current_path.as_path().display();
    let path = Path::new(dir_path);
    // Get the metadata attributes of a file/dir
    let attributes_result = fs::metadata(&path);
    // Check if it exists or something is wrong
    match attributes_result {
        Ok(attributes) => {
            if attributes.is_dir() {
                if attributes.permissions().readonly()
                {
                    println!("{}{}/{}: Is not writable", color::Fg(color::Red), current_path, path.display());
                    return false;
                }
                println!("{}{}/{}: OK", color::Fg(color::Green), current_path, path.display());
                true
            }
            else {
                println!("{}{}/{}: Is not a directory", color::Fg(color::Red), current_path,path.display());
                false
            }
        },
        Err(error) => {
            // Get the error kind to compare later
            let error_kind = error.kind();
            match error_kind {
                ErrorKind::NotFound => {
                    // The dir not exists, create dir
                    let create_result = fs::create_dir(path);
                    match create_result {
                        Ok(_) => {
                            println!("{}{}/{}: Created", color::Fg(color::Cyan), current_path,path.display());
                            true
                        },
                        Err(create_error) => 
                        {
                            println!("{}{}/{}: {}", color::Fg(color::Red), current_path,path.display(), create_error);
                            false
                        }
                    }
                },
                _ => {
                    println!("{}Error not managed {}", color::Fg(color::Red), error);
                    false
                }
            }
        }
    }
    
}