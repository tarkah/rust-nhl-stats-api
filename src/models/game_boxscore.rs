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
pub struct GameBoxscore {
    #[serde(rename = "teams", skip_serializing_if = "Option::is_none")]
    pub teams: Option<crate::models::GameBoxscoreTeams>,
    #[serde(rename = "officials", skip_serializing_if = "Option::is_none")]
    pub officials: Option<Vec<crate::models::GameOfficial>>,
}

impl GameBoxscore {
    pub fn new() -> GameBoxscore {
        GameBoxscore {
            teams: None,
            officials: None,
        }
    }
}

