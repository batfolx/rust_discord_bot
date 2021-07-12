use crate::constants;
use std::path::Path;
use std::fs::File;
use std::fs;
use serenity::{model::{channel::Message, gateway::Ready}, prelude::*};
use serenity::model::id::{GuildId, ChannelId};
use std::collections::HashSet;
use serenity::model::channel::{ChannelType, GuildChannel};


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

/// Does some setup when the bot successfully joins Discord
pub async fn on_bot_ready(ctx: &Context, ready: &Ready) -> bool {
    for guild in ready.guilds.iter() {
        let guild_id = guild.id();
        let guild_name = guild_id.to_string();
        let pathname = format!("{}/{}.json", constants::GUILDS_PATH, guild_name);
        let path = Path::new(&pathname);
        if !path.exists() {
            match File::create(&path) {
                Ok(_) => {
                    println!("Successfully created file {}", &pathname)
                },
                Err(_) => return false
            };
        }
        match setup_channels(&ctx, &guild_id).await {
          true => {
              println!("Successfully setup channels")
          },
          false => {
              println!("Failed to setup channels")
          }
        };

    }


    return true;
}

/// Gets a guild key and the member key associated with the given member
pub fn get_guild_member_key(msg: &Message) -> Option<(String, String)> {

    let guild_id = match &msg.guild_id {
        None => {
            println!("Could not get guild id");
            return None
        },
        Some(id) => id
    };

    let guild_key = guild_id.to_string().to_owned();
    let member = match &msg.member {
        None => {
            println!("Could not get member");
            return None
        },
        Some(member) => member,
    };

    let user = match &member.user {
        None => {
            println!("Could not get user");
            return None
        },
        Some(user) => user
    };

    let member_key = format!("{}-{}-{}", user.name, user.discriminator, user.id.to_string());
    return Some((guild_key, member_key));
}

/// sets up the channels with in the Guild
pub async fn setup_channels(ctx: &Context, guild_id: &GuildId) -> bool {

    let channels = match guild_id.channels(&ctx.http).await {
        Ok(channels) => channels,
        Err(error) => {
            println!("Error in setting up channels {}", error);
            return false;
        }
    };


    let mut channel_set = HashSet::new();
    let mut _channel_created = false;
    let mut _category_created = false;


    for (_, guild_channel) in channels.iter() {
        channel_set.insert(guild_channel.name.to_lowercase());
    }



    // create voice-only channel if it does not exist
    if !&channel_set.contains(constants::VOICE_ONLY_CHANNEL) {
        match guild_id.create_channel(&ctx.http, |c|
            c.name(constants::VOICE_ONLY_CHANNEL).kind(ChannelType::Private)).await {
            Ok(_ok) => {
                println!("Successfully created voice-only channel")
            }
            Err(error) => {
                println!("Failed to create voice-only channel, {}", error);
                return false;
            }
        };
    }

    return true;
}