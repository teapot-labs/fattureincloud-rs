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

/// CompanyInfoPlanInfoLimits : Company plan limits
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CompanyInfoPlanInfoLimits {
    /// Company plan clients limits
    #[serde(rename = "clients", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub clients: Option<Option<i32>>,
    /// Company plan suppliers limits
    #[serde(rename = "suppliers", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub suppliers: Option<Option<i32>>,
    /// Company plan products limits
    #[serde(rename = "products", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub products: Option<Option<i32>>,
    /// Company plan documents limits
    #[serde(rename = "documents", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub documents: Option<Option<i32>>,
}

impl CompanyInfoPlanInfoLimits {
    /// Company plan limits
    pub fn new() -> CompanyInfoPlanInfoLimits {
        CompanyInfoPlanInfoLimits {
            clients: None,
            suppliers: None,
            products: None,
            documents: None,
        }
    }
}

