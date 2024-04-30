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
pub struct Product2 {
    /// Product id
    #[serde(rename = "id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub id: Option<Option<i32>>,
    /// Product name
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
    /// Product code
    #[serde(rename = "code", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub code: Option<Option<String>>,
    /// Product net price
    #[serde(rename = "net_price", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub net_price: Option<Option<f64>>,
    /// Product gross price
    #[serde(rename = "gross_price", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub gross_price: Option<Option<f64>>,
    /// Product uses gross prices
    #[serde(rename = "use_gross_price", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub use_gross_price: Option<Option<bool>>,
    #[serde(rename = "default_vat", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub default_vat: Option<Option<Box<models::VatType>>>,
    /// Product net cost
    #[serde(rename = "net_cost", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub net_cost: Option<Option<f64>>,
    /// Product measure
    #[serde(rename = "measure", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub measure: Option<Option<String>>,
    /// Product description
    #[serde(rename = "description", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
    /// Product category
    #[serde(rename = "category", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub category: Option<Option<String>>,
    /// Product extra notes
    #[serde(rename = "notes", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub notes: Option<Option<String>>,
    /// Product has stock
    #[serde(rename = "in_stock", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub in_stock: Option<Option<bool>>,
    /// Product initial stock
    #[serde(rename = "stock_initial", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub stock_initial: Option<Option<f64>>,
    /// [Read Only] Product current stock
    #[serde(rename = "stock_current", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub stock_current: Option<Option<f64>>,
    /// Product average cost
    #[serde(rename = "average_cost", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub average_cost: Option<Option<f64>>,
    /// Product average price
    #[serde(rename = "average_price", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub average_price: Option<Option<f64>>,
    /// Product creation date
    #[serde(rename = "created_at", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Option<String>>,
    /// Product last update date
    #[serde(rename = "updated_at", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<Option<String>>,
}

impl Product2 {
    pub fn new() -> Product2 {
        Product2 {
            id: None,
            name: None,
            code: None,
            net_price: None,
            gross_price: None,
            use_gross_price: None,
            default_vat: None,
            net_cost: None,
            measure: None,
            description: None,
            category: None,
            notes: None,
            in_stock: None,
            stock_initial: None,
            stock_current: None,
            average_cost: None,
            average_price: None,
            created_at: None,
            updated_at: None,
        }
    }
}

