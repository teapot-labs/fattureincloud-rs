# IssuedDocumentPaymentsListItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | Issued document payment item id | [optional]
**due_date** | Option<[**String**](string.md)> | Issued document payment due date | [optional]
**amount** | Option<**f32**> | Issued document payment amount | [optional]
**status** | Option<**String**> | Issued document status | [optional][default to NotPaid]
**payment_account** | Option<[**crate::models::PaymentAccount**](PaymentAccount.md)> |  | [optional]
**paid_date** | Option<[**String**](string.md)> | Issued document payment date [Only if status is paid] | [optional]
**ei_raw** | Option<[**serde_json::Value**](.md)> | Issued document payment advanced raw attributes for e-invoices | [optional]
**payment_terms** | Option<[**crate::models::IssuedDocumentPaymentsListItemPaymentTerms**](IssuedDocumentPaymentsListItem_payment_terms.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


