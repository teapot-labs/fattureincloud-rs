# IssuedDocumentItemsListItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | Issued document item id | [optional]
**product_id** | Option<**i32**> | Issued document item product id | [optional]
**code** | Option<**String**> | Issued document item product code | [optional]
**name** | Option<**String**> | Issued document item product name | [optional]
**category** | Option<**String**> | Issued document item product category | [optional]
**description** | Option<**String**> | Issued document product description | [optional]
**qty** | Option<**f32**> | Issued document item quantity | [optional]
**measure** | Option<**String**> | Issued document item measure | [optional]
**net_price** | Option<**f32**> | Issued document item net price | [optional]
**gross_price** | Option<**f32**> | Issued document item gross price | [optional]
**vat** | Option<[**crate::models::VatType**](VatType.md)> |  | [optional]
**not_taxable** | Option<**bool**> | Issued document item is not taxable | [optional]
**apply_withholding_taxes** | Option<**bool**> | Issued document item apply withholding taxes, rivalsa and cassa | [optional]
**discount** | Option<**f32**> | Issued document item discount percentual value | [optional]
**discount_highlight** | Option<**bool**> | Issued document item highlight discount | [optional]
**in_dn** | Option<**bool**> | Issued document item add in delivery note | [optional]
**stock** | Option<**bool**> | Issued document item move stock | [optional]
**ei_raw** | Option<[**serde_json::Value**](.md)> | Issued document advanced raw attributes for e-invoices | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


