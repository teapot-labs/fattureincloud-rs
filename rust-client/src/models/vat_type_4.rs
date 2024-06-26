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

/// VatType4 : [Only for client] Client default vat.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct VatType4 {
    /// Vat type id
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// [Read Only] Vat type percentual value
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
    /// Vat type short description
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Vat type notes shown in documents
    #[serde(rename = "notes", skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    /// Vat type is usable for e-invoices
    #[serde(rename = "e_invoice", skip_serializing_if = "Option::is_none")]
    pub e_invoice: Option<bool>,
    /// Vat type e-invoice type (natura)
    #[serde(rename = "ei_type", skip_serializing_if = "Option::is_none")]
    pub ei_type: Option<String>,
    /// Vat type e-invoice description
    #[serde(rename = "ei_description", skip_serializing_if = "Option::is_none")]
    pub ei_description: Option<String>,
    /// [Read Only] Is the vat type is editable.
    #[serde(rename = "editable", skip_serializing_if = "Option::is_none")]
    pub editable: Option<bool>,
    /// Is the vat type disabled
    #[serde(rename = "is_disabled", skip_serializing_if = "Option::is_none")]
    pub is_disabled: Option<bool>,
}

impl VatType4 {
    /// [Only for client] Client default vat.
    pub fn new() -> VatType4 {
        VatType4 {
            id: None,
            value: None,
            description: None,
            notes: None,
            e_invoice: None,
            ei_type: None,
            ei_description: None,
            editable: None,
            is_disabled: None,
        }
    }
}

