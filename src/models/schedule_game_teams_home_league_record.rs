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
pub struct ScheduleGameTeamsHomeLeagueRecord {
    #[serde(rename = "wins", skip_serializing_if = "Option::is_none")]
    pub wins: Option<f32>,
    #[serde(rename = "losses", skip_serializing_if = "Option::is_none")]
    pub losses: Option<f32>,
    #[serde(rename = "ot", skip_serializing_if = "Option::is_none")]
    pub ot: Option<f32>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
}

impl ScheduleGameTeamsHomeLeagueRecord {
    pub fn new() -> ScheduleGameTeamsHomeLeagueRecord {
        ScheduleGameTeamsHomeLeagueRecord {
            wins: None,
            losses: None,
            ot: None,
            _type: None,
        }
    }
}

