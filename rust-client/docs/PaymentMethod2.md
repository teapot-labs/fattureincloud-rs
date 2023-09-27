# PaymentMethod2

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | Payment method id | [optional]
**name** | Option<**String**> | Payment method name | [optional]
**r#type** | Option<**String**> | Payment method type | [optional][default to Standard]
**is_default** | Option<**bool**> | Payment method is default | [optional]
**default_payment_account** | Option<[**crate::models::PaymentAccount**](PaymentAccount.md)> |  | [optional]
**details** | Option<[**Vec<crate::models::PaymentMethodDetails>**](PaymentMethodDetails.md)> | Payment method details | [optional]
**bank_iban** | Option<**String**> | Payment method bank iban | [optional]
**bank_name** | Option<**String**> | Payment method bank name | [optional]
**bank_beneficiary** | Option<**String**> | Payment method bank beneficiary | [optional]
**ei_payment_method** | Option<**String**> | E-invoice payment method | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


