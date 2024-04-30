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
pub struct CashbookEntry {
    /// Cashbook id
    #[serde(rename = "id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub id: Option<Option<String>>,
    /// Cashbook date
    #[serde(rename = "date", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub date: Option<Option<String>>,
    /// Cashbook description
    #[serde(rename = "description", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
    /// Cashbook kind
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<Kind>,
    /// Cashbook type
    #[serde(rename = "type", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Option<Type>>,
    /// Cashbook entity name
    #[serde(rename = "entity_name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub entity_name: Option<Option<String>>,
    #[serde(rename = "document", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub document: Option<Option<Box<models::CashbookEntryDocument>>>,
    /// [Only for cashbook entry in] Cashbook total amount in
    #[serde(rename = "amount_in", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub amount_in: Option<Option<f64>>,
    #[serde(rename = "payment_account_in", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub payment_account_in: Option<Option<Box<models::PaymentAccount3>>>,
    /// [Only for cashbook entry out] Cashbook total amount out
    #[serde(rename = "amount_out", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub amount_out: Option<Option<f64>>,
    #[serde(rename = "payment_account_out", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub payment_account_out: Option<Option<Box<models::PaymentAccount4>>>,
}

impl CashbookEntry {
    pub fn new() -> CashbookEntry {
        CashbookEntry {
            id: None,
            date: None,
            description: None,
            kind: None,
            r#type: None,
            entity_name: None,
            document: None,
            amount_in: None,
            payment_account_in: None,
            amount_out: None,
            payment_account_out: None,
        }
    }
}
/// Cashbook kind
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Kind {
    #[serde(rename = "cashbook")]
    Cashbook,
    #[serde(rename = "issued_document")]
    IssuedDocument,
    #[serde(rename = "received_document")]
    ReceivedDocument,
    #[serde(rename = "tax")]
    Tax,
    #[serde(rename = "receipt")]
    Receipt,
}

impl Default for Kind {
    fn default() -> Kind {
        Self::Cashbook
    }
}
/// Cashbook type
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "in")]
    In,
    #[serde(rename = "out")]
    Out,
}

impl Default for Type {
    fn default() -> Type {
        Self::In
    }
}

