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
pub struct IssuedDocument1 {
    /// Issued document id
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "entity", skip_serializing_if = "Option::is_none")]
    pub entity: Option<Box<models::Entity1>>,
    /// Issued document type
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
    /// Issued document number [If not specified, next number is used]
    #[serde(rename = "number", skip_serializing_if = "Option::is_none")]
    pub number: Option<i32>,
    /// Issued document numeration [Not available if type=delivery_note]
    #[serde(rename = "numeration", skip_serializing_if = "Option::is_none")]
    pub numeration: Option<String>,
    /// Issued document date [defaults to today's date]
    #[serde(rename = "date", skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    /// Issued document year
    #[serde(rename = "year", skip_serializing_if = "Option::is_none")]
    pub year: Option<i32>,
    #[serde(rename = "currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<Box<models::Currency1>>,
    #[serde(rename = "language", skip_serializing_if = "Option::is_none")]
    pub language: Option<Box<models::Language1>>,
    /// Issued document subject
    #[serde(rename = "subject", skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    /// Issued document visible subject
    #[serde(rename = "visible_subject", skip_serializing_if = "Option::is_none")]
    pub visible_subject: Option<String>,
    /// Issued document revenue center [or cost center if type=supplier_order].
    #[serde(rename = "rc_center", skip_serializing_if = "Option::is_none")]
    pub rc_center: Option<String>,
    /// Issued document extra notes
    #[serde(rename = "notes", skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    /// Issued document \"Rivalsa INPS\" percentual value
    #[serde(rename = "rivalsa", skip_serializing_if = "Option::is_none")]
    pub rivalsa: Option<f64>,
    /// Issued document \"Cassa previdenziale\" percentual value
    #[serde(rename = "cassa", skip_serializing_if = "Option::is_none")]
    pub cassa: Option<f64>,
    /// [Read Only] Issued document cassa amount.
    #[serde(rename = "amount_cassa", skip_serializing_if = "Option::is_none")]
    pub amount_cassa: Option<f64>,
    /// Issued document cassa taxable percentage
    #[serde(rename = "cassa_taxable", skip_serializing_if = "Option::is_none")]
    pub cassa_taxable: Option<f64>,
    /// [Can be set only if cassa_taxable is NULL] Issued document cassa taxable amount
    #[serde(rename = "amount_cassa_taxable", skip_serializing_if = "Option::is_none")]
    pub amount_cassa_taxable: Option<f64>,
    /// Issued document \"Cassa previdenziale 2\" percentual value
    #[serde(rename = "cassa2", skip_serializing_if = "Option::is_none")]
    pub cassa2: Option<f64>,
    /// [Read Only] Issued document cassa2 amount
    #[serde(rename = "amount_cassa2", skip_serializing_if = "Option::is_none")]
    pub amount_cassa2: Option<f64>,
    /// Issued document cassa2 taxable percentage
    #[serde(rename = "cassa2_taxable", skip_serializing_if = "Option::is_none")]
    pub cassa2_taxable: Option<f64>,
    /// [Can be set only if cassa2_taxable is NULL] Issued document cassa2 taxable amount
    #[serde(rename = "amount_cassa2_taxable", skip_serializing_if = "Option::is_none")]
    pub amount_cassa2_taxable: Option<f64>,
    /// Issued document global cassa taxable percentage
    #[serde(rename = "global_cassa_taxable", skip_serializing_if = "Option::is_none")]
    pub global_cassa_taxable: Option<f64>,
    /// [Can be set only if global_cassa_taxable is NULL] Issued document global cassa taxable amount
    #[serde(rename = "amount_global_cassa_taxable", skip_serializing_if = "Option::is_none")]
    pub amount_global_cassa_taxable: Option<f64>,
    /// Issued document withholding tax (ritenuta d'acconto) percentual value
    #[serde(rename = "withholding_tax", skip_serializing_if = "Option::is_none")]
    pub withholding_tax: Option<f64>,
    /// Issued document withholding tax taxable (imponibile) percentual value
    #[serde(rename = "withholding_tax_taxable", skip_serializing_if = "Option::is_none")]
    pub withholding_tax_taxable: Option<f64>,
    /// Issued document other withholding tax (altra ritenuta) percentual value
    #[serde(rename = "other_withholding_tax", skip_serializing_if = "Option::is_none")]
    pub other_withholding_tax: Option<f64>,
    /// Issued document stamp duty value [0 if not present]
    #[serde(rename = "stamp_duty", skip_serializing_if = "Option::is_none")]
    pub stamp_duty: Option<f64>,
    #[serde(rename = "payment_method", skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<Box<models::PaymentMethod1>>,
    /// Issued document uses split payment
    #[serde(rename = "use_split_payment", skip_serializing_if = "Option::is_none")]
    pub use_split_payment: Option<bool>,
    /// Issued document uses gross prices
    #[serde(rename = "use_gross_prices", skip_serializing_if = "Option::is_none")]
    pub use_gross_prices: Option<bool>,
    /// Issued document is an e-invoice.
    #[serde(rename = "e_invoice", skip_serializing_if = "Option::is_none")]
    pub e_invoice: Option<bool>,
    #[serde(rename = "ei_data", skip_serializing_if = "Option::is_none")]
    pub ei_data: Option<Box<models::IssuedDocument1EiData>>,
    /// E-invoice cassa type
    #[serde(rename = "ei_cassa_type", skip_serializing_if = "Option::is_none")]
    pub ei_cassa_type: Option<String>,
    /// E-invoice cassa2 type
    #[serde(rename = "ei_cassa2_type", skip_serializing_if = "Option::is_none")]
    pub ei_cassa2_type: Option<String>,
    /// E-invoice withholding tax causal
    #[serde(rename = "ei_withholding_tax_causal", skip_serializing_if = "Option::is_none")]
    pub ei_withholding_tax_causal: Option<String>,
    /// E-invoice other withholding tax type
    #[serde(rename = "ei_other_withholding_tax_type", skip_serializing_if = "Option::is_none")]
    pub ei_other_withholding_tax_type: Option<String>,
    /// E-invoice other withholding tax causal
    #[serde(rename = "ei_other_withholding_tax_causal", skip_serializing_if = "Option::is_none")]
    pub ei_other_withholding_tax_causal: Option<String>,
    #[serde(rename = "items_list", skip_serializing_if = "Option::is_none")]
    pub items_list: Option<Vec<models::IssuedDocumentItemsListItem>>,
    #[serde(rename = "payments_list", skip_serializing_if = "Option::is_none")]
    pub payments_list: Option<Vec<models::IssuedDocumentPaymentsListItem>>,
    #[serde(rename = "template", skip_serializing_if = "Option::is_none")]
    pub template: Option<Box<models::DocumentTemplate1>>,
    #[serde(rename = "delivery_note_template", skip_serializing_if = "Option::is_none")]
    pub delivery_note_template: Option<Box<models::DocumentTemplate1>>,
    #[serde(rename = "acc_inv_template", skip_serializing_if = "Option::is_none")]
    pub acc_inv_template: Option<Box<models::DocumentTemplate1>>,
    /// Issued document PDF horizontal margins
    #[serde(rename = "h_margins", skip_serializing_if = "Option::is_none")]
    pub h_margins: Option<i32>,
    /// Issued document PDF vertical margins
    #[serde(rename = "v_margins", skip_serializing_if = "Option::is_none")]
    pub v_margins: Option<i32>,
    /// Show the expiration dates of the payments on the document
    #[serde(rename = "show_payments", skip_serializing_if = "Option::is_none")]
    pub show_payments: Option<bool>,
    /// Show the payment method details on the document
    #[serde(rename = "show_payment_method", skip_serializing_if = "Option::is_none")]
    pub show_payment_method: Option<bool>,
    /// Show totals mode
    #[serde(rename = "show_totals", skip_serializing_if = "Option::is_none")]
    pub show_totals: Option<ShowTotals>,
    /// Show notification button in the PDF
    #[serde(rename = "show_notification_button", skip_serializing_if = "Option::is_none")]
    pub show_notification_button: Option<bool>,
    /// Show ts pay button in the PDF
    #[serde(rename = "show_tspay_button", skip_serializing_if = "Option::is_none")]
    pub show_tspay_button: Option<bool>,
    /// Issued document has delivery note
    #[serde(rename = "delivery_note", skip_serializing_if = "Option::is_none")]
    pub delivery_note: Option<bool>,
    /// Issued document has an accompanying invoice
    #[serde(rename = "accompanying_invoice", skip_serializing_if = "Option::is_none")]
    pub accompanying_invoice: Option<bool>,
    /// Issued document attached delivery note number
    #[serde(rename = "dn_number", skip_serializing_if = "Option::is_none")]
    pub dn_number: Option<i32>,
    /// Issued document attached delivery note date
    #[serde(rename = "dn_date", skip_serializing_if = "Option::is_none")]
    pub dn_date: Option<String>,
    /// Issued document attached delivery note number of packages
    #[serde(rename = "dn_ai_packages_number", skip_serializing_if = "Option::is_none")]
    pub dn_ai_packages_number: Option<String>,
    /// Issued document attached delivery note package weight
    #[serde(rename = "dn_ai_weight", skip_serializing_if = "Option::is_none")]
    pub dn_ai_weight: Option<String>,
    /// Issued document attached delivery note causal
    #[serde(rename = "dn_ai_causal", skip_serializing_if = "Option::is_none")]
    pub dn_ai_causal: Option<String>,
    /// Issued document attached delivery note destination
    #[serde(rename = "dn_ai_destination", skip_serializing_if = "Option::is_none")]
    pub dn_ai_destination: Option<String>,
    /// Issued document attached delivery note transporter
    #[serde(rename = "dn_ai_transporter", skip_serializing_if = "Option::is_none")]
    pub dn_ai_transporter: Option<String>,
    /// Issued document attached delivery note notes
    #[serde(rename = "dn_ai_notes", skip_serializing_if = "Option::is_none")]
    pub dn_ai_notes: Option<String>,
    /// Issued document is marked
    #[serde(rename = "is_marked", skip_serializing_if = "Option::is_none")]
    pub is_marked: Option<bool>,
    /// [Read only] Issued document total net amount
    #[serde(rename = "amount_net", skip_serializing_if = "Option::is_none")]
    pub amount_net: Option<f64>,
    /// [Read Only] Issued document total vat amount
    #[serde(rename = "amount_vat", skip_serializing_if = "Option::is_none")]
    pub amount_vat: Option<f64>,
    /// [Read Only] Issued document total gross amount
    #[serde(rename = "amount_gross", skip_serializing_if = "Option::is_none")]
    pub amount_gross: Option<f64>,
    /// Issued document amount due discount
    #[serde(rename = "amount_due_discount", skip_serializing_if = "Option::is_none")]
    pub amount_due_discount: Option<f64>,
    /// [Read Only] Issued document rivalsa amount
    #[serde(rename = "amount_rivalsa", skip_serializing_if = "Option::is_none")]
    pub amount_rivalsa: Option<f64>,
    /// Issued document taxable rivalsa amount
    #[serde(rename = "amount_rivalsa_taxable", skip_serializing_if = "Option::is_none")]
    pub amount_rivalsa_taxable: Option<f64>,
    /// [Read Only] Issued document withholding tax amount (ritenuta d'acconto).
    #[serde(rename = "amount_withholding_tax", skip_serializing_if = "Option::is_none")]
    pub amount_withholding_tax: Option<f64>,
    /// Issued document taxable withholding tax amount
    #[serde(rename = "amount_withholding_tax_taxable", skip_serializing_if = "Option::is_none")]
    pub amount_withholding_tax_taxable: Option<f64>,
    /// [Read Only] Issued document other withholding tax amount (altra ritenuta)
    #[serde(rename = "amount_other_withholding_tax", skip_serializing_if = "Option::is_none")]
    pub amount_other_withholding_tax: Option<f64>,
    /// Issued document taxable other withholding tax amount
    #[serde(rename = "amount_other_withholding_tax_taxable", skip_serializing_if = "Option::is_none")]
    pub amount_other_withholding_tax_taxable: Option<f64>,
    /// Issued document taxable enasarco amount
    #[serde(rename = "amount_enasarco_taxable", skip_serializing_if = "Option::is_none")]
    pub amount_enasarco_taxable: Option<f64>,
    #[serde(rename = "extra_data", skip_serializing_if = "Option::is_none")]
    pub extra_data: Option<Box<models::IssuedDocument1ExtraData>>,
    /// Issued document seen date
    #[serde(rename = "seen_date", skip_serializing_if = "Option::is_none")]
    pub seen_date: Option<String>,
    /// Issued document date of the next not paid payment
    #[serde(rename = "next_due_date", skip_serializing_if = "Option::is_none")]
    pub next_due_date: Option<String>,
    /// [Temporary] [Read Only] Issued document url of the document PDF file
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// [Temporary] [Read Only] Issued document url of the attached delivery note PDF file
    #[serde(rename = "dn_url", skip_serializing_if = "Option::is_none")]
    pub dn_url: Option<String>,
    /// [Temporary] [Read Only] Issued document url of the accompanying invoice PDF file
    #[serde(rename = "ai_url", skip_serializing_if = "Option::is_none")]
    pub ai_url: Option<String>,
    /// [Temporary] [Read Only] Issued document url of the attached file
    #[serde(rename = "attachment_url", skip_serializing_if = "Option::is_none")]
    pub attachment_url: Option<String>,
    /// [Write Only] Issued document attachment token returned by POST /issued_documents/attachment
    #[serde(rename = "attachment_token", skip_serializing_if = "Option::is_none")]
    pub attachment_token: Option<String>,
    /// Issued document advanced raw attributes for e-invoices
    #[serde(rename = "ei_raw", skip_serializing_if = "Option::is_none")]
    pub ei_raw: Option<serde_json::Value>,
    /// [Read only] Status of the e-invoice.   * **attempt** - We are trying to send the invoice, please wait up to 2 hours   * **missing** - The invoice is missing   * **not_sent** - The invoice has yet to be sent   * **sent** - The invoice was sent   * **pending** - The checks for the digital signature and sending are in progress   * **processing** - The SDI is delivering the invoice to the customer   * **error** - An error occurred while handling the invoice, please try to resend it or contact support   * **discarded** - The invoice has been rejected by the SDI, so it must be corrected and re-sent   * **not_delivered** - The SDI was unable to deliver the invoice   * **accepted** - The customer accepted the invoice   * **rejected** - The customer rejected the invoice, so it must be corrected   * **no_response** - A response has not yet been received whithin the deadline, contact the customer to ascertain the status of the invoice   * **manual_accepted** - The customer accepted the invoice   * **manual_rejected** - The customer rejected the invoice 
    #[serde(rename = "ei_status", skip_serializing_if = "Option::is_none")]
    pub ei_status: Option<EiStatus>,
    /// Issued Document can't be edited
    #[serde(rename = "locked", skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
    /// Issued document creation date
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Issued document last update date
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

