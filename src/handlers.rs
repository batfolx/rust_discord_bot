use crate::constants;
use std::path::Path;
use std::fs::File;
use std::fs;
use serenity::{model::{channel::Message, gateway::Ready}, prelude::*};
use serenity::model::id::{GuildId};
use std::collections::{HashSet, HashMap};
use serenity::model::channel::{ChannelType};


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

        let file_contents = match fs::read_to_string(&path) {
            Ok(file_contents) => {file_contents}
            Err(error) => {
                println!("Failed to read from {}, error {}", &pathname, error);
                return false;
            }
        };

        // if file content is empty we need to populate it
        if file_contents.is_empty() {
            let members = match guild_id.members(&ctx.http, Some(1000 as u64), None).await {
                Ok(members) => members,
                Err(error) => {
                    println!("Failed to get members with error {}", error);
                    return false;
                }
            };

            let mut member_hashmap: HashMap<String, HashMap<constants::MemberKeys, String>> = HashMap::new();
            for member in members.iter() {
                let user = &member.user;
                let member_key = format!("{}-{}-{}", user.name,
                                         user.discriminator, user.id);
                println!("Member key {}", member_key);

                let mut member_data: HashMap<constants::MemberKeys, String> = HashMap::new();
                member_data.insert( constants::MemberKeys::Id, user.id.to_string());
                member_data.insert(constants::MemberKeys::Name, user.name.to_owned());
                member_data.insert(constants::MemberKeys::Discriminator,
                                   user.discriminator.to_string());
                member_data.insert(constants::MemberKeys::CurrXp, String::from("0"));
                member_data.insert(constants::MemberKeys::TotalXp, String::from("0"));
                member_data.insert(constants::MemberKeys::Level, String::from("1"));
                member_data.insert(constants::MemberKeys::RoleName, String::from("Novice"));
                member_data.insert(constants::MemberKeys::MemesSent, String::from("0"));
                member_data.insert(constants::MemberKeys::MessagesSent, String::from("0"));

                member_hashmap.insert(member_key, member_data);
            }
        } else {
            println!("Members file is not empty! {}", &file_contents);
        }

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


    for (_, guild_channel) in channels.iter() {
        channel_set.insert(guild_channel.name.to_lowercase());
    }


    // create voice-only channel if it does not exist
    if !channel_set.contains(constants::VOICE_ONLY_CHANNEL) {
        match guild_id.create_channel(&ctx.http, |c|
            c.name(constants::VOICE_ONLY_CHANNEL).kind(ChannelType::Text)).await {
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

/// Gets the guild pathname
pub fn get_guild_pathname(guild_key: &String) -> String {
    return format!("{}/{}.json", constants::GUILDS_PATH, &guild_key);
}