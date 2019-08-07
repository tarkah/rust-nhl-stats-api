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
pub struct GameLinescoreTeam {
    #[serde(rename = "team", skip_serializing_if = "Option::is_none")]
    pub team: Option<crate::models::GameBoxscoreTeamTeam>,
    #[serde(rename = "goals", skip_serializing_if = "Option::is_none")]
    pub goals: Option<f32>,
    #[serde(rename = "shotsOnGoal", skip_serializing_if = "Option::is_none")]
    pub shots_on_goal: Option<f32>,
    #[serde(rename = "goaliePulled", skip_serializing_if = "Option::is_none")]
    pub goalie_pulled: Option<bool>,
    #[serde(rename = "numSkaters", skip_serializing_if = "Option::is_none")]
    pub num_skaters: Option<f32>,
    #[serde(rename = "powerPlay", skip_serializing_if = "Option::is_none")]
    pub power_play: Option<bool>,
}

impl GameLinescoreTeam {
    pub fn new() -> GameLinescoreTeam {
        GameLinescoreTeam {
            team: None,
            goals: None,
            shots_on_goal: None,
            goalie_pulled: None,
            num_skaters: None,
            power_play: None,
        }
    }
}


