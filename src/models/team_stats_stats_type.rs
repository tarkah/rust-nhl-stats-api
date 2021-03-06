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
pub struct TeamStatsStatsType {
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<crate::models::EnumStatTypes>,
}

impl TeamStatsStatsType {
    pub fn new() -> TeamStatsStatsType {
        TeamStatsStatsType {
            display_name: None,
        }
    }
}


