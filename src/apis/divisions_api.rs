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

pub struct DivisionsApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> DivisionsApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> DivisionsApiClient<C> {
        DivisionsApiClient {
            configuration: configuration,
        }
    }
}

pub trait DivisionsApi {
    fn get_division(&self, id: f32) -> Box<Future<Item = crate::models::Division, Error = Error<serde_json::Value>>>;
    fn get_divisions(&self, ) -> Box<Future<Item = crate::models::Divisions, Error = Error<serde_json::Value>>>;
}


impl<C: hyper::client::Connect>DivisionsApi for DivisionsApiClient<C> {
    fn get_division(&self, id: f32) -> Box<Future<Item = crate::models::Division, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Get, "/divisions/{id}".to_string())
            .with_path_param("id".to_string(), id.to_string())
            .execute(self.configuration.borrow())
    }

    fn get_divisions(&self, ) -> Box<Future<Item = crate::models::Divisions, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Get, "/divisions".to_string())
            .execute(self.configuration.borrow())
    }

}
