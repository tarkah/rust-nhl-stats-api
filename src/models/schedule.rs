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
pub struct Schedule {
    #[serde(rename = "copyright", skip_serializing_if = "Option::is_none")]
    pub copyright: Option<String>,
    #[serde(rename = "totalItems", skip_serializing_if = "Option::is_none")]
    pub total_items: Option<f32>,
    #[serde(rename = "totalEvents", skip_serializing_if = "Option::is_none")]
    pub total_events: Option<f32>,
    #[serde(rename = "totalGames", skip_serializing_if = "Option::is_none")]
    pub total_games: Option<f32>,
    #[serde(rename = "totalMatches", skip_serializing_if = "Option::is_none")]
    pub total_matches: Option<f32>,
    #[serde(rename = "wait", skip_serializing_if = "Option::is_none")]
    pub wait: Option<f32>,
    #[serde(rename = "dates", skip_serializing_if = "Option::is_none")]
    pub dates: Option<Vec<crate::models::ScheduleDay>>,
}

impl Schedule {
    pub fn new() -> Schedule {
        Schedule {
            copyright: None,
            total_items: None,
            total_events: None,
            total_games: None,
            total_matches: None,
            wait: None,
            dates: None,
        }
    }
}


