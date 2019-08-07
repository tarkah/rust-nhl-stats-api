use hyper;
use serde;
use serde_json;

#[derive(Debug)]
pub enum Error<T> {
    UriError(hyper::error::UriError),
    Hyper(hyper::Error),
    Serde(serde_json::Error),
    ApiError(ApiError<T>),
}

#[derive(Debug)]
pub struct ApiError<T> {
    pub code: hyper::StatusCode,
    pub content: Option<T>,
}

impl<'de, T> From<(hyper::StatusCode, &'de [u8])> for Error<T> 
    where T: serde::Deserialize<'de> {
    fn from(e: (hyper::StatusCode, &'de [u8])) -> Self {
        if e.1.len() == 0 {
            return Error::ApiError(ApiError{
                code: e.0,
                content: None,
            });
        }
        match serde_json::from_slice::<T>(e.1) {
            Ok(t) => Error::ApiError(ApiError{
                code: e.0,
                content: Some(t),
            }),
            Err(e) => {
                Error::from(e)
            }
        }
    }
}

impl<T> From<hyper::Error> for Error<T> {
    fn from(e: hyper::Error) -> Self {
        return Error::Hyper(e)
    }
}

impl<T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        return Error::Serde(e)
    }
}

mod request;

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
