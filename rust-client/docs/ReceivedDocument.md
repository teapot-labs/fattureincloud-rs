# ReceivedDocument

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | Received document id | [optional]
**r#type** | Option<**String**> | Received document type | [optional][default to Expense]
**entity** | Option<[**crate::models::Entity**](Entity.md)> |  | [optional]
**date** | Option<[**String**](string.md)> | Received document date [defaults to today's date] | [optional]
**category** | Option<**String**> | Received document category | [optional]
**description** | Option<**String**> | Received document description | [optional]
**amount_net** | Option<**f32**> | Received document total net amount | [optional]
**amount_vat** | Option<**f32**> | Received document total vat amount | [optional]
**amount_withholding_tax** | Option<**f32**> | Received document withholding tax amount | [optional]
**amount_other_withholding_tax** | Option<**f32**> | Received document other withholding tax amount | [optional]
**amount_gross** | Option<**f32**> | [Read Only] Received document total gross amount | [optional][readonly]
**amortization** | Option<**f32**> | Received document amortization value | [optional]
**rc_center** | Option<**String**> | Received document revenue center | [optional]
**invoice_number** | Option<**String**> | Received document invoice number | [optional]
**is_marked** | Option<**bool**> | Received document is marked | [optional]
**is_detailed** | Option<**bool**> | Received document has items | [optional]
**e_invoice** | Option<**bool**> | [Read Only] Received document is an e-invoice | [optional]
**next_due_date** | Option<[**String**](string.md)> | [Read Only] Received document date of the next not paid payment | [optional][readonly]
**currency** | Option<[**crate::models::Currency**](Currency.md)> |  | [optional]
**tax_deductibility** | Option<**f32**> | Received document tax deducibility percentage | [optional]
**vat_deductibility** | Option<**f32**> | Received document vat deducibility percentage | [optional]
**items_list** | Option<[**Vec<crate::models::ReceivedDocumentItemsListItem>**](ReceivedDocumentItemsListItem.md)> |  | [optional]
**payments_list** | Option<[**Vec<crate::models::ReceivedDocumentPaymentsListItem>**](ReceivedDocumentPaymentsListItem.md)> |  | [optional]
**attachment_url** | Option<**String**> | [Temporary] [Read Only] Received document url of the attached file | [optional][readonly]
**attachment_preview_url** | Option<**String**> | [Temporary] [Read Only] Received document url of the attachment preview | [optional][readonly]
**auto_calculate** | Option<**bool**> | Received document total items amount and total payments amount can differ if this field is set to false | [optional]
**attachment_token** | Option<**String**> | [Write Only] Received document attachment token returned by POST /received_documents/attachment | [optional]
**created_at** | Option<**String**> | Received document creation date | [optional]
**updated_at** | Option<**String**> | Received document last update date | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


