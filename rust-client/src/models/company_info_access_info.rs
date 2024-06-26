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
pub struct CompanyInfoAccessInfo {
    /// User company role
    #[serde(rename = "role", skip_serializing_if = "Option::is_none")]
    pub role: Option<Role>,
    #[serde(rename = "permissions", skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Box<models::Permissions>>,
    /// Company activated through accountant
    #[serde(rename = "through_accountant", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub through_accountant: Option<Option<bool>>,
}

impl CompanyInfoAccessInfo {
    pub fn new() -> CompanyInfoAccessInfo {
        CompanyInfoAccessInfo {
            role: None,
            permissions: None,
            through_accountant: None,
        }
    }
}
/// User company role
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Role {
    #[serde(rename = "master")]
    Master,
    #[serde(rename = "subaccount")]
    Subaccount,
    #[serde(rename = "employee")]
    Employee,
}

impl Default for Role {
    fn default() -> Role {
        Self::Master
    }
}

