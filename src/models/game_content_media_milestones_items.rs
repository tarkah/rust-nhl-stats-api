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
pub struct GameContentMediaMilestonesItems {
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<Type>,
    #[serde(rename = "timeAbsolute", skip_serializing_if = "Option::is_none")]
    pub time_absolute: Option<String>,
    #[serde(rename = "timeOffset", skip_serializing_if = "Option::is_none")]
    pub time_offset: Option<String>,
    #[serde(rename = "period", skip_serializing_if = "Option::is_none")]
    pub period: Option<String>,
    #[serde(rename = "statsEventId", skip_serializing_if = "Option::is_none")]
    pub stats_event_id: Option<String>,
    #[serde(rename = "teamId", skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
    #[serde(rename = "playerId", skip_serializing_if = "Option::is_none")]
    pub player_id: Option<String>,
    #[serde(rename = "periodTime", skip_serializing_if = "Option::is_none")]
    pub period_time: Option<String>,
    #[serde(rename = "ordinalNum", skip_serializing_if = "Option::is_none")]
    pub ordinal_num: Option<String>,
    #[serde(rename = "highlight", skip_serializing_if = "Option::is_none")]
    pub highlight: Option<crate::models::GameHighlight>,
}

impl GameContentMediaMilestonesItems {
    pub fn new() -> GameContentMediaMilestonesItems {
        GameContentMediaMilestonesItems {
            title: None,
            description: None,
            _type: None,
            time_absolute: None,
            time_offset: None,
            period: None,
            stats_event_id: None,
            team_id: None,
            player_id: None,
            period_time: None,
            ordinal_num: None,
            highlight: None,
        }
    }
}

/// 
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "BROADCAST_START")]
    BROADCASTSTART,
    #[serde(rename = "BROADCAST_END")]
    BROADCASTEND,
    #[serde(rename = "GOAL")]
    GOAL,
    #[serde(rename = "PERIOD_END")]
    PERIODEND,
    #[serde(rename = "PERIOD_START")]
    PERIODSTART,
    #[serde(rename = "SHOT")]
    SHOT,
}
