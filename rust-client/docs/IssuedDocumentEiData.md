# IssuedDocumentEiData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**vat_kind** | Option<**String**> | Vat kind [esigibilità IVA] | [optional]
**original_document_type** | Option<**String**> | Issued document original document type | [optional][default to Ordine]
**od_number** | Option<**String**> | E-invoice original document number | [optional]
**od_date** | Option<[**String**](string.md)> | E-invoice original document date | [optional]
**cig** | Option<**String**> | E-invoice CIG | [optional]
**cup** | Option<**String**> | E-invoice CUP | [optional]
**payment_method** | Option<**String**> | E-invoice payment method [required for e-invoices] (see [here](https://www.fatturapa.gov.it/export/documenti/fatturapa/v1.2.2/Rappresentazione_Tabellare_FattOrdinaria_V1.2.2.pdf) for the accepted values of ModalitaPagamento) | [optional]
**bank_name** | Option<**String**> | E-invoice bank name | [optional]
**bank_iban** | Option<**String**> | E-invoice bank IBAN | [optional]
**bank_beneficiary** | Option<**String**> | E-invoice bank beneficiary | [optional]
**invoice_number** | Option<**String**> | E-invoice invoice number | [optional]
**invoice_date** | Option<[**String**](string.md)> | E-invoice invoice date | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


