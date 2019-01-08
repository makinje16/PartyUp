#[macro_use]
extern crate serenity;
extern crate serde;
extern crate serde_derive;

use serenity::client::{Client, EventHandler};

use serenity::framework::standard::StandardFramework;
use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use std::env;
use reqwest;

pub mod league_api;

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

    fn to_string(&self) -> &'static str {
        match self {
            Game::Dota => "Dota 2",
            Game::Lol => "League of Legends",
            Game::Wow => "World of Warcraft",
        }
    }
}

pub fn main() {
    // Login with a bot token from the environment
    let mut client = Client::new(&env::var("DISCORD_TOKEN").expect("token"), Handler)
        .expect("Error creating client");

    client.with_framework(
        StandardFramework::new()
            .configure(|c| c.prefix("!")) // set the bot's prefix to "~"
            .cmd("commands", commands)
            .cmd("lfg", lfg)
            .cmd("ping", ping),
    );

    // start listening for events by starting a single shard
    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}

command!(commands(_ctx, message, _args) {
    let mut response = String::from("\n!lfg lol\n!lfg dota2\n!lfg wow");
    let _ = message.reply(&response);
});

command!(lfg(_ctx, message, _args) {
    let game = match Game::new(_args.single::<String>().unwrap()) {
                Some(g) => g,
                None => {
                            message.reply("Sorry that game is not implemented yet")?;
                            return Ok(())
                        }
                };
    let summoner_name = _args.single::<String>().unwrap();
    let api_key = match env::var("RIOT_API_KEY") {
                        Ok(key) => key,
                        Err(e) => panic!(e),
                    };
    let client = league_api::new_client(api_key);
    let ranked_info = client.get_ranked_info(&summoner_name);
    let mut index = 0;
    for i in 0..ranked_info.len() {
        if ranked_info[i].queue_type == "RANKED_SOLO_5x5" {
            index = i;
        }
    }

    let reply_msg = construct_lfg_reply(summoner_name, &ranked_info[index], &message, game);
    message.reply(&(*reply_msg))?;
});

fn construct_lfg_reply(summoner_name: String, ranked_info: &league_api::RankedQueue, msg: &Message, game: Game) -> Box<String> {
    let mut reply = String::from("```This is the info being added to the database:\n");
    reply.push_str("Summoner-Name : ");
    reply.push_str(&summoner_name);
    reply.push_str("\n");
    reply.push_str("Discord-Name : ");
    reply.push_str(&msg.author.name);
    reply.push_str("\n");
    reply.push_str("Game: ");
    reply.push_str(game.to_string());
    reply.push_str("\n");
    reply.push_str("Rank: ");
    reply.push_str(&ranked_info.tier);
    reply.push_str("\n```");
    Box::new(reply)
}

command!(ping(_ctx, message, _args) {
//    let client = redis::Client::open("redis-17469.c14.us-east-1-3.ec2.cloud.redislabs.com:17469");
//    let con = client.get_connection().unwrap();
//    let _ : () = con.set("my_key", 42)?;
//    match con.get("my_key") {
//        Ok(v) => {
//            let val : u32 = v;
//            println!("worked");
//            },
//        Err(why) => println!("Error: {}", why),
//    }
});
