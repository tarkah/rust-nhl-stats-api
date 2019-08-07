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
pub struct TeamRoster {
    #[serde(rename = "roster", skip_serializing_if = "Option::is_none")]
    pub roster: Option<Vec<crate::models::Roster>>,
}

impl TeamRoster {
    pub fn new() -> TeamRoster {
        TeamRoster {
            roster: None,
        }
    }
}


