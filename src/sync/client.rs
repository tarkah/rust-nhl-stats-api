use std::rc::Rc;

use super::configuration::Configuration;

pub struct APIClient {
    conferences_api: Box<crate::sync::ConferencesApi>,
    divisions_api: Box<crate::sync::DivisionsApi>,
    draft_api: Box<crate::sync::DraftApi>,
    games_api: Box<crate::sync::GamesApi>,
    players_api: Box<crate::sync::PlayersApi>,
    schedule_api: Box<crate::sync::ScheduleApi>,
    standings_api: Box<crate::sync::StandingsApi>,
    stats_api: Box<crate::sync::StatsApi>,
    teams_api: Box<crate::sync::TeamsApi>,
}

impl APIClient {
    pub fn new(configuration: Configuration) -> APIClient {
        let rc = Rc::new(configuration);

        APIClient {
            conferences_api: Box::new(crate::sync::ConferencesApiClient::new(rc.clone())),
            divisions_api: Box::new(crate::sync::DivisionsApiClient::new(rc.clone())),
            draft_api: Box::new(crate::sync::DraftApiClient::new(rc.clone())),
            games_api: Box::new(crate::sync::GamesApiClient::new(rc.clone())),
            players_api: Box::new(crate::sync::PlayersApiClient::new(rc.clone())),
            schedule_api: Box::new(crate::sync::ScheduleApiClient::new(rc.clone())),
            standings_api: Box::new(crate::sync::StandingsApiClient::new(rc.clone())),
            stats_api: Box::new(crate::sync::StatsApiClient::new(rc.clone())),
            teams_api: Box::new(crate::sync::TeamsApiClient::new(rc.clone())),
        }
    }

    pub fn conferences_api(&self) -> &crate::sync::ConferencesApi{
        self.conferences_api.as_ref()
    }

    pub fn divisions_api(&self) -> &crate::sync::DivisionsApi{
        self.divisions_api.as_ref()
    }

    pub fn draft_api(&self) -> &crate::sync::DraftApi{
        self.draft_api.as_ref()
    }

    pub fn games_api(&self) -> &crate::sync::GamesApi{
        self.games_api.as_ref()
    }

    pub fn players_api(&self) -> &crate::sync::PlayersApi{
        self.players_api.as_ref()
    }

    pub fn schedule_api(&self) -> &crate::sync::ScheduleApi{
        self.schedule_api.as_ref()
    }

    pub fn standings_api(&self) -> &crate::sync::StandingsApi{
        self.standings_api.as_ref()
    }

    pub fn stats_api(&self) -> &crate::sync::StatsApi{
        self.stats_api.as_ref()
    }

    pub fn teams_api(&self) -> &crate::sync::TeamsApi{
        self.teams_api.as_ref()
    }

}
