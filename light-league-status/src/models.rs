use serde::Deserialize;
use std::collections::HashMap;

// struct for Summoner data
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Summoner {
    pub puuid: String,
    pub game_name: String,
    pub tag_line: String,
}

// struct for champion mastery data from API
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChampionMastery {
    pub puuid: String,
    pub champion_id: u32,
    pub champion_level: u32,
    pub champion_points: u32,
    pub last_play_time: u64,
    pub champion_points_since_last_level: u32,
    pub champion_points_until_next_level: u32,
    pub markRequiredForNextLevel: u32,
    pub tokens_earned: u32,
}

// Enriched struct for champion mastery
#[derive(Debug)]
pub struct EnrichedChampionMastery {
    pub champion_id: u32,
    pub champion_level: u32,
    pub champion_points: u32,
    pub champion_name: String,
}

// struct for champion data from local JSON
#[derive(Deserialize, Debug)]
pub struct ChampionInfo {
    pub key: String,
    pub name: String,
}

// struct for the overall champion data JSON structure
#[derive(Deserialize, Debug)]
pub struct ChampionData {
    pub data: HashMap<String, ChampionInfo>,
}
