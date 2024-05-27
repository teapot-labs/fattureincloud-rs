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
pub struct Client2 {
    /// Client id
    #[serde(rename = "id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub id: Option<Option<i32>>,
    /// Client code
    #[serde(rename = "code", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub code: Option<Option<String>>,
    /// Client name
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
    /// Client type
    #[serde(rename = "type", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Option<Type>>,
    /// Client first name
    #[serde(rename = "first_name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub first_name: Option<Option<String>>,
    /// Client last name
    #[serde(rename = "last_name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub last_name: Option<Option<String>>,
    /// Client contact person
    #[serde(rename = "contact_person", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub contact_person: Option<Option<String>>,
    /// Client vat number
    #[serde(rename = "vat_number", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub vat_number: Option<Option<String>>,
    /// Client tax code
    #[serde(rename = "tax_code", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<Option<String>>,
    /// Client address street
    #[serde(rename = "address_street", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub address_street: Option<Option<String>>,
    /// Client address postal code
    #[serde(rename = "address_postal_code", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub address_postal_code: Option<Option<String>>,
    /// Client address city
    #[serde(rename = "address_city", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub address_city: Option<Option<String>>,
    /// Client address province
    #[serde(rename = "address_province", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub address_province: Option<Option<String>>,
    /// Client address extra info
    #[serde(rename = "address_extra", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub address_extra: Option<Option<String>>,
    /// Client country
    #[serde(rename = "country", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub country: Option<Option<String>>,
    /// Client country iso code
    #[serde(rename = "country_iso", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub country_iso: Option<Option<String>>,
    /// Client email
    #[serde(rename = "email", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub email: Option<Option<String>>,
    /// Client certified email
    #[serde(rename = "certified_email", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub certified_email: Option<Option<String>>,
    /// Client phone
    #[serde(rename = "phone", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub phone: Option<Option<String>>,
    /// Client fax
    #[serde(rename = "fax", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub fax: Option<Option<String>>,
    /// Client extra
    #[serde(rename = "notes", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub notes: Option<Option<String>>,
    #[serde(rename = "default_vat", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub default_vat: Option<Option<Box<models::VatType>>>,
    /// Client default payment terms
    #[serde(rename = "default_payment_terms", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub default_payment_terms: Option<Option<i32>>,
    /// Payment terms type
    #[serde(rename = "default_payment_terms_type", skip_serializing_if = "Option::is_none")]
    pub default_payment_terms_type: Option<DefaultPaymentTermsType>,
    #[serde(rename = "default_payment_method", skip_serializing_if = "Option::is_none")]
    pub default_payment_method: Option<Box<models::PaymentMethod>>,
    /// Client bank name
    #[serde(rename = "bank_name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<Option<String>>,
    /// Client bank iban
    #[serde(rename = "bank_iban", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub bank_iban: Option<Option<String>>,
    /// Client bank swift code
    #[serde(rename = "bank_swift_code", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub bank_swift_code: Option<Option<String>>,
    /// Client shipping address
    #[serde(rename = "shipping_address", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<Option<String>>,
    /// Use e-invoices for this entity
    #[serde(rename = "e_invoice", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub e_invoice: Option<Option<bool>>,
    /// Client e-invoice code 
    #[serde(rename = "ei_code", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub ei_code: Option<Option<String>>,
    /// Highlight Discount
    #[serde(rename = "discount_highlight", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub discount_highlight: Option<Option<bool>>,
    /// Client default discount
    #[serde(rename = "default_discount", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub default_discount: Option<Option<f64>>,
    /// Client has intent declaration
    #[serde(rename = "has_intent_declaration", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub has_intent_declaration: Option<Option<bool>>,
    /// Client intent declaration protocol number
    #[serde(rename = "intent_declaration_protocol_number", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub intent_declaration_protocol_number: Option<Option<String>>,
    /// Client intent declaration protocol date
    #[serde(rename = "intent_declaration_protocol_date", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub intent_declaration_protocol_date: Option<Option<String>>,
    /// Client creation date
    #[serde(rename = "created_at", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Option<String>>,
    /// Client last update date
    #[serde(rename = "updated_at", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<Option<String>>,
}

impl Client2 {
    pub fn new() -> Client2 {
        Client2 {
            id: None,
            code: None,
            name: None,
            r#type: None,
            first_name: None,
            last_name: None,
            contact_person: None,
            vat_number: None,
            tax_code: None,
            address_street: None,
            address_postal_code: None,
            address_city: None,
            address_province: None,
            address_extra: None,
            country: None,
            country_iso: None,
            email: None,
            certified_email: None,
            phone: None,
            fax: None,
            notes: None,
            default_vat: None,
            default_payment_terms: None,
            default_payment_terms_type: None,
            default_payment_method: None,
            bank_name: None,
            bank_iban: None,
            bank_swift_code: None,
            shipping_address: None,
            e_invoice: None,
            ei_code: None,
            discount_highlight: None,
            default_discount: None,
            has_intent_declaration: None,
            intent_declaration_protocol_number: None,
            intent_declaration_protocol_date: None,
            created_at: None,
            updated_at: None,
        }
    }
}
/// Client type
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "company")]
    Company,
    #[serde(rename = "person")]
    Person,
    #[serde(rename = "pa")]
    Pa,
    #[serde(rename = "condo")]
    Condo,
}

impl Default for Type {
    fn default() -> Type {
        Self::Company
    }
}
/// Payment terms type
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DefaultPaymentTermsType {
    #[serde(rename = "standard")]
    Standard,
    #[serde(rename = "end_of_month")]
    EndOfMonth,
}

impl Default for DefaultPaymentTermsType {
    fn default() -> DefaultPaymentTermsType {
        Self::Standard
    }
}

