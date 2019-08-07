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
pub struct Team {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<f32>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "link", skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
    #[serde(rename = "venue", skip_serializing_if = "Option::is_none")]
    pub venue: Option<crate::models::Venue>,
    #[serde(rename = "abbreviation", skip_serializing_if = "Option::is_none")]
    pub abbreviation: Option<String>,
    #[serde(rename = "triCode", skip_serializing_if = "Option::is_none")]
    pub tri_code: Option<String>,
    #[serde(rename = "teamName", skip_serializing_if = "Option::is_none")]
    pub team_name: Option<String>,
    #[serde(rename = "locationName", skip_serializing_if = "Option::is_none")]
    pub location_name: Option<String>,
    #[serde(rename = "firstYearOfPlay", skip_serializing_if = "Option::is_none")]
    pub first_year_of_play: Option<f32>,
    #[serde(rename = "division", skip_serializing_if = "Option::is_none")]
    pub division: Option<crate::models::StandingsDivision>,
    #[serde(rename = "conference", skip_serializing_if = "Option::is_none")]
    pub conference: Option<crate::models::DivisionConference>,
    #[serde(rename = "franchise", skip_serializing_if = "Option::is_none")]
    pub franchise: Option<crate::models::Franchise>,
    #[serde(rename = "roster", skip_serializing_if = "Option::is_none")]
    pub roster: Option<crate::models::TeamRoster>,
    #[serde(rename = "nextGameSchedule", skip_serializing_if = "Option::is_none")]
    pub next_game_schedule: Option<crate::models::TeamNextGameSchedule>,
    #[serde(rename = "shortName", skip_serializing_if = "Option::is_none")]
    pub short_name: Option<String>,
    #[serde(rename = "officialSiteUrl", skip_serializing_if = "Option::is_none")]
    pub official_site_url: Option<String>,
    #[serde(rename = "franchiseId", skip_serializing_if = "Option::is_none")]
    pub franchise_id: Option<f32>,
    #[serde(rename = "active", skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
}

impl Team {
    pub fn new() -> Team {
        Team {
            id: None,
            name: None,
            link: None,
            venue: None,
            abbreviation: None,
            tri_code: None,
            team_name: None,
            location_name: None,
            first_year_of_play: None,
            division: None,
            conference: None,
            franchise: None,
            roster: None,
            next_game_schedule: None,
            short_name: None,
            official_site_url: None,
            franchise_id: None,
            active: None,
        }
    }
}


