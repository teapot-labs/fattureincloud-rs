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
pub struct PaymentMethod {
    /// Payment method id
    #[serde(rename = "id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub id: Option<Option<i32>>,
    /// Payment method name
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
    /// Payment method type
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
    /// Payment method is default
    #[serde(rename = "is_default", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub is_default: Option<Option<bool>>,
    #[serde(rename = "default_payment_account", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub default_payment_account: Option<Option<Box<models::PaymentAccount>>>,
    /// Payment method details
    #[serde(rename = "details", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub details: Option<Option<Vec<models::PaymentMethodDetails>>>,
    /// Payment method bank iban
    #[serde(rename = "bank_iban", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub bank_iban: Option<Option<String>>,
    /// Payment method bank name
    #[serde(rename = "bank_name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<Option<String>>,
    /// Payment method bank beneficiary
    #[serde(rename = "bank_beneficiary", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub bank_beneficiary: Option<Option<String>>,
    /// E-invoice payment method
    #[serde(rename = "ei_payment_method", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub ei_payment_method: Option<Option<String>>,
}

impl PaymentMethod {
    pub fn new() -> PaymentMethod {
        PaymentMethod {
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

