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
pub struct Game {
    #[serde(rename = "copyright", skip_serializing_if = "Option::is_none")]
    pub copyright: Option<String>,
    #[serde(rename = "gamePk", skip_serializing_if = "Option::is_none")]
    pub game_pk: Option<f32>,
    #[serde(rename = "link", skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
    #[serde(rename = "metaData", skip_serializing_if = "Option::is_none")]
    pub meta_data: Option<crate::models::GameMetaData>,
    #[serde(rename = "gameData", skip_serializing_if = "Option::is_none")]
    pub game_data: Option<crate::models::GameGameData>,
    #[serde(rename = "liveData", skip_serializing_if = "Option::is_none")]
    pub live_data: Option<crate::models::GameLiveData>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            copyright: None,
            game_pk: None,
            link: None,
            meta_data: None,
            game_data: None,
            live_data: None,
        }
    }
}


