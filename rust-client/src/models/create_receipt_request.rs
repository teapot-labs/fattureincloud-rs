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

/// CreateReceiptRequest : 
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateReceiptRequest {
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Box<models::Receipt1>>,
    /// If true, the number is autocompleted progressively.
    #[serde(rename = "autocomplete_number", skip_serializing_if = "Option::is_none")]
    pub autocomplete_number: Option<bool>,
}

impl CreateReceiptRequest {
    /// 
    pub fn new() -> CreateReceiptRequest {
        CreateReceiptRequest {
            data: None,
            autocomplete_number: None,
        }
    }
}

