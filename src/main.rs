#[macro_use] extern crate serenity;

use serenity::client::{
    Client,
    EventHandler,
};

use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use serenity::framework::standard::StandardFramework;
use std::env;

struct Handler;

impl EventHandler for Handler {
    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name)
    }
}

pub fn main() {
    // Login with a bot token from the environment
    let mut client = Client::new(&env::var("DISCORD_TOKEN").expect("token"), Handler)
        .expect("Error creating client");

    client.with_framework(StandardFramework::new()
        .configure(|c| c.prefix("!")) // set the bot's prefix to "~"
        .cmd("ping", ping)
        .cmd("commands", commands));

    // start listening for events by starting a single shard
    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}

command!(ping(_context, message) {
    let _ = message.reply("Pong!");
});

command!(commands(_ctx, message, _args) {
    let mut response = String::from("\n!lfg lol\n!lfg dota2\n!lfg wow");
    let _ = message.reply(&response);
});