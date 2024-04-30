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
pub struct ReceiptPreCreateInfo {
    #[serde(rename = "numerations", skip_serializing_if = "Option::is_none")]
    pub numerations: Option<std::collections::HashMap<String, std::collections::HashMap<String, i32>>>,
    /// Receipt used numerations list
    #[serde(rename = "numerations_list", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub numerations_list: Option<Option<Vec<String>>>,
    /// Receipt used revenue centers list
    #[serde(rename = "rc_centers_list", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub rc_centers_list: Option<Option<Vec<String>>>,
    /// Payment accounts list
    #[serde(rename = "payment_accounts_list", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub payment_accounts_list: Option<Option<Vec<models::PaymentAccount>>>,
    /// Receipt categories list
    #[serde(rename = "categories_list", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub categories_list: Option<Option<Vec<String>>>,
    /// Vat types list
    #[serde(rename = "vat_types_list", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub vat_types_list: Option<Option<Vec<models::VatType>>>,
}

impl ReceiptPreCreateInfo {
    pub fn new() -> ReceiptPreCreateInfo {
        ReceiptPreCreateInfo {
            numerations: None,
            numerations_list: None,
            rc_centers_list: None,
            payment_accounts_list: None,
            categories_list: None,
            vat_types_list: None,
        }
    }
}

