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
pub struct Company {
    /// Company id
    #[serde(rename = "id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub id: Option<Option<i32>>,
    /// Company name
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
    /// Company type
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
    /// Company authentication token for this company. [Only if type=company]
    #[serde(rename = "access_token", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub access_token: Option<Option<String>>,
    /// Company list of controlled companies [Only if type=accountant]
    #[serde(rename = "controlled_companies", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub controlled_companies: Option<Option<Vec<crate::models::ControlledCompany>>>,
    /// Company connection id
    #[serde(rename = "connection_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub connection_id: Option<Option<i32>>,
    /// Company tax code
    #[serde(rename = "tax_code", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<Option<String>>,
}

impl Company {
    pub fn new() -> Company {
        Company {
            id: None,
            name: None,
            r#type: None,
            access_token: None,
            controlled_companies: None,
            connection_id: None,
            tax_code: None,
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

