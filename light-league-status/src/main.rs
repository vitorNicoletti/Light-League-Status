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
struct ChampionMastery{
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load the .env file
    dotenv().ok();

    // --- CONFIGURATION ---
    let api_key = std::env::var("RIOT_API_KEY")
        .expect("Expected a RIOT_API_KEY in the environment");

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
        mastery(summoner.puuid, 5).await?;
    } else {
        println!("Request failed with status: {}", response.status());
        let error_text = response.text().await?;
        println!("Error details: {}", error_text);
    }


    Ok(())
}

// gets the first n champion mastery entries for a given puuid
async fn mastery(puuid : String, count : i32) -> Result<(), Box<dyn std::error::Error>> {
    // Load the .env file
    dotenv().ok();

    // --- CONFIGURATION ---
    let api_key = std::env::var("RIOT_API_KEY")
        .expect("Expected a RIOT_API_KEY in the environment");

    // unsure why riot api uses na1 here for champion mastery
    let region = "na1";
    let request_url = format!(
        "https://{}.api.riotgames.com/lol/champion-mastery/v4/champion-masteries/by-puuid/{}/top?count={}&api_key={}",
        region, puuid, count, api_key
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
    // print response status code
    if response.status().is_success() {
        let mastery: Vec<ChampionMastery> = response.json().await?;
        println!("Successfully fetched champion mastery data!");
        println!("{:#?}", mastery);
    } else {
        println!("Request failed with status: {}", response.status());
        let error_text = response.text().await?;
        println!("Error details: {}", error_text);
        }
    Ok(())
}
