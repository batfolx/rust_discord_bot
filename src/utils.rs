use crate::handlers;

use std::env;
use std::path::Path;
use std::fs::{File};
use std::io::{Read};
use std::io::Result;

/// Gets the guild file contents, creating it if it does not exist
pub fn get_guild_file_contents(guild_key: &String) -> std::io::Result<String> {
    let pathname = handlers::get_guild_pathname(&guild_key);
    let path = Path::new(&pathname);
    if !path.exists() {
        match File::create(&path) {
            Ok(_) => println!("Successfully created {}", &pathname),
            Err(error) =>  {
                println!("Failed to create path {} with error {}", &pathname, error);
                return Err(error);
            }
        }
    }

    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(error) => {
            println!("Failed to open path {} with error {}", &pathname, error);
            return Err(error);
        }
    };


    let mut file_string = String::new();
    let _size = match file.read_to_string(&mut file_string) {
        Ok(size) => size,
        Err(error) => {
            println!("Error in reading file contents to string {}", error);
            return Err(error);
        }
    };

    return Ok(file_string);
}