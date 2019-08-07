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

pub struct TeamsApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> TeamsApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> TeamsApiClient<C> {
        TeamsApiClient {
            configuration: configuration,
        }
    }
}

pub trait TeamsApi {
    fn get_team(&self, id: f32, expand: &str, season: f32) -> Box<Future<Item = crate::models::Team, Error = Error<serde_json::Value>>>;
    fn get_team_roster(&self, id: f32, season: f32) -> Box<Future<Item = crate::models::Rosters, Error = Error<serde_json::Value>>>;
    fn get_team_stats(&self, id: f32) -> Box<Future<Item = crate::models::TeamStats, Error = Error<serde_json::Value>>>;
    fn get_teams(&self, expand: &str, season: f32) -> Box<Future<Item = crate::models::Teams, Error = Error<serde_json::Value>>>;
}


impl<C: hyper::client::Connect>TeamsApi for TeamsApiClient<C> {
    fn get_team(&self, id: f32, expand: &str, season: f32) -> Box<Future<Item = crate::models::Team, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Get, "/teams/{id}".to_string())
            .with_query_param("expand".to_string(), expand.to_string())
            .with_query_param("season".to_string(), season.to_string())
            .with_path_param("id".to_string(), id.to_string())
            .execute(self.configuration.borrow())
    }

    fn get_team_roster(&self, id: f32, season: f32) -> Box<Future<Item = crate::models::Rosters, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Get, "/teams/{id}/roster".to_string())
            .with_query_param("season".to_string(), season.to_string())
            .with_path_param("id".to_string(), id.to_string())
            .execute(self.configuration.borrow())
    }

    fn get_team_stats(&self, id: f32) -> Box<Future<Item = crate::models::TeamStats, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Get, "/teams/{id}/stats".to_string())
            .with_path_param("id".to_string(), id.to_string())
            .execute(self.configuration.borrow())
    }

    fn get_teams(&self, expand: &str, season: f32) -> Box<Future<Item = crate::models::Teams, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Get, "/teams".to_string())
            .with_query_param("expand".to_string(), expand.to_string())
            .with_query_param("season".to_string(), season.to_string())
            .execute(self.configuration.borrow())
    }

}