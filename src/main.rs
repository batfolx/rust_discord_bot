mod constants;
mod handlers;
use std::collections::HashMap;
use std::env;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {


    async fn message(&self, ctx: Context, msg: Message) {
        println!("Oi got a message");
        //if msg.content == "!ping" {
            //if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
            //    println!("Error sending message: {:?}", why);
            //}
        //}
        let (guild_key, member_key) = match handlers::get_guild_member_key(&msg) {
            None => {return}
            Some((gkey, mkey)) => (gkey, mkey)
        };

        println!("This is member key {} and this is guild key {}", &member_key, &guild_key);
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        match handlers::on_bot_ready(&ready).await {
            true => {}
            false => {}
        }
    }
}

#[tokio::main]
async fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Need to pass a discord token");
    }
    if !handlers::setup_env() {
        panic!("Failed to setup file system");
    }

    let token = &args[1];
    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}