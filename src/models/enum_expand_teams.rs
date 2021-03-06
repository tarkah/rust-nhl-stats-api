/*
 * NHL API
 *
 * Documenting the publicly accessible portions of the NHL API.
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */


/// 
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum EnumExpandTeams {
    #[serde(rename = "team.roster")]
    TeamRoster,
    #[serde(rename = "person.names")]
    PersonNames,
    #[serde(rename = "team.schedule.next")]
    TeamScheduleNext,
    #[serde(rename = "team.schedule.previous")]
    TeamSchedulePrevious,
    #[serde(rename = "team.stats")]
    TeamStats,

}



