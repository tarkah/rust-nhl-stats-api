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
pub struct Roster {
    #[serde(rename = "person", skip_serializing_if = "Option::is_none")]
    pub person: Option<crate::models::RosterPerson>,
    #[serde(rename = "jerseyNumber", skip_serializing_if = "Option::is_none")]
    pub jersey_number: Option<String>,
    #[serde(rename = "position", skip_serializing_if = "Option::is_none")]
    pub position: Option<crate::models::DraftProspectPrimaryPosition>,
}

impl Roster {
    pub fn new() -> Roster {
        Roster {
            person: None,
            jersey_number: None,
            position: None,
        }
    }
}


