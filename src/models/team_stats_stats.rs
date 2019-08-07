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
pub struct TeamStatsStats {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<crate::models::TeamStatsType>,
    #[serde(rename = "splits", skip_serializing_if = "Option::is_none")]
    pub splits: Option<Vec<crate::models::TeamStatsSplits>>,
}

impl TeamStatsStats {
    pub fn new() -> TeamStatsStats {
        TeamStatsStats {
            _type: None,
            splits: None,
        }
    }
}


