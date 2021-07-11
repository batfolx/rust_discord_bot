use crate::constants;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

pub async fn on_bot_ready(ready: &Ready) -> bool {
    for guild in ready.guilds.iter() {
        let guild_id = guild.id();
        let guild_name = guild_id.to_string();
        let pathname = format!("{}/{}", constants::GUILDS_PATH, guild_name);
        let path = Path::new(&pathname);
        if !path.exists() {
            let _file = match File::create(&path) {
                Ok(file) => file,
                Err(why) => return false
            };
        }

    }

    return true;
}