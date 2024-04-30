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
pub struct IssuedDocumentTotals {
    /// Issued document total net amount
    #[serde(rename = "amount_net", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub amount_net: Option<Option<f64>>,
    /// Issued document rivalsa amount
    #[serde(rename = "amount_rivalsa", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub amount_rivalsa: Option<Option<f64>>,
    /// Issued document net amount with rivalsa
    #[serde(rename = "amount_net_with_rivalsa", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub amount_net_with_rivalsa: Option<Option<f64>>,
    /// Issued document cassa amount
    #[serde(rename = "amount_cassa", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub amount_cassa: Option<Option<f64>>,
    /// Issued document taxable amount
    #[serde(rename = "taxable_amount", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub taxable_amount: Option<Option<f64>>,
    /// Issued document not taxable amount
    #[serde(rename = "not_taxable_amount", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub not_taxable_amount: Option<Option<f64>>,
    /// Issued document total vat amount
    #[serde(rename = "amount_vat", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub amount_vat: Option<Option<f64>>,
    /// Issued document total gross amount
    #[serde(rename = "amount_gross", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub amount_gross: Option<Option<f64>>,
    /// Issued document Taxable withholding tax amount
    #[serde(rename = "taxable_amount_withholding_tax", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub taxable_amount_withholding_tax: Option<Option<f64>>,
    /// Issued document withholding tax amount
    #[serde(rename = "amount_withholding_tax", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub amount_withholding_tax: Option<Option<f64>>,
    /// Issued document other withholding tax taxable amount
    #[serde(rename = "taxable_amount_other_withholding_tax", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub taxable_amount_other_withholding_tax: Option<Option<f64>>,
    /// Issued document other withholding tax amount
    #[serde(rename = "amount_other_withholding_tax", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub amount_other_withholding_tax: Option<Option<f64>>,
    /// Issued document stamp duty value [0 if not present].
    #[serde(rename = "stamp_duty", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub stamp_duty: Option<Option<f64>>,
    /// Issued document total amount due
    #[serde(rename = "amount_due", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub amount_due: Option<Option<f64>>,
    /// Is enasarco maximal excedeed
    #[serde(rename = "is_enasarco_maximal_exceeded", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub is_enasarco_maximal_exceeded: Option<Option<bool>>,
    /// Issued document payments sum
    #[serde(rename = "payments_sum", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub payments_sum: Option<Option<f64>>,
    /// Issued document vat list
    #[serde(rename = "vat_list", skip_serializing_if = "Option::is_none")]
    pub vat_list: Option<std::collections::HashMap<String, models::VatList>>,
}

impl IssuedDocumentTotals {
    pub fn new() -> IssuedDocumentTotals {
        IssuedDocumentTotals {
            amount_net: None,
            amount_rivalsa: None,
            amount_net_with_rivalsa: None,
            amount_cassa: None,
            taxable_amount: None,
            not_taxable_amount: None,
            amount_vat: None,
            amount_gross: None,
            taxable_amount_withholding_tax: None,
            amount_withholding_tax: None,
            taxable_amount_other_withholding_tax: None,
            amount_other_withholding_tax: None,
            stamp_duty: None,
            amount_due: None,
            is_enasarco_maximal_exceeded: None,
            payments_sum: None,
            vat_list: None,
        }
    }
}

