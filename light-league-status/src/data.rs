use crate::models::ChampionData;
use std::collections::HashMap;
use std::error::Error;
use std::fs;

/// Loads champion data from the local JSON file into a HashMap for easy lookup.
pub fn load_champion_data() -> Result<HashMap<u32, String>, Box<dyn Error>> {
    let file_content = fs::read_to_string("data/en_US/champion.json")?;
    let parsed_data: ChampionData = serde_json::from_str(&file_content)?;

    let mut champion_map = HashMap::new();
    for champion in parsed_data.data.values() {
        if let Ok(id) = champion.key.parse::<u32>() {
            champion_map.insert(id, champion.name.clone());
        }
    }

    Ok(champion_map)
}