impl IssuedDocument1 {
    pub fn new() -> IssuedDocument1 {
        IssuedDocument1 {
            id: None,
            entity: None,
            r#type: None,
            number: None,
            numeration: None,
            date: None,
            year: None,
            currency: None,
            language: None,
            subject: None,
            visible_subject: None,
            rc_center: None,
            notes: None,
            rivalsa: None,
            cassa: None,
            amount_cassa: None,
            cassa_taxable: None,
            amount_cassa_taxable: None,
            cassa2: None,
            amount_cassa2: None,
            cassa2_taxable: None,
            amount_cassa2_taxable: None,
            global_cassa_taxable: None,
            amount_global_cassa_taxable: None,
            withholding_tax: None,
            withholding_tax_taxable: None,
            other_withholding_tax: None,
            stamp_duty: None,
            payment_method: None,
            use_split_payment: None,
            use_gross_prices: None,
            e_invoice: None,
            ei_data: None,
            ei_cassa_type: None,
            ei_cassa2_type: None,
            ei_withholding_tax_causal: None,
            ei_other_withholding_tax_type: None,
            ei_other_withholding_tax_causal: None,
            items_list: None,
            payments_list: None,
            template: None,
            delivery_note_template: None,
            acc_inv_template: None,
            h_margins: None,
            v_margins: None,
            show_payments: None,
            show_payment_method: None,
            show_totals: None,
            show_notification_button: None,
            show_tspay_button: None,
            delivery_note: None,
            accompanying_invoice: None,
            dn_number: None,
            dn_date: None,
            dn_ai_packages_number: None,
            dn_ai_weight: None,
            dn_ai_causal: None,
            dn_ai_destination: None,
            dn_ai_transporter: None,
            dn_ai_notes: None,
            is_marked: None,
            amount_net: None,
            amount_vat: None,
            amount_gross: None,
            amount_due_discount: None,
            amount_rivalsa: None,
            amount_rivalsa_taxable: None,
            amount_withholding_tax: None,
            amount_withholding_tax_taxable: None,
            amount_other_withholding_tax: None,
            amount_other_withholding_tax_taxable: None,
            amount_enasarco_taxable: None,
            extra_data: None,
            seen_date: None,
            next_due_date: None,
            url: None,
            dn_url: None,
            ai_url: None,
            attachment_url: None,
            attachment_token: None,
            ei_raw: None,
            ei_status: None,
            locked: None,
            created_at: None,
            updated_at: None,
        }
    }
}
/// Issued document type
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "invoice")]
    Invoice,
    #[serde(rename = "quote")]
    Quote,
    #[serde(rename = "proforma")]
    Proforma,
    #[serde(rename = "receipt")]
    Receipt,
    #[serde(rename = "delivery_note")]
    DeliveryNote,
    #[serde(rename = "credit_note")]
    CreditNote,
    #[serde(rename = "order")]
    Order,
    #[serde(rename = "work_report")]
    WorkReport,
    #[serde(rename = "supplier_order")]
    SupplierOrder,
    #[serde(rename = "self_own_invoice")]
    SelfOwnInvoice,
    #[serde(rename = "self_supplier_invoice")]
    SelfSupplierInvoice,
}

