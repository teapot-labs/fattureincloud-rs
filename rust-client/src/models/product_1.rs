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
pub struct Product1 {
    /// Product id
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// Product name
    #[serde(rename = "name")]
    pub name: String,
    /// Product code
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// Product net price
    #[serde(rename = "net_price", skip_serializing_if = "Option::is_none")]
    pub net_price: Option<f64>,
    /// Product gross price
    #[serde(rename = "gross_price", skip_serializing_if = "Option::is_none")]
    pub gross_price: Option<f64>,
    /// Product uses gross prices
    #[serde(rename = "use_gross_price", skip_serializing_if = "Option::is_none")]
    pub use_gross_price: Option<bool>,
    #[serde(rename = "default_vat", skip_serializing_if = "Option::is_none")]
    pub default_vat: Option<Box<models::VatType1>>,
    /// Product net cost
    #[serde(rename = "net_cost", skip_serializing_if = "Option::is_none")]
    pub net_cost: Option<f64>,
    /// Product measure
    #[serde(rename = "measure", skip_serializing_if = "Option::is_none")]
    pub measure: Option<String>,
    /// Product description
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Product category
    #[serde(rename = "category", skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// Product extra notes
    #[serde(rename = "notes", skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    /// Product has stock
    #[serde(rename = "in_stock", skip_serializing_if = "Option::is_none")]
    pub in_stock: Option<bool>,
    /// Product initial stock
    #[serde(rename = "stock_initial", skip_serializing_if = "Option::is_none")]
    pub stock_initial: Option<f64>,
    /// [Read Only] Product current stock
    #[serde(rename = "stock_current", skip_serializing_if = "Option::is_none")]
    pub stock_current: Option<f64>,
    /// Product average cost
    #[serde(rename = "average_cost", skip_serializing_if = "Option::is_none")]
    pub average_cost: Option<f64>,
    /// Product average price
    #[serde(rename = "average_price", skip_serializing_if = "Option::is_none")]
    pub average_price: Option<f64>,
    /// Product creation date
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Product last update date
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

impl Product1 {
    pub fn new(name: String) -> Product1 {
        Product1 {
            id: None,
            name,
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

