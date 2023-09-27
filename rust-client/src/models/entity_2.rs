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
pub struct Entity2 {
    /// Entity id
    #[serde(rename = "id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub id: Option<Option<i32>>,
    /// Entity code
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// Entity name
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Entity type
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
    /// Entity first name
    #[serde(rename = "first_name", skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// Entity last name
    #[serde(rename = "last_name", skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Entity contact person
    #[serde(rename = "contact_person", skip_serializing_if = "Option::is_none")]
    pub contact_person: Option<String>,
    /// Entity vat number
    #[serde(rename = "vat_number", skip_serializing_if = "Option::is_none")]
    pub vat_number: Option<String>,
    /// Entity tax code
    #[serde(rename = "tax_code", skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<String>,
    /// Entitity address street
    #[serde(rename = "address_street", skip_serializing_if = "Option::is_none")]
    pub address_street: Option<String>,
    /// Entity address postal code
    #[serde(rename = "address_postal_code", skip_serializing_if = "Option::is_none")]
    pub address_postal_code: Option<String>,
    /// Entity address city
    #[serde(rename = "address_city", skip_serializing_if = "Option::is_none")]
    pub address_city: Option<String>,
    /// Entity address province
    #[serde(rename = "address_province", skip_serializing_if = "Option::is_none")]
    pub address_province: Option<String>,
    /// Entity address extra info
    #[serde(rename = "address_extra", skip_serializing_if = "Option::is_none")]
    pub address_extra: Option<String>,
    /// Entity country
    #[serde(rename = "country", skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Entity country iso code
    #[serde(rename = "country_iso", skip_serializing_if = "Option::is_none")]
    pub country_iso: Option<String>,
    /// Entity email
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Entity certified email
    #[serde(rename = "certified_email", skip_serializing_if = "Option::is_none")]
    pub certified_email: Option<String>,
    /// Entity phone
    #[serde(rename = "phone", skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// Entity fax
    #[serde(rename = "fax", skip_serializing_if = "Option::is_none")]
    pub fax: Option<String>,
    /// Entity extra
    #[serde(rename = "notes", skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    /// [Only for client] Client default payment terms
    #[serde(rename = "default_payment_terms", skip_serializing_if = "Option::is_none")]
    pub default_payment_terms: Option<i32>,
    #[serde(rename = "default_vat", skip_serializing_if = "Option::is_none")]
    pub default_vat: Option<Box<crate::models::VatType4>>,
    /// [Only for client] Client default payment terms type
    #[serde(rename = "default_payment_terms_type", skip_serializing_if = "Option::is_none")]
    pub default_payment_terms_type: Option<DefaultPaymentTermsType>,
    #[serde(rename = "default_payment_method", skip_serializing_if = "Option::is_none")]
    pub default_payment_method: Option<Box<crate::models::PaymentMethod5>>,
    /// [Only for client] Client bank name
    #[serde(rename = "bank_name", skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,
    /// [Only for client] Client bank iban
    #[serde(rename = "bank_iban", skip_serializing_if = "Option::is_none")]
    pub bank_iban: Option<String>,
    /// [Only for client] Client bank swift code
    #[serde(rename = "bank_swift_code", skip_serializing_if = "Option::is_none")]
    pub bank_swift_code: Option<String>,
    /// [Only for client] Client Shipping address
    #[serde(rename = "shipping_address", skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<String>,
    /// [Only for client] Use e-invoices.
    #[serde(rename = "e_invoice", skip_serializing_if = "Option::is_none")]
    pub e_invoice: Option<bool>,
    /// [Only for client] E-invoices code.
    #[serde(rename = "ei_code", skip_serializing_if = "Option::is_none")]
    pub ei_code: Option<String>,
    /// [Only for client] Has intent declaration.
    #[serde(rename = "has_intent_declaration", skip_serializing_if = "Option::is_none")]
    pub has_intent_declaration: Option<bool>,
    /// [Only for client] Client intent declaration protocol number
    #[serde(rename = "intent_declaration_protocol_number", skip_serializing_if = "Option::is_none")]
    pub intent_declaration_protocol_number: Option<String>,
    /// [Only for client] Client intent declaration protocol date
    #[serde(rename = "intent_declaration_protocol_date", skip_serializing_if = "Option::is_none")]
    pub intent_declaration_protocol_date: Option<String>,
    /// Entity creation date
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Entity last update date
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

impl Entity2 {
    pub fn new() -> Entity2 {
        Entity2 {
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
            default_payment_terms: None,
            default_vat: None,
            default_payment_terms_type: None,
            default_payment_method: None,
            bank_name: None,
            bank_iban: None,
            bank_swift_code: None,
            shipping_address: None,
            e_invoice: None,
            ei_code: None,
            has_intent_declaration: None,
            intent_declaration_protocol_number: None,
            intent_declaration_protocol_date: None,
            created_at: None,
            updated_at: None,
        }
    }
}

/// Entity type
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
/// [Only for client] Client default payment terms type
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
