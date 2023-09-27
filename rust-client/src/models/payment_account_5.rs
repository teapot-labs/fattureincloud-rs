/*
 * Fatture in Cloud API v2 - API Reference
 *
 * Connect your software with Fatture in Cloud, the invoicing platform chosen by more than 500.000 businesses in Italy.   The Fatture in Cloud API is based on REST, and makes possible to interact with the user related data prior authorization via OAuth2 protocol.
 *
 * The version of the OpenAPI document: 2.0.29
 * Contact: info@fattureincloud.it
 * Generated by: https://openapi-generator.tech
 */

/// PaymentAccount5 : [Only for cashbook entry in] Cashbook payment account in



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PaymentAccount5 {
    /// Payment account id
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// Payment account name
    #[serde(rename = "name")]
    pub name: String,
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
    #[serde(rename = "cuc", skip_serializing_if = "Option::is_none")]
    pub cuc: Option<String>,
    /// Payment method is virtual
    #[serde(rename = "virtual", skip_serializing_if = "Option::is_none")]
    pub r#virtual: Option<bool>,
}

impl PaymentAccount5 {
    /// [Only for cashbook entry in] Cashbook payment account in
    pub fn new(name: String) -> PaymentAccount5 {
        PaymentAccount5 {
            id: None,
            name,
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
