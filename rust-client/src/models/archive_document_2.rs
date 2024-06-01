/*
 * Fatture in Cloud API v2 - API Reference
 *
 * Connect your software with Fatture in Cloud, the invoicing platform chosen by more than 500.000 businesses in Italy.   The Fatture in Cloud API is based on REST, and makes possible to interact with the user related data prior authorization via OAuth2 protocol.
 *
 * The version of the OpenAPI document: 2.0.33
 * Contact: info@fattureincloud.it
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ArchiveDocument2 {
    /// Archive document id
    #[serde(rename = "id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub id: Option<Option<i32>>,
    /// Archive document date
    #[serde(rename = "date", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub date: Option<Option<String>>,
    /// Archive Document description
    #[serde(rename = "description", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
    /// [Temporary] [Read Only] Archive Document url of the attached file
    #[serde(rename = "attachment_url", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub attachment_url: Option<Option<String>>,
    /// Archive document category
    #[serde(rename = "category", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub category: Option<Option<String>>,
    /// [Write Only]  [Required] Archive document attachment token returned by POST /archive/attachment
    #[serde(rename = "attachment_token", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub attachment_token: Option<Option<String>>,
}

impl ArchiveDocument2 {
    pub fn new() -> ArchiveDocument2 {
        ArchiveDocument2 {
            id: None,
            date: None,
            description: None,
            attachment_url: None,
            category: None,
            attachment_token: None,
        }
    }
}

