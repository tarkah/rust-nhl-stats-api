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
pub struct GameEditorialKeyword {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<Type>,
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}

impl GameEditorialKeyword {
    pub fn new() -> GameEditorialKeyword {
        GameEditorialKeyword {
            _type: None,
            value: None,
            display_name: None,
        }
    }
}

/// 
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "bodyParagraphCount")]
    BodyParagraphCount,
    #[serde(rename = "bodyWordCount")]
    BodyWordCount,
    #[serde(rename = "clob_autoTagSkip_playerCards")]
    ClobAutoTagSkipPlayerCards,
    #[serde(rename = "content")]
    Content,
    #[serde(rename = "embeddable")]
    Embeddable,
    #[serde(rename = "gameId")]
    GameId,
    #[serde(rename = "language")]
    Language,
    #[serde(rename = "playerId")]
    PlayerId,
    #[serde(rename = "previewParagraphCount")]
    PreviewParagraphCount,
    #[serde(rename = "primaryTag")]
    PrimaryTag,
    #[serde(rename = "previewWordCount")]
    PreviewWordCount,
    #[serde(rename = "shareable")]
    Shareable,
    #[serde(rename = "statsEventId")]
    StatsEventId,
    #[serde(rename = "teamFileCode")]
    TeamFileCode,
    #[serde(rename = "teamId")]
    TeamId,
}

