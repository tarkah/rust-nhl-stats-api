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
pub struct GameContentMedia {
    #[serde(rename = "epg", skip_serializing_if = "Option::is_none")]
    pub epg: Option<Vec<crate::models::AnyOfGameMediaNhltvGameMediaAudioGameHighlightType>>,
    #[serde(rename = "milestones", skip_serializing_if = "Option::is_none")]
    pub milestones: Option<crate::models::GameContentMediaMilestones>,
}

impl GameContentMedia {
    pub fn new() -> GameContentMedia {
        GameContentMedia {
            epg: None,
            milestones: None,
        }
    }
}


