use dotenvy::dotenv;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Summoner {
    puuid: String,
    game_name: String,
    tag_line: String,
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
    } else {
        println!("Request failed with status: {}", response.status());
        let error_text = response.text().await?;
        println!("Error details: {}", error_text);
    }

    Ok(())
}
