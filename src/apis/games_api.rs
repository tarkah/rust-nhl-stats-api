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

pub struct GamesApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> GamesApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> GamesApiClient<C> {
        GamesApiClient {
            configuration: configuration,
        }
    }
}

pub trait GamesApi {
    fn get_game(&self, id: f32) -> Box<Future<Item = crate::models::Game, Error = Error<serde_json::Value>>>;
    fn get_game_boxscore(&self, id: f32) -> Box<Future<Item = crate::models::GameBoxscores, Error = Error<serde_json::Value>>>;
    fn get_game_content(&self, id: f32) -> Box<Future<Item = crate::models::GameContent, Error = Error<serde_json::Value>>>;
    fn get_game_diff(&self, id: f32, start_time_code: &str) -> Box<Future<Item = crate::models::Game, Error = Error<serde_json::Value>>>;
}


impl<C: hyper::client::Connect>GamesApi for GamesApiClient<C> {
    fn get_game(&self, id: f32) -> Box<Future<Item = crate::models::Game, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Get, "/game/{id}/feed/live".to_string())
            .with_path_param("id".to_string(), id.to_string())
            .execute(self.configuration.borrow())
    }

    fn get_game_boxscore(&self, id: f32) -> Box<Future<Item = crate::models::GameBoxscores, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Get, "/game/{id}/boxscore".to_string())
            .with_path_param("id".to_string(), id.to_string())
            .execute(self.configuration.borrow())
    }

    fn get_game_content(&self, id: f32) -> Box<Future<Item = crate::models::GameContent, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Get, "/game/{id}/content".to_string())
            .with_path_param("id".to_string(), id.to_string())
            .execute(self.configuration.borrow())
    }

    fn get_game_diff(&self, id: f32, start_time_code: &str) -> Box<Future<Item = crate::models::Game, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Get, "/game/{id}/feed/live/diffPatch".to_string())
            .with_query_param("startTimeCode".to_string(), start_time_code.to_string())
            .with_path_param("id".to_string(), id.to_string())
            .execute(self.configuration.borrow())
    }

}
