use std::rc::Rc;

use hyper;
use super::configuration::Configuration;

pub struct APIClient {
    conferences_api: Box<crate::async::ConferencesApi>,
    divisions_api: Box<crate::async::DivisionsApi>,
    draft_api: Box<crate::async::DraftApi>,
    games_api: Box<crate::async::GamesApi>,
    players_api: Box<crate::async::PlayersApi>,
    schedule_api: Box<crate::async::ScheduleApi>,
    standings_api: Box<crate::async::StandingsApi>,
    stats_api: Box<crate::async::StatsApi>,
    teams_api: Box<crate::async::TeamsApi>,
}

impl APIClient {
    pub fn new<C: hyper::client::Connect>(configuration: Configuration<C>) -> APIClient {
        let rc = Rc::new(configuration);

        APIClient {
            conferences_api: Box::new(crate::async::ConferencesApiClient::new(rc.clone())),
            divisions_api: Box::new(crate::async::DivisionsApiClient::new(rc.clone())),
            draft_api: Box::new(crate::async::DraftApiClient::new(rc.clone())),
            games_api: Box::new(crate::async::GamesApiClient::new(rc.clone())),
            players_api: Box::new(crate::async::PlayersApiClient::new(rc.clone())),
            schedule_api: Box::new(crate::async::ScheduleApiClient::new(rc.clone())),
            standings_api: Box::new(crate::async::StandingsApiClient::new(rc.clone())),
            stats_api: Box::new(crate::async::StatsApiClient::new(rc.clone())),
            teams_api: Box::new(crate::async::TeamsApiClient::new(rc.clone())),
        }
    }

    pub fn conferences_api(&self) -> &crate::async::ConferencesApi{
        self.conferences_api.as_ref()
    }

    pub fn divisions_api(&self) -> &crate::async::DivisionsApi{
        self.divisions_api.as_ref()
    }

    pub fn draft_api(&self) -> &crate::async::DraftApi{
        self.draft_api.as_ref()
    }

    pub fn games_api(&self) -> &crate::async::GamesApi{
        self.games_api.as_ref()
    }

    pub fn players_api(&self) -> &crate::async::PlayersApi{
        self.players_api.as_ref()
    }

    pub fn schedule_api(&self) -> &crate::async::ScheduleApi{
        self.schedule_api.as_ref()
    }

    pub fn standings_api(&self) -> &crate::async::StandingsApi{
        self.standings_api.as_ref()
    }

    pub fn stats_api(&self) -> &crate::async::StatsApi{
        self.stats_api.as_ref()
    }

    pub fn teams_api(&self) -> &crate::async::TeamsApi{
        self.teams_api.as_ref()
    }

}
