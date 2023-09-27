/*
 * Fatture in Cloud API v2 - API Reference
 *
 * Connect your software with Fatture in Cloud, the invoicing platform chosen by more than 500.000 businesses in Italy.   The Fatture in Cloud API is based on REST, and makes possible to interact with the user related data prior authorization via OAuth2 protocol.
 *
 * The version of the OpenAPI document: 2.0.29
 * Contact: info@fattureincloud.it
 * Generated by: https://openapi-generator.tech
 */

/// IssuedDocumentExtraData : Issued document extra data [TS fields follow the technical specifications provided by \"Sistema Tessera Sanitaria\"]



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IssuedDocumentExtraData {
    #[serde(rename = "show_sofort_button", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub show_sofort_button: Option<Option<bool>>,
    #[serde(rename = "multifatture_sent", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub multifatture_sent: Option<Option<i32>>,
    /// Send issued document to \"Sistema Tessera Sanitaria\"
    #[serde(rename = "ts_communication", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub ts_communication: Option<Option<bool>>,
    /// Issued document ts \"tipo spesa\" [TK, FC, FV, SV,SP, AD, AS, ECG, SR]
    #[serde(rename = "ts_flag_tipo_spesa", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub ts_flag_tipo_spesa: Option<Option<f32>>,
    /// Issued document ts traced payment
    #[serde(rename = "ts_pagamento_tracciato", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub ts_pagamento_tracciato: Option<Option<bool>>,
    /// Can be [ 'TK', 'FC', 'FV', 'SV', 'SP', 'AD', 'AS', 'SR', 'CT', 'PI', 'IC', 'AA' ]. Refer to the technical specifications to learn more.
    #[serde(rename = "ts_tipo_spesa", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub ts_tipo_spesa: Option<Option<String>>,
    /// Issued document ts \"opposizione\"
    #[serde(rename = "ts_opposizione", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub ts_opposizione: Option<Option<bool>>,
    /// Issued document ts status
    #[serde(rename = "ts_status", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub ts_status: Option<Option<i32>>,
    /// Issued document ts file id
    #[serde(rename = "ts_file_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub ts_file_id: Option<Option<String>>,
    /// Issued document ts sent date
    #[serde(rename = "ts_sent_date", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub ts_sent_date: Option<Option<String>>,
    /// Issued document ts total amount
    #[serde(rename = "ts_full_amount", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub ts_full_amount: Option<Option<bool>>,
    /// Issued document imported by software
    #[serde(rename = "imported_by", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub imported_by: Option<Option<String>>,
}

impl IssuedDocumentExtraData {
    /// Issued document extra data [TS fields follow the technical specifications provided by \"Sistema Tessera Sanitaria\"]
    pub fn new() -> IssuedDocumentExtraData {
        IssuedDocumentExtraData {
            show_sofort_button: None,
            multifatture_sent: None,
            ts_communication: None,
            ts_flag_tipo_spesa: None,
            ts_pagamento_tracciato: None,
            ts_tipo_spesa: None,
            ts_opposizione: None,
            ts_status: None,
            ts_file_id: None,
            ts_sent_date: None,
            ts_full_amount: None,
            imported_by: None,
        }
    }
}


