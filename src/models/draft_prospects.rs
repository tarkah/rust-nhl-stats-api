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
pub struct DraftProspects {
    #[serde(rename = "copyright", skip_serializing_if = "Option::is_none")]
    pub copyright: Option<String>,
    #[serde(rename = "prospects", skip_serializing_if = "Option::is_none")]
    pub prospects: Option<Vec<crate::models::DraftProspect>>,
}

impl DraftProspects {
    pub fn new() -> DraftProspects {
        DraftProspects {
            copyright: None,
            prospects: None,
        }
    }
}


