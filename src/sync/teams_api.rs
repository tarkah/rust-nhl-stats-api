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

pub struct TeamsApiClient {
    configuration: Rc<configuration::Configuration>,
}

impl TeamsApiClient {
    pub fn new(configuration: Rc<configuration::Configuration>) -> TeamsApiClient {
        TeamsApiClient {
            configuration: configuration,
        }
    }
}

pub trait TeamsApi {
    fn get_team(&self, id: f32, expand: &str, season: f32) -> Result<crate::models::Team, Error>;
    fn get_team_roster(&self, id: f32, season: f32) -> Result<crate::models::Rosters, Error>;
    fn get_team_stats(&self, id: f32) -> Result<crate::models::TeamStats, Error>;
    fn get_teams(&self, expand: &str, season: f32) -> Result<crate::models::Teams, Error>;
}

impl TeamsApi for TeamsApiClient {
    fn get_team(&self, id: f32, expand: &str, season: f32) -> Result<crate::models::Team, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/teams/{id}", configuration.base_path, id=id);
        let mut req_builder = client.get(uri_str.as_str());

        req_builder = req_builder.query(&[("expand", &expand.to_string())]);
        req_builder = req_builder.query(&[("season", &season.to_string())]);
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn get_team_roster(&self, id: f32, season: f32) -> Result<crate::models::Rosters, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/teams/{id}/roster", configuration.base_path, id=id);
        let mut req_builder = client.get(uri_str.as_str());

        req_builder = req_builder.query(&[("season", &season.to_string())]);
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn get_team_stats(&self, id: f32) -> Result<crate::models::TeamStats, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/teams/{id}/stats", configuration.base_path, id=id);
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn get_teams(&self, expand: &str, season: f32) -> Result<crate::models::Teams, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/teams", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        req_builder = req_builder.query(&[("expand", &expand.to_string())]);
        req_builder = req_builder.query(&[("season", &season.to_string())]);
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

}