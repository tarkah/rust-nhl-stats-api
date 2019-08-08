#![allow(clippy::unreadable_literal)]

extern crate nhl_stats;

use nhl_stats::sync::configuration::Configuration;
use nhl_stats::sync::*;
use std::rc::Rc;

#[test]
fn all_api_clients() -> Result<(), Error> {
    let config = Rc::from(Configuration::default());

    let conferences_client = ConferencesApiClient::new(config.clone());
    let divisions_client = DivisionsApiClient::new(config.clone());
    let draft_client = DraftApiClient::new(config.clone());
    let games_client = GamesApiClient::new(config.clone());
    let players_client = PlayersApiClient::new(config.clone());
    let schedule_client = ScheduleApiClient::new(config.clone());
    let standings_client = StandingsApiClient::new(config.clone());
    let stats_client = StatsApiClient::new(config.clone());
    let teams_client = TeamsApiClient::new(config.clone());

    {
        println!("Conferences...");
        let _ = conferences_client.get_conference(6)?;
        let _ = conferences_client.get_conferences()?;
    }

    {
        println!("Divisions...");
        let _ = divisions_client.get_division(17)?;
        let _ = divisions_client.get_divisions()?;
    }

    {
        println!("Drafts...");
        let _ = draft_client.get_draft()?;
        let _ = draft_client.get_draft_by_year(2018.0)?;
        let _ = draft_client.get_draft_prospect(53727)?;
        let _ = draft_client.get_draft_prospects()?;
    }

    {
        println!("Games...");
        let _ = games_client.get_game(2018010011)?;
        let _ = games_client.get_game_boxscore(2018010011)?;
        let _ = games_client.get_game_content(2018010011)?;
        let _ = games_client.get_game_diff(2018010011, "20180918_003000")?;
    }

    {
        println!("Players...");
        let _ = players_client.get_player(8477474)?;
        let _ = players_client.get_player_stats(8477474, "statsSingleSeason", "20182019")?;
    }

    {
        println!("Schedule...");
        let _ = schedule_client.get_schedule(
            "",
            "",
            "2018-01-01".to_string(),
            "2018-01-01".to_string(),
        )?;
    }

    {
        println!("Standings...");
        let _ = standings_client.get_standing_types()?;
        let _ = standings_client.get_standings("".to_string(), "".to_string())?;
        let _ = standings_client.get_standings_by_type("regularSeason")?;
    }

    {
        println!("Stats...");
        let _ = stats_client.get_stat_types()?;
    }

    {
        println!("Teams...");
        let _ = teams_client.get_team(1, "", "20192020")?;
        let _ = teams_client.get_team_roster(1, "20192020")?;
        let _ = teams_client.get_team_stats(1)?;
        let _ = teams_client.get_teams("", "20192020")?;
    }

    Ok(())
}
