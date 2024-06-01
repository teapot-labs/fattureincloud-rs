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
pub struct WebhooksSubscriptionConfig1 {
    /// Determines the webhook payload shape
    #[serde(rename = "mapping", skip_serializing_if = "Option::is_none")]
    pub mapping: Option<Mapping>,
}

impl WebhooksSubscriptionConfig1 {
    pub fn new() -> WebhooksSubscriptionConfig1 {
        WebhooksSubscriptionConfig1 {
            mapping: None,
        }
    }
}
/// Determines the webhook payload shape
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Mapping {
    #[serde(rename = "binary")]
    Binary,
    #[serde(rename = "structured")]
    Structured,
}

impl Default for Mapping {
    fn default() -> Mapping {
        Self::Binary
    }
}

