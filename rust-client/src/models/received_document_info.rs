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
pub struct ReceivedDocumentInfo {
    #[serde(rename = "default_values", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub default_values: Option<Option<Box<models::ReceivedDocumentInfoDefaultValues>>>,
    #[serde(rename = "items_default_values", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub items_default_values: Option<Option<Box<models::ReceivedDocumentInfoItemsDefaultValues>>>,
    /// Countries list
    #[serde(rename = "countries_list", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub countries_list: Option<Option<Vec<String>>>,
    /// Currencies list
    #[serde(rename = "currencies_list", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub currencies_list: Option<Option<Vec<models::Currency>>>,
    /// Categories list
    #[serde(rename = "categories_list", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub categories_list: Option<Option<Vec<String>>>,
    /// Payments accounts list
    #[serde(rename = "payment_accounts_list", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub payment_accounts_list: Option<Option<Vec<models::PaymentAccount>>>,
    /// Vat types list
    #[serde(rename = "vat_types_list", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub vat_types_list: Option<Option<Vec<models::VatType>>>,
}

impl ReceivedDocumentInfo {
    pub fn new() -> ReceivedDocumentInfo {
        ReceivedDocumentInfo {
            default_values: None,
            items_default_values: None,
            countries_list: None,
            currencies_list: None,
            categories_list: None,
            payment_accounts_list: None,
            vat_types_list: None,
        }
    }
}

