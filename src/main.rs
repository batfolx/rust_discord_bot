mod constants;
mod handlers;
use std::env;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {


    async fn message(&self, _ctx: Context, msg: Message) {
        println!("Oi got a message");
        let (guild_key, member_key) = match handlers::get_guild_member_key(&msg) {
            None => {return}
            Some((gkey, mkey)) => (gkey, mkey)
        };

        println!("This is member key {} and this is guild key {}", &member_key, &guild_key);
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        match handlers::on_bot_ready(&ctx, &ready).await {
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