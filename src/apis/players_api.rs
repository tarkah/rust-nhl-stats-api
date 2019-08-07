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

pub struct PlayersApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> PlayersApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> PlayersApiClient<C> {
        PlayersApiClient {
            configuration: configuration,
        }
    }
}

pub trait PlayersApi {
    fn get_player(&self, id: f32) -> Box<Future<Item = crate::models::Players, Error = Error<serde_json::Value>>>;
    fn get_player_stats(&self, id: f32, stats: &str, season: f32) -> Box<Future<Item = crate::models::PlayerStats, Error = Error<serde_json::Value>>>;
}


impl<C: hyper::client::Connect>PlayersApi for PlayersApiClient<C> {
    fn get_player(&self, id: f32) -> Box<Future<Item = crate::models::Players, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Get, "/people/{id}".to_string())
            .with_path_param("id".to_string(), id.to_string())
            .execute(self.configuration.borrow())
    }

    fn get_player_stats(&self, id: f32, stats: &str, season: f32) -> Box<Future<Item = crate::models::PlayerStats, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Get, "/people/{id}/stats".to_string())
            .with_query_param("stats".to_string(), stats.to_string())
            .with_query_param("season".to_string(), season.to_string())
            .with_path_param("id".to_string(), id.to_string())
            .execute(self.configuration.borrow())
    }

}
