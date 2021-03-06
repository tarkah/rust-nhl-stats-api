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
pub struct GameEditorialTokenData {
    #[serde(rename = "tokenGUID", skip_serializing_if = "Option::is_none")]
    pub token_guid: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<Type>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "teamId", skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "seoName", skip_serializing_if = "Option::is_none")]
    pub seo_name: Option<String>,
    #[serde(rename = "href", skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    #[serde(rename = "hrefMobile", skip_serializing_if = "Option::is_none")]
    pub href_mobile: Option<String>,
}

impl GameEditorialTokenData {
    pub fn new() -> GameEditorialTokenData {
        GameEditorialTokenData {
            token_guid: None,
            _type: None,
            id: None,
            team_id: None,
            name: None,
            seo_name: None,
            href: None,
            href_mobile: None,
        }
    }
}

/// 
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "hyperLink")]
    HyperLink,
    #[serde(rename = "playerCard")]
    PlayerCard,
}

