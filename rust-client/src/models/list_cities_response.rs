/*
 * Fatture in Cloud API v2 - API Reference
 *
 * Connect your software with Fatture in Cloud, the invoicing platform chosen by more than 500.000 businesses in Italy.   The Fatture in Cloud API is based on REST, and makes possible to interact with the user related data prior authorization via OAuth2 protocol.
 *
 * The version of the OpenAPI document: 2.0.29
 * Contact: info@fattureincloud.it
 * Generated by: https://openapi-generator.tech
 */

/// ListCitiesResponse : 



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListCitiesResponse {
    #[serde(rename = "data", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub data: Option<Option<Vec<crate::models::City>>>,
}

impl ListCitiesResponse {
    /// 
    pub fn new() -> ListCitiesResponse {
        ListCitiesResponse {
            data: None,
        }
    }
}


