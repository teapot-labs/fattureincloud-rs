# CashbookEntry

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | Cashbook id | [optional]
**date** | Option<[**String**](string.md)> | Cashbook date | [optional]
**description** | Option<**String**> | Cashbook description | [optional]
**kind** | Option<**String**> | Cashbook kind | [optional]
**r#type** | Option<**String**> | Cashbook type | [optional]
**entity_name** | Option<**String**> | Cashbook entity name | [optional]
**document** | Option<[**crate::models::CashbookEntryDocument**](CashbookEntry_document.md)> |  | [optional]
**amount_in** | Option<**f32**> | [Only for cashbook entry in] Cashbook total amount in | [optional]
**payment_account_in** | Option<[**crate::models::PaymentAccount3**](PaymentAccount_3.md)> |  | [optional]
**amount_out** | Option<**f32**> | [Only for cashbook entry out] Cashbook total amount out | [optional]
**payment_account_out** | Option<[**crate::models::PaymentAccount4**](PaymentAccount_4.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


