#![allow(clippy::unreadable_literal)]

extern crate nhl_stats;

use nhl_stats::parameters::*;
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
        conferences_client.get_conference(6)?;
        conferences_client.get_conferences()?;
    }

    {
        println!("Divisions...");
        divisions_client.get_division(17)?;
        divisions_client.get_divisions()?;
    }

    {
        println!("Drafts...");
        draft_client.get_draft()?;
        draft_client.get_draft_by_year(2018.0)?;
        draft_client.get_draft_prospect(53727)?;
        draft_client.get_draft_prospects()?;
    }

    {
        println!("Games...");
        games_client.get_game(2018010011)?;
        games_client.get_game_boxscore(2018010011)?;
        games_client.get_game_content(2018010011)?;
        games_client.get_game_diff(2018010011, "20180918_003000")?;
    }

    {
        println!("Players...");
        let stats_types = vec![
            EnumStatTypes::ByDayOfWeek,
            EnumStatTypes::ByMonth,
            EnumStatTypes::GoalsByGameSituation,
            EnumStatTypes::HomeAndAway,
            EnumStatTypes::OnPaceRegularSeason,
            EnumStatTypes::RegularSeasonStatRankings,
            EnumStatTypes::StatsSingleSeason,
            EnumStatTypes::VsConference,
            EnumStatTypes::VsDivision,
            EnumStatTypes::VsTeam,
            EnumStatTypes::WinLoss,
            EnumStatTypes::ByDayOfWeekPlayoffs,
            EnumStatTypes::ByMonthPlayoffs,
            EnumStatTypes::CareerPlayoffs,
            EnumStatTypes::CareerRegularSeason,
            EnumStatTypes::GameLog,
            EnumStatTypes::GoalsByGameSituationPlayoffs,
            EnumStatTypes::HomeAndAwayPlayoffs,
            EnumStatTypes::PlayoffGameLog,
            EnumStatTypes::PlayoffStatRankings,
            EnumStatTypes::StatsSingleSeasonPlayoffs,
            EnumStatTypes::VsConferencePlayoffs,
            EnumStatTypes::VsDivisionPlayoffs,
            EnumStatTypes::VsTeamPlayoffs,
            EnumStatTypes::WinLossPlayoffs,
            EnumStatTypes::YearByYear,
            EnumStatTypes::YearByYearPlayoffs,
            EnumStatTypes::YearByYearPlayoffsRank,
            EnumStatTypes::YearByYearRank,
        ];
        let _ = players_client.get_player(8477474)?;
        for _type in stats_types.into_iter() {
            players_client.get_player_stats(8477474, _type, "20182019")?;
        }
    }

    {
        println!("Schedule...");
        let expands = vec![
            EnumExpandSchedule::Broadcasts,
            EnumExpandSchedule::Linescore,
            EnumExpandSchedule::Ticket,
        ];
        for expand in expands.into_iter() {
            schedule_client.get_schedule(
                expand,
                "",
                "2018-01-01".to_string(),
                "2018-01-01".to_string(),
            )?;
        }
    }

    {
        println!("Standings...");
        let standings_types = vec![
            EnumStandingTypes::ByConference,
            EnumStandingTypes::ByDivision,
            EnumStandingTypes::ByLeague,
            EnumStandingTypes::DivisionLeaders,
            // EnumStandingTypes::Postseason,  // neither of these types are working
            // EnumStandingTypes::Preseason,
            EnumStandingTypes::RegularSeason,
            EnumStandingTypes::WildCard,
            EnumStandingTypes::WildCardWithLeaders,
        ];
        standings_client.get_standing_types()?;
        standings_client.get_standings("".to_string(), "".to_string())?;
        for _type in standings_types.into_iter() {
            standings_client.get_standings_by_type(_type, "".to_string(), "".to_string())?;
        }
    }

    {
        println!("Stats...");
        stats_client.get_stat_types()?;
    }

    {
        println!("Teams...");
        let expands = vec![
            EnumExpandTeams::PersonNames,
            EnumExpandTeams::TeamRoster,
            EnumExpandTeams::TeamScheduleNext,
            EnumExpandTeams::TeamSchedulePrevious,
            EnumExpandTeams::TeamStats,
        ];
        for expand in expands.into_iter() {
            teams_client.get_team(1, expand.clone(), "20192020")?;
            teams_client.get_teams(expand, "20192020")?;
        }
        teams_client.get_team_roster(1, "20192020")?;
        teams_client.get_team_stats(1)?;
    }

    Ok(())
}
