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

use reqwest;

use super::{Error, configuration};

pub struct ScheduleApiClient {
    configuration: Rc<configuration::Configuration>,
}

impl ScheduleApiClient {
    pub fn new(configuration: Rc<configuration::Configuration>) -> ScheduleApiClient {
        ScheduleApiClient {
            configuration: configuration,
        }
    }
}

pub trait ScheduleApi {
    fn get_schedule(&self, expand: crate::models::EnumExpandSchedule, team_id: &str, start_date: String, end_date: String) -> Result<crate::models::Schedule, Error>;
}

impl ScheduleApi for ScheduleApiClient {
    fn get_schedule(&self, expand: crate::models::EnumExpandSchedule, team_id: &str, start_date: String, end_date: String) -> Result<crate::models::Schedule, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/schedule", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        req_builder = req_builder.query(&[("expand", &expand.to_string())]);
        req_builder = req_builder.query(&[("teamId", &team_id.to_string())]);
        req_builder = req_builder.query(&[("startDate", &start_date.to_string())]);
        req_builder = req_builder.query(&[("endDate", &end_date.to_string())]);
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

}
