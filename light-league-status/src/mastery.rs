use crate::models::{ChampionMastery, EnrichedChampionMastery};
use std::collections::HashMap;
use std::error::Error;

/// Fetches the top 'count' champion masteries for a given PUUID.
pub async fn get_mastery(
    api_key: &str,
    puuid: &str,
    count: i32,
    champion_lookup: &HashMap<u32, String>,
) -> Result<Vec<EnrichedChampionMastery>, Box<dyn Error>> {
    let region = "na1"; // Champion Mastery API region

    let request_url = format!(
        "https://{}.api.riotgames.com/lol/champion-mastery/v4/champion-masteries/by-puuid/{}/top?count={}",
        region, puuid, count
    );

    let client = reqwest::Client::new();
    let response = client
        .get(&request_url)
        .header("X-Riot-Token", api_key) // Use header for auth
        .send()
        .await?;

    if response.status().is_success() {
        // Deserialize into the API-only struct
        let api_mastery_list: Vec<ChampionMastery> = response.json().await?;

        // Map the API data to our enriched struct
        let enriched_mastery_list: Vec<EnrichedChampionMastery> = api_mastery_list
            .into_iter()
            .map(|api_entry| {
                // Look up the name using the ID from the API entry
                let champion_name = champion_lookup
                    .get(&api_entry.champion_id)
                    .cloned()
                    .unwrap_or_else(|| "Unknown Champion".to_string());

                // Create the enriched struct
                EnrichedChampionMastery {
                    champion_id: api_entry.champion_id,
                    champion_level: api_entry.champion_level,
                    champion_points: api_entry.champion_points,
                    champion_name, // Added the looked-up name here
                }
            })
            .collect();

        Ok(enriched_mastery_list)
    } else {
        let status = response.status();
        let error_text = response.text().await?;
        Err(format!(
            "Request failed with status: {}. Details: {}",
            status, error_text
        )
        .into())
    }
}
