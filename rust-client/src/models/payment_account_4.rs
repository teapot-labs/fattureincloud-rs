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

/// PaymentAccount4 : [Only for cashbook entry out] Cashbook payment account out
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PaymentAccount4 {
    /// Payment account id
    #[serde(rename = "id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub id: Option<Option<i32>>,
    /// Payment account name
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
    /// Payment account type
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
    /// Payment account iban
    #[serde(rename = "iban", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub iban: Option<Option<String>>,
    /// Payment account sia
    #[serde(rename = "sia", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub sia: Option<Option<String>>,
    /// Payment account cuc
    #[serde(rename = "cuc", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub cuc: Option<Option<String>>,
    /// Payment method is virtual
    #[serde(rename = "virtual", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub r#virtual: Option<Option<bool>>,
}

impl PaymentAccount4 {
    /// [Only for cashbook entry out] Cashbook payment account out
    pub fn new() -> PaymentAccount4 {
        PaymentAccount4 {
            id: None,
            name: None,
            r#type: None,
            iban: None,
            sia: None,
            cuc: None,
            r#virtual: None,
        }
    }
}
/// Payment account type
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "standard")]
    Standard,
    #[serde(rename = "bank")]
    Bank,
}

impl Default for Type {
    fn default() -> Type {
        Self::Standard
    }
}

