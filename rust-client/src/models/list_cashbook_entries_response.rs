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

/// ListCashbookEntriesResponse : 
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListCashbookEntriesResponse {
    #[serde(rename = "data", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub data: Option<Option<Vec<models::CashbookEntry>>>,
}

impl ListCashbookEntriesResponse {
    /// 
    pub fn new() -> ListCashbookEntriesResponse {
        ListCashbookEntriesResponse {
            data: None,
        }
    }
}

