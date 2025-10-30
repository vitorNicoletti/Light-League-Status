use std::collections::HashMap;
use std::fs;

use dotenvy::dotenv;
use serde::Deserialize;

// struct for Summoner data
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Summoner {
    puuid: String,
    game_name: String,
    tag_line: String,
}
// struct for champion mastery data
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ChampionMastery {
    puuid: String,
    champion_id: u32,
    champion_level: u32,
    champion_points: u32,
    last_play_time: u64,
    champion_points_since_last_level: u32,
    champion_points_until_next_level: u32,
    markRequiredForNextLevel: u32,
    tokens_earned: u32,
}
// enriched struct for champion mastery
#[derive(Debug)]
struct EnrichedChampionMastery {
    champion_id: u32,
    champion_level: u32,
    champion_points: u32,
    champion_name: String,
}

// struct for champion data from local JSON
#[derive(Deserialize, Debug)]
struct ChampionInfo {
    key: String,
    name: String,
}
// struct for the overall champion data JSON structure
#[derive(Deserialize, Debug)]
struct ChampionData {
    data: HashMap<String, ChampionInfo>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load the .env file
    dotenv().ok();

    let champion_lookup = load_champion_data()?;

    // --- CONFIGURATION ---
    let api_key =
        std::env::var("RIOT_API_KEY").expect("Expected a RIOT_API_KEY in the environment");

    let summoner_name = "imkenzo";
    let region = "americas";
    let tagline = "0101";

    let request_url = format!(
        "https://{}.api.riotgames.com/riot/account/v1/accounts/by-riot-id/{}/{}?api_key={}",
        region, summoner_name, tagline, api_key
    );

    println!("Requesting URL: {}", request_url);

    // --- HTTP REQUEST ---
    let client = reqwest::Client::new();
    let response = client
        .get(&request_url)
        .header("X-Riot-Token", &api_key)
        .send()
        .await?;

    // --- RESPONSE HANDLING ---
    if response.status().is_success() {
        let summoner: Summoner = response.json().await?;
        println!("Successfully fetched summoner data!");
        println!("{:#?}", summoner);
        println!("Fetching mastery data...");
        mastery(summoner.puuid, 5, &champion_lookup).await?;
    } else {
        println!("Request failed with status: {}", response.status());
        let error_text = response.text().await?;
        println!("Error details: {}", error_text);
    }

    Ok(())
}

// gets the first n champion mastery entries for a given puuid
async fn mastery(
    puuid: String,
    count: i32,
    champion_lookup: &HashMap<u32, String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("RIOT_API_KEY").expect("RIOT_API_KEY not set");
    let region = "na1";

    let request_url = format!(
        "https://{}.api.riotgames.com/lol/champion-mastery/v4/champion-masteries/by-puuid/{}/top?count={}&api_key={}",
        region, puuid, count, api_key
    );

    println!("Requesting URL: {}", request_url);

    let client = reqwest::Client::new();
    let response = client.get(&request_url).send().await?;

    if response.status().is_success() {
        // Deserialize into the API-only struct
        let api_mastery_list: Vec<ChampionMastery> = response.json().await?;

        //  Map the API data to enriched struct
        let enriched_mastery_list: Vec<EnrichedChampionMastery> = api_mastery_list
            .into_iter()
            .map(|api_entry| {
                // Look up the name using the ID from the API entry
                let champion_name = champion_lookup
                    .get(&api_entry.champion_id)
                    .cloned()
                    .unwrap_or_else(|| "Unknown Champion".to_string());

                // same struct but with name added
                EnrichedChampionMastery {
                    champion_id: api_entry.champion_id,
                    champion_level: api_entry.champion_level,
                    champion_points: api_entry.champion_points,
                    champion_name, // Added the looked-up name here
                }
            })
            .collect();

        println!(
            "--- Top {} Champion Masteries ---",
            enriched_mastery_list.len()
        );
        for entry in enriched_mastery_list {
            println!(
                "Champion: {:<16} | Level: {} | Points: {}",
                entry.champion_name, entry.champion_level, entry.champion_points
            );
        }
    } else {
        println!("Request failed with status: {}", response.status());
        let error_text = response.text().await?;
        println!("Error details: {}", error_text);
    }

    Ok(())
}

fn load_champion_data() -> Result<HashMap<u32, String>, Box<dyn std::error::Error>> {
    let file_content = fs::read_to_string("data/en_US/champion.json")?;
    let parsed_data: ChampionData = serde_json::from_str(&file_content)?;
    let mut champion_map = HashMap::new();
    for champion in parsed_data.data.values() {
        if let Ok(id) = champion.key.parse::<u32>() {
            champion_map.insert(id, champion.name.clone());
        }
    }
    println!("Champion data loaded successfully!");
    Ok(champion_map)
}
