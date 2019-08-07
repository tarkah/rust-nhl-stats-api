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
pub struct TeamNextGameScheduleStatus {
    #[serde(rename = "abstractGameState", skip_serializing_if = "Option::is_none")]
    pub abstract_game_state: Option<AbstractGameState>,
    #[serde(rename = "codedGameState", skip_serializing_if = "Option::is_none")]
    pub coded_game_state: Option<CodedGameState>,
    #[serde(rename = "detailedState", skip_serializing_if = "Option::is_none")]
    pub detailed_state: Option<DetailedState>,
    #[serde(rename = "statusCode", skip_serializing_if = "Option::is_none")]
    pub status_code: Option<StatusCode>,
    #[serde(rename = "startTimeTBD", skip_serializing_if = "Option::is_none")]
    pub start_time_tbd: Option<bool>,
}

impl TeamNextGameScheduleStatus {
    pub fn new() -> TeamNextGameScheduleStatus {
        TeamNextGameScheduleStatus {
            abstract_game_state: None,
            coded_game_state: None,
            detailed_state: None,
            status_code: None,
            start_time_tbd: None,
        }
    }
}

/// 
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum AbstractGameState {
    #[serde(rename = "Live")]
    Live,
    #[serde(rename = "Preview")]
    Preview,
}
/// 
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum CodedGameState {
    #[serde(rename = "2")]
    _2,
    #[serde(rename = "3")]
    _3,
}
/// 
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum DetailedState {
    #[serde(rename = "In Progress")]
    In_Progress,
    #[serde(rename = "Pre-Game")]
    PreGame,
}
/// 
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum StatusCode {
    #[serde(rename = "2")]
    _2,
    #[serde(rename = "3")]
    _3,
}

