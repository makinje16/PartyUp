#[macro_use]
extern crate serenity;
extern crate serde;
extern crate serde_derive;

use serenity::client::{Client, EventHandler};

use serenity::framework::standard::StandardFramework;
use serenity::http::raw::get_user;
use serenity::model;
use serenity::model::channel::ChannelType;
use serenity::model::id::ChannelId;
use serenity::model::user::User;
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
            .cmd("remove", remove)
            .cmd("invite", invite),
    );

    // start listening for events by starting a single shard
    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}

command!(commands(_ctx, message, _args) {
    let mut response = String::from("**Here's a list of commands:**\n\t**!lfg lol <summoner name>**\n\t**!find <rank>**\n\t**!invite <db_id> <voice-channel-name>**\n\t**!remove**\n\t**!commands**");
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
    let mut summoner_name = _args.single::<String>().unwrap();
    while !_args.is_empty() {
        summoner_name = format!("{} {}", summoner_name, _args.single::<String>().unwrap());
    }
    let api_key = match env::var("RIOT_API_KEY") {
                        Ok(key) => key,
                        Err(e) => panic!(e),
                    };
    let client = league_api::new_client(api_key);
    let ranked_info = client.get_ranked_info(&summoner_name);
    let mut rank: Option<String> = None;
    for i in 0..ranked_info.len() {
        if ranked_info[i].queue_type == "RANKED_SOLO_5x5" {
            rank = Some(ranked_info[i].tier.clone());
        }
    }
    if rank == None {
        rank = Some("UNRANKED".to_string());
    }
    let rank = rank.unwrap();
    let reply_msg = construct_lfg_reply(&summoner_name, &rank, &message, game);
    lfgdb_interface::insert_player(summoner_name, &message.author.name, &message.author.discriminator,
                                    &message.author.id.to_string(), &rank);
    message.reply(&reply_msg)?;
});

command!(find(_ctx, message, _args) {
    let rank = _args.single::<String>().unwrap();
    let rank = rank.to_uppercase();
    let player_list = lfgdb_interface::get_players(rank);
    let reply = construct_get_reply(player_list.players);
    let reply = format!("{}\n **To invite a player to your server run:** ```!invite <Id> <voice channel name>```", reply);
    message.reply(&reply)?;
});

command!(remove(_ctx, message, _args) {
    lfgdb_interface::remove_player(&message.author.name, &message.author.discriminator);
    message.reply("I removed you from the database.")?;
});

command!(invite(_ctx, message, _args) {
    let guild_id = message.guild_id.unwrap().channels().unwrap();
    let mut db_id = _args.single::<String>().unwrap();
    let mut channel_name = _args.single::<String>().unwrap();
    while !_args.is_empty() {
        channel_name = format!("{} {}", channel_name, _args.single::<String>().unwrap());
    }
    let mut id: Option<ChannelId> = None;
    for (channel_id, guild_channel) in guild_id {
        if guild_channel.kind == ChannelType::Voice && guild_channel.name == channel_name {
            id = Some(channel_id);
        }
    }
    let invited_player = lfgdb_interface::find_by_id(db_id);
    match id {
        Some(channel_id) => {
            let invite_link = model::invite::Invite::create(channel_id, |i| i.max_age(3600))?;
            if invited_player.players.is_empty() {
                message.reply("Sorry I couldn't find that player in the database with that id. Can you try that again?")?;
            } else {
                let recipient_user: User = get_user(invited_player.players[0].discord_id.parse::<u64>().unwrap()).unwrap();
                let recipient_msg = format!("Hey {} {}#{} want's to invite you to their server to play a game! {}",
                 recipient_user.name, message.author.name, message.author.discriminator, invite_link.url());
                let reply_str = format!(":ballot_box_with_check:Sending this {} to {}#{}", invite_link.url(), recipient_user.name, recipient_user.discriminator);
                recipient_user.direct_message(|m| m
                    .content(recipient_msg)
                    .tts(true))?;
                message.reply(&reply_str)?;
            }
        },
        None => {message.reply(":regional_indicator_x: \nI couldn't find the voice channel you searched for. Can you make sure it is spelled correctly and exists?")?;},
    }
});

fn construct_lfg_reply(
    summoner_name: &String,
    rank: &String,
    msg: &Message,
    game: Game,
) -> String {
    format!(":video_game::ballot_box_with_check:```css\nThis is the info being added to the database:\n\tSummoner-Name : {}\n\tDiscord-Name : {}#{}\n\tDiscord-Id : {}\n\tGame : {}\n\tRank : {}\n\t```"
            , &summoner_name, msg.author.name, msg.author.discriminator, msg.author.id, game.to_string(), rank)
}

fn construct_get_reply(player_list: Vec<lfgdb_interface::Player>) -> String {
    let mut reply = String::from(
        ":video_game::ballot_box_with_check:```css\nThese are the players looking for a game:\n",
    );
    for i in 0..player_list.len() {
        let insertion = format!("\tSummoner-name : {}\n\tDiscord-Name : {}\n\tId : {}\n------------------------------------\n", player_list[i].username, player_list[i].discord_name, player_list[i].id);
        reply.push_str(&insertion);
    }
    reply.push_str("\n```");
    reply
}
