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
pub struct StandingsRecords {
    #[serde(rename = "standingsType", skip_serializing_if = "Option::is_none")]
    pub standings_type: Option<crate::models::EnumStandingTypes>,
    #[serde(rename = "league", skip_serializing_if = "Option::is_none")]
    pub league: Option<crate::models::StandingsLeague>,
    #[serde(rename = "division", skip_serializing_if = "Option::is_none")]
    pub division: Option<crate::models::StandingsDivision>,
    #[serde(rename = "conference", skip_serializing_if = "Option::is_none")]
    pub conference: Option<crate::models::DivisionConference>,
    #[serde(rename = "teamRecords", skip_serializing_if = "Option::is_none")]
    pub team_records: Option<Vec<crate::models::StandingsTeamRecords>>,
}

impl StandingsRecords {
    pub fn new() -> StandingsRecords {
        StandingsRecords {
            standings_type: None,
            league: None,
            division: None,
            conference: None,
            team_records: None,
        }
    }
}


