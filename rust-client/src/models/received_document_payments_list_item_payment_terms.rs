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
pub struct ReceivedDocumentPaymentsListItemPaymentTerms {
    /// Received document payment number of days by which the payment must be made
    #[serde(rename = "days", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub days: Option<Option<i32>>,
    /// Payment terms type
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
}

impl ReceivedDocumentPaymentsListItemPaymentTerms {
    pub fn new() -> ReceivedDocumentPaymentsListItemPaymentTerms {
        ReceivedDocumentPaymentsListItemPaymentTerms {
            days: None,
            r#type: None,
        }
    }
}
/// Payment terms type
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "standard")]
    Standard,
    #[serde(rename = "end_of_month")]
    EndOfMonth,
}

impl Default for Type {
    fn default() -> Type {
        Self::Standard
    }
}

