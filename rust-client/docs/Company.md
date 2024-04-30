# Company

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | Company id | [optional]
**name** | Option<**String**> | Company name | [optional]
**r#type** | Option<**String**> | Company type | [optional]
**access_token** | Option<**String**> | Company authentication token for this company. [Only if type=company] | [optional]
**controlled_companies** | Option<[**Vec<models::ControlledCompany>**](Controlled_Company.md)> | Company list of controlled companies [Only if type=accountant] | [optional]
**fic_license_expire** | Option<[**String**](string.md)> |  | [optional]
**fic_plan** | Option<**String**> | Fatture in Cloud account plan type. | [optional]
**connection_id** | Option<**i32**> | Company connection id | [optional]
**tax_code** | Option<**String**> | Company tax code | [optional]
**vat_number** | Option<**String**> | Company vat number | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


