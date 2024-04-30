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

/// CompanyInfoPlanInfo : Company plan info
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CompanyInfoPlanInfo {
    #[serde(rename = "limits", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub limits: Option<Option<Box<models::CompanyInfoPlanInfoLimits>>>,
    #[serde(rename = "functions", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub functions: Option<Option<Box<models::CompanyInfoPlanInfoFunctions>>>,
    #[serde(rename = "functions_status", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub functions_status: Option<Option<Box<models::CompanyInfoPlanInfoFunctionsStatus>>>,
}

impl CompanyInfoPlanInfo {
    /// Company plan info
    pub fn new() -> CompanyInfoPlanInfo {
        CompanyInfoPlanInfo {
            limits: None,
            functions: None,
            functions_status: None,
        }
    }
}