impl Default for Type {
    fn default() -> Type {
        Self::Invoice
    }
}
/// Show totals mode
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ShowTotals {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "nets")]
    Nets,
    #[serde(rename = "all")]
    All,
}

impl Default for ShowTotals {
    fn default() -> ShowTotals {
        Self::None
    }
}
/// [Read only] Status of the e-invoice.   * **attempt** - We are trying to send the invoice, please wait up to 2 hours   * **missing** - The invoice is missing   * **not_sent** - The invoice has yet to be sent   * **sent** - The invoice was sent   * **pending** - The checks for the digital signature and sending are in progress   * **processing** - The SDI is delivering the invoice to the customer   * **error** - An error occurred while handling the invoice, please try to resend it or contact support   * **discarded** - The invoice has been rejected by the SDI, so it must be corrected and re-sent   * **not_delivered** - The SDI was unable to deliver the invoice   * **accepted** - The customer accepted the invoice   * **rejected** - The customer rejected the invoice, so it must be corrected   * **no_response** - A response has not yet been received whithin the deadline, contact the customer to ascertain the status of the invoice   * **manual_accepted** - The customer accepted the invoice   * **manual_rejected** - The customer rejected the invoice 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EiStatus {
    #[serde(rename = "attempt")]
    Attempt,
    #[serde(rename = "missing")]
    Missing,
    #[serde(rename = "not_sent")]
    NotSent,
    #[serde(rename = "sent")]
    Sent,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "processing")]
    Processing,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "discarded")]
    Discarded,
    #[serde(rename = "not_delivered")]
    NotDelivered,
    #[serde(rename = "accepted")]
    Accepted,
    #[serde(rename = "rejected")]
    Rejected,
    #[serde(rename = "no_response")]
    NoResponse,
    #[serde(rename = "manual_accepted")]
    ManualAccepted,
    #[serde(rename = "manual_rejected")]
    ManualRejected,
}

impl Default for EiStatus {
    fn default() -> EiStatus {
        Self::Attempt
    }
}

