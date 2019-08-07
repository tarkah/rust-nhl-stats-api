use std::rc::Rc;

use hyper;
use super::configuration::Configuration;

pub struct APIClient {
    conferences_api: Box<crate::apis::ConferencesApi>,
    divisions_api: Box<crate::apis::DivisionsApi>,
    draft_api: Box<crate::apis::DraftApi>,
    games_api: Box<crate::apis::GamesApi>,
    players_api: Box<crate::apis::PlayersApi>,
    schedule_api: Box<crate::apis::ScheduleApi>,
    standings_api: Box<crate::apis::StandingsApi>,
    stats_api: Box<crate::apis::StatsApi>,
    teams_api: Box<crate::apis::TeamsApi>,
}

impl APIClient {
    pub fn new<C: hyper::client::Connect>(configuration: Configuration<C>) -> APIClient {
        let rc = Rc::new(configuration);

        APIClient {
            conferences_api: Box::new(crate::apis::ConferencesApiClient::new(rc.clone())),
            divisions_api: Box::new(crate::apis::DivisionsApiClient::new(rc.clone())),
            draft_api: Box::new(crate::apis::DraftApiClient::new(rc.clone())),
            games_api: Box::new(crate::apis::GamesApiClient::new(rc.clone())),
            players_api: Box::new(crate::apis::PlayersApiClient::new(rc.clone())),
            schedule_api: Box::new(crate::apis::ScheduleApiClient::new(rc.clone())),
            standings_api: Box::new(crate::apis::StandingsApiClient::new(rc.clone())),
            stats_api: Box::new(crate::apis::StatsApiClient::new(rc.clone())),
            teams_api: Box::new(crate::apis::TeamsApiClient::new(rc.clone())),
        }
    }

    pub fn conferences_api(&self) -> &crate::apis::ConferencesApi{
        self.conferences_api.as_ref()
    }

    pub fn divisions_api(&self) -> &crate::apis::DivisionsApi{
        self.divisions_api.as_ref()
    }

    pub fn draft_api(&self) -> &crate::apis::DraftApi{
        self.draft_api.as_ref()
    }

    pub fn games_api(&self) -> &crate::apis::GamesApi{
        self.games_api.as_ref()
    }

    pub fn players_api(&self) -> &crate::apis::PlayersApi{
        self.players_api.as_ref()
    }

    pub fn schedule_api(&self) -> &crate::apis::ScheduleApi{
        self.schedule_api.as_ref()
    }

    pub fn standings_api(&self) -> &crate::apis::StandingsApi{
        self.standings_api.as_ref()
    }

    pub fn stats_api(&self) -> &crate::apis::StatsApi{
        self.stats_api.as_ref()
    }

    pub fn teams_api(&self) -> &crate::apis::TeamsApi{
        self.teams_api.as_ref()
    }

}
