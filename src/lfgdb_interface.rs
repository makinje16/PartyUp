use reqwest;
use reqwest::Response;
use serde_derive::{Deserialize, Serialize};
use serde_json;

const INSERT_BASE: &'static str = "http://35.237.240.242/insert";
const DELETE_BASE: &'static str = "http://35.237.240.242/delete";
const GET_BASE: &'static str = "http://35.237.240.242/get";
const GET_BY_ID: &'static str = "http://35.237.240.242/get/id";

#[derive(Serialize, Deserialize)]
pub struct PlayerList {
    pub players : Vec<Player>,
}

#[derive(Serialize, Deserialize)]
pub struct Player {
    pub id : i32,
    pub username : String,
    pub discord_name : String,
    pub rank : String,
    pub discord_id : String,
}

pub fn insert_player(
    username: String,
    discord_name: &String,
    discriminator: &u16,
    discord_id: &String,
    rank: &String,
) -> Response {
    let endpoint = format!(
        "{}/{}/{}%23{}/{}/{}",
        INSERT_BASE, username, discord_name, discriminator, discord_id, rank
    );
    reqwest::get(&endpoint).unwrap()
}

pub fn remove_player(discord_name: &String, discriminator : &u16) -> Response {
    let endpoint = format!("{}/{}%23{}", DELETE_BASE, discord_name, discriminator);
    reqwest::get(&endpoint).unwrap()
}

pub fn get_players(rank: String) -> PlayerList {
    let endpoint = format!("{}/{}", GET_BASE, rank);
    let response = reqwest::get(&endpoint).unwrap().text().unwrap();
    let player_list: PlayerList = serde_json::from_str(&response).unwrap();
    player_list
}

pub fn find_by_id(id: String) -> Option<PlayerList> {
    let endpoint = format!("{}/{}",GET_BY_ID, id);
    let response = reqwest::get(&endpoint).unwrap().text().unwrap();
    let player: PlayerList = serde_json::from_str(&response).unwrap();
    if player.players.len() != 0 { return Some(player) } else {return None}
}
