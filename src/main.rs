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

enum Game {
    Lol,
    Dota,
    Wow,
}

impl Game {
    fn new(game_type: String) -> Option<Game> {
        match game_type.as_ref() {
            "dota2" => Some(Game::Dota),
            "lol" => Some(Game::Lol),
            "wow" => Some(Game::Wow),
            _ => None,
        }
    }
}

pub fn main() {
    // Login with a bot token from the environment
    let mut client = Client::new(&env::var("DISCORD_TOKEN").expect("token"), Handler)
        .expect("Error creating client");

    client.with_framework(StandardFramework::new()
        .configure(|c| c.prefix("!")) // set the bot's prefix to "~"
        .cmd("ping", ping)
        .cmd("commands", commands)
        .cmd("lfg", lfg));

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

command!(lfg(_ctx, message, _args) {
    let mut response = String::from("You are looking for a ");
    let game = Game::new(_args.single::<String>().unwrap());
    match game {
        Some(Game::Dota) => message.reply("You are looking for a Dota2 game"),
        Some(Game::Lol)  => message.reply("You are looking for a LoL game"),
        Some(Game::Wow)  => message.reply("You are looking for a WoW game"),
        None             => message.reply("Sorry we don't have functionality for that game yet"),
    };
    // let _ = message.reply(&response);
});