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
pub struct ReceivedDocumentItemsListItem {
    /// Received document item id
    #[serde(rename = "id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub id: Option<Option<i32>>,
    /// Received document product id
    #[serde(rename = "product_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub product_id: Option<Option<i32>>,
    /// Received document item product code
    #[serde(rename = "code", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub code: Option<Option<String>>,
    /// Received document item product name
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
    /// Received document item measure
    #[serde(rename = "measure", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub measure: Option<Option<String>>,
    /// Received document item product net price
    #[serde(rename = "net_price", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub net_price: Option<Option<f32>>,
    /// Received document item product category
    #[serde(rename = "category", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub category: Option<Option<String>>,
    /// Received document item quantity
    #[serde(rename = "qty", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub qty: Option<Option<f32>>,
    #[serde(rename = "vat", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub vat: Option<Option<Box<crate::models::VatType>>>,
    /// Received document item product number of items in stock
    #[serde(rename = "stock", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub stock: Option<Option<f32>>,
}

impl ReceivedDocumentItemsListItem {
    pub fn new() -> ReceivedDocumentItemsListItem {
        ReceivedDocumentItemsListItem {
            id: None,
            product_id: None,
            code: None,
            name: None,
            measure: None,
            net_price: None,
            category: None,
            qty: None,
            vat: None,
            stock: None,
        }
    }
}

