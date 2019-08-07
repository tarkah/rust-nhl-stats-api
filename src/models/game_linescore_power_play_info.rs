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
pub struct GameLinescorePowerPlayInfo {
    #[serde(rename = "situationTimeRemaining", skip_serializing_if = "Option::is_none")]
    pub situation_time_remaining: Option<f32>,
    #[serde(rename = "situationTimeElapsed", skip_serializing_if = "Option::is_none")]
    pub situation_time_elapsed: Option<f32>,
    #[serde(rename = "inSituation", skip_serializing_if = "Option::is_none")]
    pub in_situation: Option<bool>,
}

impl GameLinescorePowerPlayInfo {
    pub fn new() -> GameLinescorePowerPlayInfo {
        GameLinescorePowerPlayInfo {
            situation_time_remaining: None,
            situation_time_elapsed: None,
            in_situation: None,
        }
    }
}


