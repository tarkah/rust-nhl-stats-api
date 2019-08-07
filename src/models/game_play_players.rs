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
pub struct GamePlayPlayers {
    #[serde(rename = "player", skip_serializing_if = "Option::is_none")]
    pub player: Option<crate::models::GamePlayPlayer>,
    #[serde(rename = "playerType", skip_serializing_if = "Option::is_none")]
    pub player_type: Option<String>,
}

impl GamePlayPlayers {
    pub fn new() -> GamePlayPlayers {
        GamePlayPlayers {
            player: None,
            player_type: None,
        }
    }
}


