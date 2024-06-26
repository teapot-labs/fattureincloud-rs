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
pub struct PermissionsFicIssuedDocumentsDetailed {
    /// Permission level
    #[serde(rename = "quotes", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub quotes: Option<Option<Quotes>>,
    /// Permission level
    #[serde(rename = "proformas", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub proformas: Option<Option<Proformas>>,
    /// Permission level
    #[serde(rename = "invoices", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub invoices: Option<Option<Invoices>>,
    /// Permission level
    #[serde(rename = "receipts", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub receipts: Option<Option<Receipts>>,
    /// Permission level
    #[serde(rename = "delivery_notes", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub delivery_notes: Option<Option<DeliveryNotes>>,
    /// Permission level
    #[serde(rename = "credit_notes", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub credit_notes: Option<Option<CreditNotes>>,
    /// Permission level
    #[serde(rename = "orders", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub orders: Option<Option<Orders>>,
    /// Permission level
    #[serde(rename = "work_reports", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub work_reports: Option<Option<WorkReports>>,
    /// Permission level
    #[serde(rename = "supplier_orders", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub supplier_orders: Option<Option<SupplierOrders>>,
    /// Permission level
    #[serde(rename = "self_invoices", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub self_invoices: Option<Option<SelfInvoices>>,
}

impl PermissionsFicIssuedDocumentsDetailed {
    pub fn new() -> PermissionsFicIssuedDocumentsDetailed {
        PermissionsFicIssuedDocumentsDetailed {
            quotes: None,
            proformas: None,
            invoices: None,
            receipts: None,
            delivery_notes: None,
            credit_notes: None,
            orders: None,
            work_reports: None,
            supplier_orders: None,
            self_invoices: None,
        }
    }
}
/// Permission level
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Quotes {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
    #[serde(rename = "detailed")]
    Detailed,
}

impl Default for Quotes {
    fn default() -> Quotes {
        Self::None
    }
}
/// Permission level
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Proformas {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
    #[serde(rename = "detailed")]
    Detailed,
}

impl Default for Proformas {
    fn default() -> Proformas {
        Self::None
    }
}
/// Permission level
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Invoices {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
    #[serde(rename = "detailed")]
    Detailed,
}

impl Default for Invoices {
    fn default() -> Invoices {
        Self::None
    }
}
/// Permission level
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Receipts {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
    #[serde(rename = "detailed")]
    Detailed,
}

impl Default for Receipts {
    fn default() -> Receipts {
        Self::None
    }
}
/// Permission level
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DeliveryNotes {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
    #[serde(rename = "detailed")]
    Detailed,
}

impl Default for DeliveryNotes {
    fn default() -> DeliveryNotes {
        Self::None
    }
}
/// Permission level
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CreditNotes {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
    #[serde(rename = "detailed")]
    Detailed,
}

impl Default for CreditNotes {
    fn default() -> CreditNotes {
        Self::None
    }
}
/// Permission level
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Orders {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
    #[serde(rename = "detailed")]
    Detailed,
}

impl Default for Orders {
    fn default() -> Orders {
        Self::None
    }
}
/// Permission level
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum WorkReports {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
    #[serde(rename = "detailed")]
    Detailed,
}

impl Default for WorkReports {
    fn default() -> WorkReports {
        Self::None
    }
}
/// Permission level
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SupplierOrders {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
    #[serde(rename = "detailed")]
    Detailed,
}

impl Default for SupplierOrders {
    fn default() -> SupplierOrders {
        Self::None
    }
}
/// Permission level
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SelfInvoices {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
    #[serde(rename = "detailed")]
    Detailed,
}

impl Default for SelfInvoices {
    fn default() -> SelfInvoices {
        Self::None
    }
}

