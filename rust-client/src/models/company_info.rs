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
pub struct CompanyInfo {
    /// Company id
    #[serde(rename = "id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub id: Option<Option<i32>>,
    /// Company name
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
    /// Company email
    #[serde(rename = "email", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub email: Option<Option<String>>,
    /// Company type
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
    #[serde(rename = "access_info", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub access_info: Option<Option<Box<models::CompanyInfoAccessInfo>>>,
    #[serde(rename = "fic_license_expire", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub fic_license_expire: Option<Option<String>>,
    /// Fatture in Cloud account plan type.
    #[serde(rename = "fic_plan_name", skip_serializing_if = "Option::is_none")]
    pub fic_plan_name: Option<FicPlanName>,
    #[serde(rename = "plan_info", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub plan_info: Option<Option<Box<models::CompanyInfoPlanInfo>>>,
    /// Company accountant id
    #[serde(rename = "accountant_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub accountant_id: Option<Option<i32>>,
    /// Is the logged account an accountant.
    #[serde(rename = "is_accountant", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub is_accountant: Option<Option<bool>>,
}

impl CompanyInfo {
    pub fn new() -> CompanyInfo {
        CompanyInfo {
            id: None,
            name: None,
            email: None,
            r#type: None,
            access_info: None,
            fic_license_expire: None,
            fic_plan_name: None,
            plan_info: None,
            accountant_id: None,
            is_accountant: None,
        }
    }
}
/// Company type
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "company")]
    Company,
    #[serde(rename = "accountant")]
    Accountant,
}

impl Default for Type {
    fn default() -> Type {
        Self::Company
    }
}
/// Fatture in Cloud account plan type.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FicPlanName {
    #[serde(rename = "trial")]
    Trial,
    #[serde(rename = "standard")]
    Standard,
    #[serde(rename = "premium")]
    Premium,
    #[serde(rename = "premium_plus")]
    PremiumPlus,
    #[serde(rename = "complete")]
    Complete,
}

impl Default for FicPlanName {
    fn default() -> FicPlanName {
        Self::Trial
    }
}

