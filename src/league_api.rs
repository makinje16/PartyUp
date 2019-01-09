use reqwest;
use serde_derive::{Deserialize, Serialize};
use serde_json;

const SUMMONER_INFO_BY_NAME_ENDPOINT: &str =
    "https://na1.api.riotgames.com/lol/summoner/v4/summoners/by-name/";
const RANKED_INFO_BY_SUMMONER_ID_ENDPOINT: &str =
    "https://na1.api.riotgames.com/lol/league/v4/positions/by-summoner/";

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SummonerInfo {
    profile_icon_id: i64,
    pub name: String,
    pub puuid: String,
    pub summoner_level: i64,
    revision_date: i64,
    pub id: String, //Summoner ID(Encrypted)
    account_id: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RankedQueue {
    pub queue_type: String,
    summoner_name: String,
    wins: i64,
    losses: i64,
    league_id: String,
    pub rank: String,
    league_name: String,
    pub tier: String,
    summoner_id: String,
    league_points: i64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MiniSeriesDTO {
    wins: i64,
    losses: i64,
    target: i64,
    progress: String,
}

pub struct Client {
    api_key: String,
}

pub fn new_client(api_key: String) -> Client {
    Client { api_key: api_key }
}

fn build_endpoint(client: &Client, base: &str, end: &String) -> Box<String> {
    let mut endpoint = String::from(base);
    endpoint.push_str(&end);
    endpoint.push_str("?api_key=");
    endpoint.push_str(client.api_key.as_ref());
    Box::new(endpoint)
}

impl Client {
    pub fn get_ranked_info(&self, summoner_name: &String) -> Vec<RankedQueue> {
        let endpoint = build_endpoint(self, SUMMONER_INFO_BY_NAME_ENDPOINT, &summoner_name);
        let endpoint_ref: &str = endpoint.as_ref();
        let response = reqwest::get(endpoint_ref).unwrap().text().unwrap();
        let summoner_info: SummonerInfo = serde_json::from_str(&response).unwrap();

        let ranked_endpoint = build_endpoint(
            &self,
            RANKED_INFO_BY_SUMMONER_ID_ENDPOINT,
            &summoner_info.id,
        );
        let ranked_endpoint_ref: &str = ranked_endpoint.as_ref();
        let response = reqwest::get(ranked_endpoint_ref).unwrap().text().unwrap();
        let ranked_info: Vec<RankedQueue> = serde_json::from_str(&response).unwrap();
        ranked_info
    }
}

impl SummonerInfo {}
