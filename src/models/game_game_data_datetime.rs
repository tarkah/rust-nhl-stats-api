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
pub struct GameGameDataDatetime {
    #[serde(rename = "dateTime", skip_serializing_if = "Option::is_none")]
    pub date_time: Option<String>,
    #[serde(rename = "endDateTime", skip_serializing_if = "Option::is_none")]
    pub end_date_time: Option<String>,
}

impl GameGameDataDatetime {
    pub fn new() -> GameGameDataDatetime {
        GameGameDataDatetime {
            date_time: None,
            end_date_time: None,
        }
    }
}


