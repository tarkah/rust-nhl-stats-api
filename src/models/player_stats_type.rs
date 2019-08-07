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
pub struct PlayerStatsType {
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<DisplayName>,
}

impl PlayerStatsType {
    pub fn new() -> PlayerStatsType {
        PlayerStatsType {
            display_name: None,
        }
    }
}

/// 
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum DisplayName {
    #[serde(rename = "byDayOfWeek")]
    ByDayOfWeek,
    #[serde(rename = "byMonth")]
    ByMonth,
    #[serde(rename = "goalsByGameSituation")]
    GoalsByGameSituation,
    #[serde(rename = "homeAndAway")]
    HomeAndAway,
    #[serde(rename = "onPaceRegularSeason")]
    OnPaceRegularSeason,
    #[serde(rename = "regularSeasonStatRankings")]
    RegularSeasonStatRankings,
    #[serde(rename = "statsSingleSeason")]
    StatsSingleSeason,
    #[serde(rename = "vsConference")]
    VsConference,
    #[serde(rename = "vsDivision")]
    VsDivision,
    #[serde(rename = "vsTeam")]
    VsTeam,
    #[serde(rename = "winLoss")]
    WinLoss,
}

