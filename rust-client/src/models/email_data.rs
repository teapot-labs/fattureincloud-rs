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
pub struct EmailData {
    /// Email recipient
    #[serde(rename = "recipient_email", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub recipient_email: Option<Option<String>>,
    #[serde(rename = "default_sender_email", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub default_sender_email: Option<Option<Box<crate::models::EmailDataDefaultSenderEmail>>>,
    /// List of all emails from which the document can be sent
    #[serde(rename = "sender_emails_list", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub sender_emails_list: Option<Option<Vec<crate::models::SenderEmail>>>,
    /// Email cc [by default is the logged company email]
    #[serde(rename = "cc_email", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub cc_email: Option<Option<String>>,
    /// Email subject
    #[serde(rename = "subject", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub subject: Option<Option<String>>,
    /// Email body
    #[serde(rename = "body", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub body: Option<Option<String>>,
    /// Document exists if it is not a delivery note
    #[serde(rename = "document_exists", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub document_exists: Option<Option<bool>>,
    /// Document is a delivery note
    #[serde(rename = "delivery_note_exists", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub delivery_note_exists: Option<Option<bool>>,
    /// Document has attachment
    #[serde(rename = "attachment_exists", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub attachment_exists: Option<Option<bool>>,
    /// Document has accompanying invoice
    #[serde(rename = "accompanying_invoice_exists", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub accompanying_invoice_exists: Option<Option<bool>>,
    /// Attach document pdf
    #[serde(rename = "default_attach_pdf", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub default_attach_pdf: Option<Option<bool>>,
}

impl EmailData {
    pub fn new() -> EmailData {
        EmailData {
            recipient_email: None,
            default_sender_email: None,
            sender_emails_list: None,
            cc_email: None,
            subject: None,
            body: None,
            document_exists: None,
            delivery_note_exists: None,
            attachment_exists: None,
            accompanying_invoice_exists: None,
            default_attach_pdf: None,
        }
    }
}

