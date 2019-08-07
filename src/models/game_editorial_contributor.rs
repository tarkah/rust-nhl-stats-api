/*
 * NHL API
 *
 * Documenting the publicly accessible portions of the NHL API.
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */



#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GameEditorialContributor {
    #[serde(rename = "contributors", skip_serializing_if = "Option::is_none")]
    pub contributors: Option<Vec<crate::models::GameEditorialContributorContributors>>,
    #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

impl GameEditorialContributor {
    pub fn new() -> GameEditorialContributor {
        GameEditorialContributor {
            contributors: None,
            source: None,
        }
    }
}

