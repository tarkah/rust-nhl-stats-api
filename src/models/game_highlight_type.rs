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
pub struct GameHighlightType {
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "topicList", skip_serializing_if = "Option::is_none")]
    pub topic_list: Option<String>,
    #[serde(rename = "items", skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<crate::models::GameHighlight>>,
}

impl GameHighlightType {
    pub fn new() -> GameHighlightType {
        GameHighlightType {
            title: None,
            topic_list: None,
            items: None,
        }
    }
}


