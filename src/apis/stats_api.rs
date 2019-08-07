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

pub struct StatsApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> StatsApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> StatsApiClient<C> {
        StatsApiClient {
            configuration: configuration,
        }
    }
}

pub trait StatsApi {
    fn get_stat_types(&self, ) -> Box<Future<Item = Vec<serde_json::Value>, Error = Error<serde_json::Value>>>;
}


impl<C: hyper::client::Connect>StatsApi for StatsApiClient<C> {
    fn get_stat_types(&self, ) -> Box<Future<Item = Vec<serde_json::Value>, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Get, "/statTypes".to_string())
            .execute(self.configuration.borrow())
    }

}