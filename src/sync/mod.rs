use reqwest;
use serde_json;

#[derive(Debug)]
pub enum Error {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    Io(std::io::Error),
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    ::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}

mod conferences_api;
pub use self::conferences_api::{ ConferencesApi, ConferencesApiClient };
mod divisions_api;
pub use self::divisions_api::{ DivisionsApi, DivisionsApiClient };
mod draft_api;
pub use self::draft_api::{ DraftApi, DraftApiClient };
mod games_api;
pub use self::games_api::{ GamesApi, GamesApiClient };
mod players_api;
pub use self::players_api::{ PlayersApi, PlayersApiClient };
mod schedule_api;
pub use self::schedule_api::{ ScheduleApi, ScheduleApiClient };
mod standings_api;
pub use self::standings_api::{ StandingsApi, StandingsApiClient };
mod stats_api;
pub use self::stats_api::{ StatsApi, StatsApiClient };
mod teams_api;
pub use self::teams_api::{ TeamsApi, TeamsApiClient };

pub mod configuration;
pub mod client;
