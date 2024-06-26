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
pub struct Currency {
    /// Currency code
    #[serde(rename = "id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub id: Option<Option<String>>,
    /// Currency symbol
    #[serde(rename = "symbol", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Option<String>>,
    /// Currency exchange rate (EUR to this)
    #[serde(rename = "exchange_rate", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub exchange_rate: Option<Option<String>>,
    /// Currency html code
    #[serde(rename = "html_symbol", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub html_symbol: Option<Option<String>>,
}

impl Currency {
    pub fn new() -> Currency {
        Currency {
            id: None,
            symbol: None,
            exchange_rate: None,
            html_symbol: None,
        }
    }
}

