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
pub struct User {
    /// User id
    #[serde(rename = "id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub id: Option<Option<i32>>,
    /// User full name
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
    /// User first name
    #[serde(rename = "first_name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub first_name: Option<Option<String>>,
    /// User last name
    #[serde(rename = "last_name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub last_name: Option<Option<String>>,
    /// User email address
    #[serde(rename = "email", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub email: Option<Option<String>>,
    /// User hash
    #[serde(rename = "hash", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub hash: Option<Option<String>>,
    /// User picture
    #[serde(rename = "picture", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub picture: Option<Option<String>>,
}

impl User {
    pub fn new() -> User {
        User {
            id: None,
            name: None,
            first_name: None,
            last_name: None,
            email: None,
            hash: None,
            picture: None,
        }
    }
}

