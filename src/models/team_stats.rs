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
pub struct TeamStats {
    #[serde(rename = "copyright", skip_serializing_if = "Option::is_none")]
    pub copyright: Option<String>,
    #[serde(rename = "stats", skip_serializing_if = "Option::is_none")]
    pub stats: Option<crate::models::TeamStatsStats>,
}

impl TeamStats {
    pub fn new() -> TeamStats {
        TeamStats {
            copyright: None,
            stats: None,
        }
    }
}


