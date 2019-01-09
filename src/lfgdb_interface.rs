use reqwest;
use reqwest::Response;
use serde_derive::{Deserialize, Serialize};
use serde_json;

const INSERT_BASE: &'static str = "http://35.237.240.242/insert";
const DELETE_BASE: &'static str = "http://35.237.240.242/delete";
const GET_BASE: &'static str = "http://35.237.240.242/get";

#[derive(Serialize, Deserialize)]
pub struct PlayerList {
    pub players : Vec<Player>,
}

#[derive(Serialize, Deserialize)]
pub struct Player {
    id : i64,
    pub username : String,
    pub discord_name : String,
    pub rank : String,
}

pub fn insert_player(
    username: String,
    discord_name: &String,
    discriminator: &u16,
    rank: &String,
) -> Response {
    let endpoint = format!(
        "{}/{}/{}%23{}/{}",
        INSERT_BASE, username, discord_name, discriminator, rank
    );
    println!("endpoint: {}", endpoint);
    reqwest::get(&endpoint).unwrap()
}

pub fn remove_player(discord_name: String) -> Response {
    let endpoint = format!("{}/{}", DELETE_BASE, discord_name);
    reqwest::get(&endpoint).unwrap()
}

pub fn get_players(rank: String) -> PlayerList {
    let endpoint = format!("{}/{}", GET_BASE, rank);
    let response = reqwest::get(&endpoint).unwrap().text().unwrap();
    println!("Response: {}", response);
    let player_list: PlayerList = serde_json::from_str(&response).unwrap();
    player_list
}
