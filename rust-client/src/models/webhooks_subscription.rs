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
pub struct WebhooksSubscription {
    /// Webhooks subscription id
    #[serde(rename = "id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub id: Option<Option<String>>,
    /// Webhooks callback uri.
    #[serde(rename = "sink", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub sink: Option<Option<String>>,
    /// [Read Only] True if the webhooks subscription has been verified.
    #[serde(rename = "verified", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub verified: Option<Option<bool>>,
    /// Webhooks events types.
    #[serde(rename = "types", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub types: Option<Option<Vec<Types>>>,
    #[serde(rename = "config", skip_serializing_if = "Option::is_none")]
    pub config: Option<Box<models::WebhooksSubscriptionConfig>>,
}

impl WebhooksSubscription {
    pub fn new() -> WebhooksSubscription {
        WebhooksSubscription {
            id: None,
            sink: None,
            verified: None,
            types: None,
            config: None,
        }
    }
}
/// Webhooks events types.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Types {
    #[serde(rename = "it.fattureincloud.webhooks.issued_documents.invoices.create")]
    IssuedDocumentsPeriodInvoicesPeriodCreate,
    #[serde(rename = "it.fattureincloud.webhooks.issued_documents.invoices.update")]
    IssuedDocumentsPeriodInvoicesPeriodUpdate,
    #[serde(rename = "it.fattureincloud.webhooks.issued_documents.invoices.delete")]
    IssuedDocumentsPeriodInvoicesPeriodDelete,
    #[serde(rename = "it.fattureincloud.webhooks.issued_documents.quotes.create")]
    IssuedDocumentsPeriodQuotesPeriodCreate,
    #[serde(rename = "it.fattureincloud.webhooks.issued_documents.quotes.update")]
    IssuedDocumentsPeriodQuotesPeriodUpdate,
    #[serde(rename = "it.fattureincloud.webhooks.issued_documents.quotes.delete")]
    IssuedDocumentsPeriodQuotesPeriodDelete,
    #[serde(rename = "it.fattureincloud.webhooks.issued_documents.proformas.create")]
    IssuedDocumentsPeriodProformasPeriodCreate,
    #[serde(rename = "it.fattureincloud.webhooks.issued_documents.proformas.update")]
    IssuedDocumentsPeriodProformasPeriodUpdate,
    #[serde(rename = "it.fattureincloud.webhooks.issued_documents.proformas.delete")]
    IssuedDocumentsPeriodProformasPeriodDelete,
    #[serde(rename = "it.fattureincloud.webhooks.issued_documents.receipts.create")]
    IssuedDocumentsPeriodReceiptsPeriodCreate,
    #[serde(rename = "it.fattureincloud.webhooks.issued_documents.receipts.update")]
    IssuedDocumentsPeriodReceiptsPeriodUpdate,
    #[serde(rename = "it.fattureincloud.webhooks.issued_documents.receipts.delete")]
    IssuedDocumentsPeriodReceiptsPeriodDelete,
    #[serde(rename = "it.fattureincloud.webhooks.issued_documents.delivery_notes.create")]
    IssuedDocumentsPeriodDeliveryNotesPeriodCreate,
    #[serde(rename = "it.fattureincloud.webhooks.issued_documents.delivery_notes.update")]
    IssuedDocumentsPeriodDeliveryNotesPeriodUpdate,
    #[serde(rename = "it.fattureincloud.webhooks.issued_documents.delivery_notes.delete")]
    IssuedDocumentsPeriodDeliveryNotesPeriodDelete,
    #[serde(rename = "it.fattureincloud.webhooks.issued_documents.credit_notes.create")]
    IssuedDocumentsPeriodCreditNotesPeriodCreate,
    #[serde(rename = "it.fattureincloud.webhooks.issued_documents.credit_notes.update")]
    IssuedDocumentsPeriodCreditNotesPeriodUpdate,
    #[serde(rename = "it.fattureincloud.webhooks.issued_documents.credit_notes.delete")]
    IssuedDocumentsPeriodCreditNotesPeriodDelete,
    #[serde(rename = "it.fattureincloud.webhooks.issued_documents.orders.create")]
    IssuedDocumentsPeriodOrdersPeriodCreate,
    #[serde(rename = "it.fattureincloud.webhooks.issued_documents.orders.update")]
    IssuedDocumentsPeriodOrdersPeriodUpdate,
    #[serde(rename = "it.fattureincloud.webhooks.issued_documents.orders.delete")]
    IssuedDocumentsPeriodOrdersPeriodDelete,
    #[serde(rename = "it.fattureincloud.webhooks.issued_documents.work_reports.create")]
    IssuedDocumentsPeriodWorkReportsPeriodCreate,
    #[serde(rename = "it.fattureincloud.webhooks.issued_documents.work_reports.update")]
    IssuedDocumentsPeriodWorkReportsPeriodUpdate,
    #[serde(rename = "it.fattureincloud.webhooks.issued_documents.work_reports.delete")]
    IssuedDocumentsPeriodWorkReportsPeriodDelete,
    #[serde(rename = "it.fattureincloud.webhooks.issued_documents.supplier_orders.create")]
    IssuedDocumentsPeriodSupplierOrdersPeriodCreate,
    #[serde(rename = "it.fattureincloud.webhooks.issued_documents.supplier_orders.update")]
    IssuedDocumentsPeriodSupplierOrdersPeriodUpdate,
    #[serde(rename = "it.fattureincloud.webhooks.issued_documents.supplier_orders.delete")]
    IssuedDocumentsPeriodSupplierOrdersPeriodDelete,
    #[serde(rename = "it.fattureincloud.webhooks.issued_documents.self_invoices.create")]
    IssuedDocumentsPeriodSelfInvoicesPeriodCreate,
    #[serde(rename = "it.fattureincloud.webhooks.issued_documents.self_invoices.update")]
    IssuedDocumentsPeriodSelfInvoicesPeriodUpdate,
    #[serde(rename = "it.fattureincloud.webhooks.issued_documents.self_invoices.delete")]
    IssuedDocumentsPeriodSelfInvoicesPeriodDelete,
    #[serde(rename = "it.fattureincloud.webhooks.issued_documents.all.create")]
    IssuedDocumentsPeriodAllPeriodCreate,
    #[serde(rename = "it.fattureincloud.webhooks.issued_documents.all.update")]
    IssuedDocumentsPeriodAllPeriodUpdate,
    #[serde(rename = "it.fattureincloud.webhooks.issued_documents.all.delete")]
    IssuedDocumentsPeriodAllPeriodDelete,
    #[serde(rename = "it.fattureincloud.webhooks.received_documents.create")]
    ReceivedDocumentsPeriodCreate,
    #[serde(rename = "it.fattureincloud.webhooks.received_documents.update")]
    ReceivedDocumentsPeriodUpdate,
    #[serde(rename = "it.fattureincloud.webhooks.received_documents.delete")]
    ReceivedDocumentsPeriodDelete,
    #[serde(rename = "it.fattureincloud.webhooks.receipts.create")]
    ReceiptsPeriodCreate,
    #[serde(rename = "it.fattureincloud.webhooks.receipts.update")]
    ReceiptsPeriodUpdate,
    #[serde(rename = "it.fattureincloud.webhooks.receipts.delete")]
    ReceiptsPeriodDelete,
    #[serde(rename = "it.fattureincloud.webhooks.taxes.create")]
    TaxesPeriodCreate,
    #[serde(rename = "it.fattureincloud.webhooks.taxes.update")]
    TaxesPeriodUpdate,
    #[serde(rename = "it.fattureincloud.webhooks.taxes.delete")]
    TaxesPeriodDelete,
    #[serde(rename = "it.fattureincloud.webhooks.archive_documents.create")]
    ArchiveDocumentsPeriodCreate,
    #[serde(rename = "it.fattureincloud.webhooks.archive_documents.update")]
    ArchiveDocumentsPeriodUpdate,
    #[serde(rename = "it.fattureincloud.webhooks.archive_documents.delete")]
    ArchiveDocumentsPeriodDelete,
    #[serde(rename = "it.fattureincloud.webhooks.cashbook.create")]
    CashbookPeriodCreate,
    #[serde(rename = "it.fattureincloud.webhooks.cashbook.update")]
    CashbookPeriodUpdate,
    #[serde(rename = "it.fattureincloud.webhooks.cashbook.delete")]
    CashbookPeriodDelete,
    #[serde(rename = "it.fattureincloud.webhooks.products.create")]
    ProductsPeriodCreate,
    #[serde(rename = "it.fattureincloud.webhooks.products.update")]
    ProductsPeriodUpdate,
    #[serde(rename = "it.fattureincloud.webhooks.products.delete")]
    ProductsPeriodDelete,
    #[serde(rename = "it.fattureincloud.webhooks.products.stock_update")]
    ProductsPeriodStockUpdate,
    #[serde(rename = "it.fattureincloud.webhooks.entities.clients.create")]
    EntitiesPeriodClientsPeriodCreate,
    #[serde(rename = "it.fattureincloud.webhooks.entities.clients.update")]
    EntitiesPeriodClientsPeriodUpdate,
    #[serde(rename = "it.fattureincloud.webhooks.entities.clients.delete")]
    EntitiesPeriodClientsPeriodDelete,
    #[serde(rename = "it.fattureincloud.webhooks.entities.suppliers.create")]
    EntitiesPeriodSuppliersPeriodCreate,
    #[serde(rename = "it.fattureincloud.webhooks.entities.suppliers.update")]
    EntitiesPeriodSuppliersPeriodUpdate,
    #[serde(rename = "it.fattureincloud.webhooks.entities.suppliers.delete")]
    EntitiesPeriodSuppliersPeriodDelete,
    #[serde(rename = "it.fattureincloud.webhooks.entities.all.create")]
    EntitiesPeriodAllPeriodCreate,
    #[serde(rename = "it.fattureincloud.webhooks.entities.all.update")]
    EntitiesPeriodAllPeriodUpdate,
    #[serde(rename = "it.fattureincloud.webhooks.entities.all.delete")]
    EntitiesPeriodAllPeriodDelete,
    #[serde(rename = "it.fattureincloud.webhooks.issued_documents.e_invoices.status_update")]
    IssuedDocumentsPeriodEInvoicesPeriodStatusUpdate,
    #[serde(rename = "it.fattureincloud.webhooks.received_documents.e_invoices.receive")]
    ReceivedDocumentsPeriodEInvoicesPeriodReceive,
}

impl Default for Types {
    fn default() -> Types {
        Self::IssuedDocumentsPeriodInvoicesPeriodCreate
    }
}

