/*
 * NHL API
 *
 * Documenting the publicly accessible portions of the NHL API.
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use std::rc::Rc;
use std::borrow::Borrow;

use hyper;
use serde_json;
use futures::Future;

use super::{Error, configuration};
use super::request as __internal_request;

pub struct StandingsApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> StandingsApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> StandingsApiClient<C> {
        StandingsApiClient {
            configuration: configuration,
        }
    }
}

pub trait StandingsApi {
    fn get_standing_types(&self, ) -> Box<Future<Item = crate::models::StandingTypes, Error = Error<serde_json::Value>>>;
    fn get_standings(&self, season: String, date: String) -> Box<Future<Item = crate::models::Standings, Error = Error<serde_json::Value>>>;
    fn get_standings_by_type(&self, _type: crate::models::EnumStandingTypes, date: String, season: String) -> Box<Future<Item = crate::models::Standings, Error = Error<serde_json::Value>>>;
}


impl<C: hyper::client::Connect>StandingsApi for StandingsApiClient<C> {
    fn get_standing_types(&self, ) -> Box<Future<Item = crate::models::StandingTypes, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Get, "/standingsTypes".to_string())
            .execute(self.configuration.borrow())
    }

    fn get_standings(&self, season: String, date: String) -> Box<Future<Item = crate::models::Standings, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Get, "/standings".to_string())
            .with_query_param("season".to_string(), season.to_string())
            .with_query_param("date".to_string(), date.to_string())
            .execute(self.configuration.borrow())
    }

    fn get_standings_by_type(&self, _type: crate::models::EnumStandingTypes, date: String, season: String) -> Box<Future<Item = crate::models::Standings, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Get, "/standings/{type}".to_string())
            .with_query_param("date".to_string(), date.to_string())
            .with_query_param("season".to_string(), season.to_string())
            .with_path_param("type".to_string(), _type.to_string())
            .execute(self.configuration.borrow())
    }

}
