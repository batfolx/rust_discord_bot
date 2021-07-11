use crate::constants;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::fs;
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use std::io::Error;


/// Sets up the file system environment
pub fn setup_env() -> bool {

    for directory in constants::DIRECTORIES.iter() {
        match fs::create_dir_all(Path::new(directory)) {
            Ok(_) => {}
            Err(why) => {
                println!("Failed to create directory {} why {}", &directory, why);
                return false;
            }
        }
    }
    return true;
}

//! Does some setup when the bot successfully joins Discord
pub async fn on_bot_ready(ready: &Ready) -> bool {
    for guild in ready.guilds.iter() {
        let guild_id = guild.id();
        let guild_name = guild_id.to_string();
        let pathname = format!("{}/{}", constants::GUILDS_PATH, guild_name);
        let path = Path::new(&pathname);
        if !path.exists() {
            let _file = match File::create(&path) {
                Ok(file) => file,
                Err(_) => return false
            };
        }
    }
    return true;
}