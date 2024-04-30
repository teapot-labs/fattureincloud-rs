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

/// PaymentMethod4 : [Only for client] Client default payment method
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PaymentMethod4 {
    /// Payment method id
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// Payment method name
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Payment method type
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
    /// Payment method is default
    #[serde(rename = "is_default", skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    #[serde(rename = "default_payment_account", skip_serializing_if = "Option::is_none")]
    pub default_payment_account: Option<Box<models::PaymentAccount2>>,
    /// Payment method details
    #[serde(rename = "details", skip_serializing_if = "Option::is_none")]
    pub details: Option<Vec<models::PaymentMethodDetails>>,
    /// Payment method bank iban
    #[serde(rename = "bank_iban", skip_serializing_if = "Option::is_none")]
    pub bank_iban: Option<String>,
    /// Payment method bank name
    #[serde(rename = "bank_name", skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,
    /// Payment method bank beneficiary
    #[serde(rename = "bank_beneficiary", skip_serializing_if = "Option::is_none")]
    pub bank_beneficiary: Option<String>,
    /// E-invoice payment method
    #[serde(rename = "ei_payment_method", skip_serializing_if = "Option::is_none")]
    pub ei_payment_method: Option<String>,
}

impl PaymentMethod4 {
    /// [Only for client] Client default payment method
    pub fn new() -> PaymentMethod4 {
        PaymentMethod4 {
            id: None,
            name: None,
            r#type: None,
            is_default: None,
            default_payment_account: None,
            details: None,
            bank_iban: None,
            bank_name: None,
            bank_beneficiary: None,
            ei_payment_method: None,
        }
    }
}
/// Payment method type
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "standard")]
    Standard,
    #[serde(rename = "riba")]
    Riba,
}

impl Default for Type {
    fn default() -> Type {
        Self::Standard
    }
}

