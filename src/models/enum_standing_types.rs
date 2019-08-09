/*
 * NHL API
 *
 * Documenting the publicly accessible portions of the NHL API.
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */


/// 
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum EnumStandingTypes {
    #[serde(rename = "byConference")]
    ByConference,
    #[serde(rename = "byDivision")]
    ByDivision,
    #[serde(rename = "byLeague")]
    ByLeague,
    #[serde(rename = "divisionLeaders")]
    DivisionLeaders,
    #[serde(rename = "postseason")]
    Postseason,
    #[serde(rename = "preseason")]
    Preseason,
    #[serde(rename = "regularSeason")]
    RegularSeason,
    #[serde(rename = "wildCard")]
    WildCard,
    #[serde(rename = "wildCardWithLeaders")]
    WildCardWithLeaders,

}



