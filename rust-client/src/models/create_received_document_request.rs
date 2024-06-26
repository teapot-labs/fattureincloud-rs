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

/// CreateReceivedDocumentRequest : 
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateReceivedDocumentRequest {
    /// Pending received document id of the document from which the new document is created.
    #[serde(rename = "pending_id", skip_serializing_if = "Option::is_none")]
    pub pending_id: Option<i32>,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Box<models::ReceivedDocument1>>,
}

impl CreateReceivedDocumentRequest {
    /// 
    pub fn new() -> CreateReceivedDocumentRequest {
        CreateReceivedDocumentRequest {
            pending_id: None,
            data: None,
        }
    }
}

