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
pub struct GameBoxscoreTeamPlayersPerson {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<f32>,
    #[serde(rename = "fullName", skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    #[serde(rename = "link", skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
    #[serde(rename = "shootsCatches", skip_serializing_if = "Option::is_none")]
    pub shoots_catches: Option<String>,
    #[serde(rename = "rosterStatus", skip_serializing_if = "Option::is_none")]
    pub roster_status: Option<String>,
}

impl GameBoxscoreTeamPlayersPerson {
    pub fn new() -> GameBoxscoreTeamPlayersPerson {
        GameBoxscoreTeamPlayersPerson {
            id: None,
            full_name: None,
            link: None,
            shoots_catches: None,
            roster_status: None,
        }
    }
}


