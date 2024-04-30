/*
 * Fatture in Cloud API v2 - API Reference
 *
 * Connect your software with Fatture in Cloud, the invoicing platform chosen by more than 500.000 businesses in Italy.   The Fatture in Cloud API is based on REST, and makes possible to interact with the user related data prior authorization via OAuth2 protocol.
 *
 * The version of the OpenAPI document: 2.0.32
 * Contact: info@fattureincloud.it
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct City {
    /// City postal code
    #[serde(rename = "postal_code", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<Option<String>>,
    /// City name
    #[serde(rename = "city", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub city: Option<Option<String>>,
    /// City province
    #[serde(rename = "province", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub province: Option<Option<String>>,
}

impl City {
    pub fn new() -> City {
        City {
            postal_code: None,
            city: None,
            province: None,
        }
    }
}

