// Declare our new modules
mod account;
mod data;
mod mastery;
mod models;

use dotenvy::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // --- CONFIGURATION ---
    dotenv().ok();
    let api_key =
        std::env::var("RIOT_API_KEY").expect("Expected a RIOT_API_KEY in the environment");

    // Load champion data once at the start
    let champion_lookup = data::load_champion_data()?;
    println!("Champion data loaded successfully!");

    // --- INPUTS ---
    let summoner_name = "imkenzo";
    let tagline = "0101";
    let mastery_count = 5;

    // --- GET SUMMONER (USER LOOKUP) ---
    println!(
        "Fetching summoner data for {}#{}...",
        summoner_name, tagline
    );
    match account::get_summoner(&api_key, summoner_name, tagline).await {
        Ok(summoner) => {
            println!("Successfully fetched summoner!");
            println!("  PUUID: {}", summoner.puuid);
            println!("  GameName: {}", summoner.game_name);

            // 3. --- GET MASTERY (using the PUUID from step 2) ---
            println!("\nFetching top {} mastery data...", mastery_count);
            match mastery::get_mastery(&api_key, &summoner.puuid, mastery_count, &champion_lookup)
                .await
            {
                Ok(mastery_list) => {
                    println!("--- Top {} Champion Masteries ---", mastery_list.len());
                    for entry in mastery_list {
                        println!(
                            "  Champion: {:<16} | Level: {} | Points: {}",
                            entry.champion_name, entry.champion_level, entry.champion_points
                        );
                    }
                }
                Err(e) => {
                    println!("Failed to fetch mastery data: {}", e);
                }
            }
        }
        Err(e) => {
            println!("Failed to fetch summoner data: {}", e);
        }
    }

    Ok(())
}
