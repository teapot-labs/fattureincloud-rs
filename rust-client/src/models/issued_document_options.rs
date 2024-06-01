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
pub struct IssuedDocumentOptions {
    /// Fixes your last payment amount to match your document total
    #[serde(rename = "fix_payments", skip_serializing_if = "Option::is_none")]
    pub fix_payments: Option<bool>,
    /// Original documents ids [only for join/transform]
    #[serde(rename = "create_from", skip_serializing_if = "Option::is_none")]
    pub create_from: Option<Vec<String>>,
    /// Tranform a document [only for transform]
    #[serde(rename = "transform", skip_serializing_if = "Option::is_none")]
    pub transform: Option<bool>,
    /// Keep original document [only for transform]
    #[serde(rename = "keep_copy", skip_serializing_if = "Option::is_none")]
    pub keep_copy: Option<bool>,
    /// Join type [only for join]
    #[serde(rename = "join_type", skip_serializing_if = "Option::is_none")]
    pub join_type: Option<String>,
}

impl IssuedDocumentOptions {
    pub fn new() -> IssuedDocumentOptions {
        IssuedDocumentOptions {
            fix_payments: None,
            create_from: None,
            transform: None,
            keep_copy: None,
            join_type: None,
        }
    }
}

