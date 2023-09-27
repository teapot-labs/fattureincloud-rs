/*
 * Fatture in Cloud API v2 - API Reference
 *
 * Connect your software with Fatture in Cloud, the invoicing platform chosen by more than 500.000 businesses in Italy.   The Fatture in Cloud API is based on REST, and makes possible to interact with the user related data prior authorization via OAuth2 protocol.
 *
 * The version of the OpenAPI document: 2.0.29
 * Contact: info@fattureincloud.it
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Receipt1 {
    /// Receipt id
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// Receipt date
    #[serde(rename = "date")]
    pub date: String,
    /// Receipt number
    #[serde(rename = "number", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub number: Option<Option<f32>>,
    /// Receipt numeration
    #[serde(rename = "numeration", skip_serializing_if = "Option::is_none")]
    pub numeration: Option<String>,
    /// Receipt total net amount
    #[serde(rename = "amount_net", skip_serializing_if = "Option::is_none")]
    pub amount_net: Option<f32>,
    /// Receipt total vat amount
    #[serde(rename = "amount_vat", skip_serializing_if = "Option::is_none")]
    pub amount_vat: Option<f32>,
    /// Receipt total gross amount
    #[serde(rename = "amount_gross", skip_serializing_if = "Option::is_none")]
    pub amount_gross: Option<f32>,
    /// Receipt uses gross prices
    #[serde(rename = "use_gross_prices", skip_serializing_if = "Option::is_none")]
    pub use_gross_prices: Option<bool>,
    /// Receipt type
    #[serde(rename = "type")]
    pub r#type: Type,
    /// Receipt description
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Receipt revenue center
    #[serde(rename = "rc_center", skip_serializing_if = "Option::is_none")]
    pub rc_center: Option<String>,
    /// Receipt creation date
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Receipt last update date
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "payment_account", deserialize_with = "Option::deserialize")]
    pub payment_account: Option<Box<crate::models::PaymentAccount1>>,
    #[serde(rename = "items_list", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub items_list: Option<Option<Vec<crate::models::ReceiptItemsListItem>>>,
}

impl Receipt1 {
    pub fn new(date: String, r#type: Type, payment_account: Option<crate::models::PaymentAccount1>) -> Receipt1 {
        Receipt1 {
            id: None,
            date,
            number: None,
            numeration: None,
            amount_net: None,
            amount_vat: None,
            amount_gross: None,
            use_gross_prices: None,
            r#type,
            description: None,
            rc_center: None,
            created_at: None,
            updated_at: None,
            payment_account: if let Some(x) = payment_account {Some(Box::new(x))} else {None},
            items_list: None,
        }
    }
}

/// Receipt type
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "till_receipt")]
    TillReceipt,
    #[serde(rename = "sales_receipt")]
    SalesReceipt,
}

impl Default for Type {
    fn default() -> Type {
        Self::TillReceipt
    }
}

