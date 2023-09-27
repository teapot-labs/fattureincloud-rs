/*
 * Fatture in Cloud API v2 - API Reference
 *
 * Connect your software with Fatture in Cloud, the invoicing platform chosen by more than 500.000 businesses in Italy.   The Fatture in Cloud API is based on REST, and makes possible to interact with the user related data prior authorization via OAuth2 protocol.
 *
 * The version of the OpenAPI document: 2.0.29
 * Contact: info@fattureincloud.it
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListProductsResponse {
    /// Current page number.
    #[serde(rename = "current_page", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub current_page: Option<Option<i32>>,
    /// First page url.
    #[serde(rename = "first_page_url", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub first_page_url: Option<Option<String>>,
    /// First result of the page.
    #[serde(rename = "from", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub from: Option<Option<i32>>,
    /// Last page number.
    #[serde(rename = "last_page", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub last_page: Option<Option<i32>>,
    /// Last page url.
    #[serde(rename = "last_page_url", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub last_page_url: Option<Option<String>>,
    /// Next page url
    #[serde(rename = "next_page_url", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub next_page_url: Option<Option<String>>,
    /// Request path.
    #[serde(rename = "path", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub path: Option<Option<String>>,
    /// Number of result per page.
    #[serde(rename = "per_page", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub per_page: Option<Option<i32>>,
    /// Previous page url.
    #[serde(rename = "prev_page_url", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub prev_page_url: Option<Option<String>>,
    /// Last result of the page.
    #[serde(rename = "to", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub to: Option<Option<i32>>,
    /// Total number of results
    #[serde(rename = "total", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub total: Option<Option<i32>>,
    #[serde(rename = "data", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub data: Option<Option<Vec<crate::models::Product>>>,
}

impl ListProductsResponse {
    pub fn new() -> ListProductsResponse {
        ListProductsResponse {
            current_page: None,
            first_page_url: None,
            from: None,
            last_page: None,
            last_page_url: None,
            next_page_url: None,
            path: None,
            per_page: None,
            prev_page_url: None,
            to: None,
            total: None,
            data: None,
        }
    }
}


