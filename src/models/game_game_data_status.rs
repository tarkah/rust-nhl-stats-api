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
pub struct GameGameDataStatus {
    #[serde(rename = "abstractGameState", skip_serializing_if = "Option::is_none")]
    pub abstract_game_state: Option<String>,
    #[serde(rename = "codedGameState", skip_serializing_if = "Option::is_none")]
    pub coded_game_state: Option<String>,
    #[serde(rename = "detailedState", skip_serializing_if = "Option::is_none")]
    pub detailed_state: Option<String>,
    #[serde(rename = "statusCode", skip_serializing_if = "Option::is_none")]
    pub status_code: Option<String>,
    #[serde(rename = "startTimeTBD", skip_serializing_if = "Option::is_none")]
    pub start_time_tbd: Option<bool>,
}

impl GameGameDataStatus {
    pub fn new() -> GameGameDataStatus {
        GameGameDataStatus {
            abstract_game_state: None,
            coded_game_state: None,
            detailed_state: None,
            status_code: None,
            start_time_tbd: None,
        }
    }
}

