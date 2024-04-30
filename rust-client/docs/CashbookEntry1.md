# CashbookEntry1

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Cashbook id | 
**date** | [**String**](string.md) | Cashbook date | 
**description** | **String** | Cashbook description | 
**kind** | **String** | Cashbook kind | 
**r#type** | Option<**String**> | Cashbook type | [optional]
**entity_name** | Option<**String**> | Cashbook entity name | [optional]
**document** | Option<[**models::CashbookEntry1Document**](CashbookEntry_1_document.md)> |  | [optional]
**amount_in** | Option<**f64**> | [Only for cashbook entry in] Cashbook total amount in | [optional]
**payment_account_in** | Option<[**models::PaymentAccount5**](PaymentAccount_5.md)> |  | [optional]
**amount_out** | Option<**f64**> | [Only for cashbook entry out] Cashbook total amount out | [optional]
**payment_account_out** | Option<[**models::PaymentAccount6**](PaymentAccount_6.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


