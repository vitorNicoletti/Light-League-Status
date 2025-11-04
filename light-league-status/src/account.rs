use crate::models::Summoner;
use std::error::Error;

/// Fetches a Summoner's account data by their Riot ID (game name + tagline).
pub async fn get_summoner(
    api_key: &str,
    summoner_name: &str,
    tagline: &str,
) -> Result<Summoner, Box<dyn Error>> {
    let region = "americas"; // Account API region

    let request_url = format!(
        "https://{}.api.riotgames.com/riot/account/v1/accounts/by-riot-id/{}/{}",
        region, summoner_name, tagline
    );

    let client = reqwest::Client::new();
    let response = client
        .get(&request_url)
        .header("X-Riot-Token", api_key) // Use header for auth
        .send()
        .await?;

    if response.status().is_success() {
        let summoner: Summoner = response.json().await?;
        Ok(summoner)
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
