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

pub struct StandingsApiClient {
    configuration: Rc<configuration::Configuration>,
}

impl StandingsApiClient {
    pub fn new(configuration: Rc<configuration::Configuration>) -> StandingsApiClient {
        StandingsApiClient {
            configuration: configuration,
        }
    }
}

pub trait StandingsApi {
    fn get_standing_types(&self, ) -> Result<crate::models::StandingTypes, Error>;
    fn get_standings(&self, season: String, date: String) -> Result<crate::models::Standings, Error>;
    fn get_standings_by_type(&self, _type: crate::models::EnumStandingTypes, date: String, season: String) -> Result<crate::models::Standings, Error>;
}

impl StandingsApi for StandingsApiClient {
    fn get_standing_types(&self, ) -> Result<crate::models::StandingTypes, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/standingsTypes", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn get_standings(&self, season: String, date: String) -> Result<crate::models::Standings, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/standings", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        req_builder = req_builder.query(&[("season", &season.to_string())]);
        req_builder = req_builder.query(&[("date", &date.to_string())]);
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn get_standings_by_type(&self, _type: crate::models::EnumStandingTypes, date: String, season: String) -> Result<crate::models::Standings, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/standings/{type}", configuration.base_path, type=_type);
        let mut req_builder = client.get(uri_str.as_str());

        req_builder = req_builder.query(&[("date", &date.to_string())]);
        req_builder = req_builder.query(&[("season", &season.to_string())]);
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

}
