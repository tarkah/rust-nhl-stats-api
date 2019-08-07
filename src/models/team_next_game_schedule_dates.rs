/*
 * NHL API
 *
 * Documenting the publicly accessible portions of the NHL API.
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */



#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct TeamNextGameScheduleDates {
    #[serde(rename = "date", skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(rename = "totalItems", skip_serializing_if = "Option::is_none")]
    pub total_items: Option<f32>,
    #[serde(rename = "totalEvents", skip_serializing_if = "Option::is_none")]
    pub total_events: Option<f32>,
    #[serde(rename = "totalGames", skip_serializing_if = "Option::is_none")]
    pub total_games: Option<f32>,
    #[serde(rename = "totalMatches", skip_serializing_if = "Option::is_none")]
    pub total_matches: Option<f32>,
    #[serde(rename = "games", skip_serializing_if = "Option::is_none")]
    pub games: Option<Vec<crate::models::TeamNextGameScheduleGames>>,
    #[serde(rename = "events", skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<serde_json::Value>>,
    #[serde(rename = "matches", skip_serializing_if = "Option::is_none")]
    pub matches: Option<Vec<serde_json::Value>>,
}

impl TeamNextGameScheduleDates {
    pub fn new() -> TeamNextGameScheduleDates {
        TeamNextGameScheduleDates {
            date: None,
            total_items: None,
            total_events: None,
            total_games: None,
            total_matches: None,
            games: None,
            events: None,
            matches: None,
        }
    }
}


