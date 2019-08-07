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
pub struct GameLiveDataDecisions {
    #[serde(rename = "winner", skip_serializing_if = "Option::is_none")]
    pub winner: Option<crate::models::GameDecisionPlayer>,
    #[serde(rename = "loser", skip_serializing_if = "Option::is_none")]
    pub loser: Option<crate::models::GameDecisionPlayer>,
    #[serde(rename = "firstStar", skip_serializing_if = "Option::is_none")]
    pub first_star: Option<crate::models::GameDecisionPlayer>,
    #[serde(rename = "secondStar", skip_serializing_if = "Option::is_none")]
    pub second_star: Option<crate::models::GameDecisionPlayer>,
    #[serde(rename = "thirdStar", skip_serializing_if = "Option::is_none")]
    pub third_star: Option<crate::models::GameDecisionPlayer>,
}

impl GameLiveDataDecisions {
    pub fn new() -> GameLiveDataDecisions {
        GameLiveDataDecisions {
            winner: None,
            loser: None,
            first_star: None,
            second_star: None,
            third_star: None,
        }
    }
}

