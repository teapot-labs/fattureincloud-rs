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
pub struct ReceivedDocumentPaymentsListItem {
    /// Received document payment id
    #[serde(rename = "id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub id: Option<Option<i32>>,
    /// Received document payment total amount
    #[serde(rename = "amount", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub amount: Option<Option<f64>>,
    /// Due date
    #[serde(rename = "due_date", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub due_date: Option<Option<String>>,
    /// Received document payment paid date
    #[serde(rename = "paid_date", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub paid_date: Option<Option<String>>,
    #[serde(rename = "payment_terms", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub payment_terms: Option<Option<Box<models::ReceivedDocumentPaymentsListItemPaymentTerms>>>,
    /// Received document payment status
    #[serde(rename = "status", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub status: Option<Option<String>>,
    #[serde(rename = "payment_account", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub payment_account: Option<Option<Box<models::PaymentAccount>>>,
}

impl ReceivedDocumentPaymentsListItem {
    pub fn new() -> ReceivedDocumentPaymentsListItem {
        ReceivedDocumentPaymentsListItem {
            id: None,
            amount: None,
            due_date: None,
            paid_date: None,
            payment_terms: None,
            status: None,
            payment_account: None,
        }
    }
}

