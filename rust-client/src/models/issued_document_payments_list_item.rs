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
pub struct IssuedDocumentPaymentsListItem {
    /// Issued document payment item id
    #[serde(rename = "id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub id: Option<Option<i32>>,
    /// Issued document payment due date
    #[serde(rename = "due_date", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub due_date: Option<Option<String>>,
    /// Issued document payment amount
    #[serde(rename = "amount", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub amount: Option<Option<f64>>,
    /// Issued document status
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    #[serde(rename = "payment_account", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub payment_account: Option<Option<Box<models::PaymentAccount>>>,
    /// Issued document payment date [Only if status is paid]
    #[serde(rename = "paid_date", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub paid_date: Option<Option<String>>,
    /// Issued document payment advanced raw attributes for e-invoices
    #[serde(rename = "ei_raw", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub ei_raw: Option<Option<serde_json::Value>>,
    #[serde(rename = "payment_terms", skip_serializing_if = "Option::is_none")]
    pub payment_terms: Option<Box<models::IssuedDocumentPaymentsListItemPaymentTerms>>,
}

impl IssuedDocumentPaymentsListItem {
    pub fn new() -> IssuedDocumentPaymentsListItem {
        IssuedDocumentPaymentsListItem {
            id: None,
            due_date: None,
            amount: None,
            status: None,
            payment_account: None,
            paid_date: None,
            ei_raw: None,
            payment_terms: None,
        }
    }
}
/// Issued document status
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "not_paid")]
    NotPaid,
    #[serde(rename = "paid")]
    Paid,
    #[serde(rename = "reversed")]
    Reversed,
}

impl Default for Status {
    fn default() -> Status {
        Self::NotPaid
    }
}

