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
pub struct GetUserInfoResponseInfo {
    #[serde(rename = "need_marketing_consents_confirmation", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub need_marketing_consents_confirmation: Option<Option<bool>>,
    #[serde(rename = "need_password_change", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub need_password_change: Option<Option<bool>>,
    #[serde(rename = "need_terms_of_service_confirmation", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub need_terms_of_service_confirmation: Option<Option<bool>>,
}

impl GetUserInfoResponseInfo {
    pub fn new() -> GetUserInfoResponseInfo {
        GetUserInfoResponseInfo {
            need_marketing_consents_confirmation: None,
            need_password_change: None,
            need_terms_of_service_confirmation: None,
        }
    }
}

