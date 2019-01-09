use reqwest;
use reqwest::Response;

const INSERT_BASE: &'static str = "http://35.237.240.242/insert";
const DELETE_BASE: &'static str = "http://35.237.240.242/delete";
const GET_BASE: &'static str = "http://35.237.240.242/get";

pub fn insert_player(
    username: String,
    discord_name: &String,
    discriminator: &u16,
    rank: &String,
) -> Response {
    let endpoint = format!("{}/{}/{}%23{}/{}", INSERT_BASE, username, discord_name, discriminator, rank);
    println!("endpoint: {}", endpoint);
    reqwest::get(&endpoint).unwrap()
}

pub fn remove_player(discord_name: String) -> Response {
    let endpoint = format!("{}/{}", DELETE_BASE, discord_name);
    reqwest::get(&endpoint).unwrap()
}

pub fn get_players(rank: String) -> Response {
    let endpoint = format!("{}/{}", GET_BASE, rank);
    reqwest::get(&endpoint).unwrap()
}
