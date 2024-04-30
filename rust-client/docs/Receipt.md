# Receipt

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | Receipt id | [optional]
**date** | Option<[**String**](string.md)> | Receipt date | [optional]
**number** | Option<**f64**> | Receipt number | [optional]
**numeration** | Option<**String**> | Receipt numeration | [optional]
**amount_net** | Option<**f64**> | Receipt total net amount | [optional]
**amount_vat** | Option<**f64**> | Receipt total vat amount | [optional]
**amount_gross** | Option<**f64**> | Receipt total gross amount | [optional]
**use_gross_prices** | Option<**bool**> | Receipt uses gross prices | [optional]
**r#type** | Option<**String**> | Receipt type | [optional]
**description** | Option<**String**> | Receipt description | [optional]
**rc_center** | Option<**String**> | Receipt revenue center | [optional]
**created_at** | Option<**String**> | Receipt creation date | [optional]
**updated_at** | Option<**String**> | Receipt last update date | [optional]
**payment_account** | Option<[**models::PaymentAccount**](PaymentAccount.md)> |  | [optional]
**items_list** | Option<[**Vec<models::ReceiptItemsListItem>**](ReceiptItemsListItem.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


