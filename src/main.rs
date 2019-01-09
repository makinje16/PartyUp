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

pub mod league_api;
pub mod lfgdb_interface;

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
            .cmd("find", find)
            .cmd("remove", remove),
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

    let reply_msg = construct_lfg_reply(&summoner_name, &ranked_info[index], &message, game);
    lfgdb_interface::insert_player(summoner_name, &message.author.name, &message.author.discriminator,
                                    &ranked_info[index].tier);
    message.reply(&reply_msg)?;
});

command!(find(_ctx, message, _args) {
    let rank = _args.single::<String>().unwrap();
    let rank = rank.to_uppercase();
    let player_list = lfgdb_interface::get_players(rank);
    let reply = construct_get_reply(player_list.players);
    message.reply(&reply)?;
});

command!(remove(_ctx, message, _args) {
    lfgdb_interface::remove_player(&message.author.name, &message.author.discriminator);
    message.reply("I removed you from the database.")?;
});

fn construct_lfg_reply(
    summoner_name: &String,
    ranked_info: &league_api::RankedQueue,
    msg: &Message,
    game: Game,
) -> String {
    format!("```This is the info being added to the database:\n\tSummoner-Name : {}\n\tDiscord-Name : {}#{}\n\tGame: {}\n\tRank: {}\n\t```"
            , &summoner_name, msg.author.name, msg.author.discriminator, game.to_string(), ranked_info.tier)
}

fn construct_get_reply(player_list: Vec<lfgdb_interface::Player>) -> String {
    let mut reply = String::from("```These are the players looking for a game:\n");
    for i in 0..player_list.len() {
        let insertion = format!("\tSummoner-name : {}\n\tDiscord-Name : {}\n------------\n", player_list[i].username, player_list[i].discord_name);
        reply.push_str(&insertion);
    }
    reply.push_str("```");
    reply
}
