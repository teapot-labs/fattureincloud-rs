# IssuedDocumentPreCreateInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**numerations** | Option<[**std::collections::HashMap<String, std::collections::HashMap<String, i32>>**](std::collections::HashMap.md)> | Issued document numerations | [optional]
**dn_numerations** | Option<[**std::collections::HashMap<String, std::collections::HashMap<String, i32>>**](std::collections::HashMap.md)> | Issued document delivery note numerations | [optional]
**default_values** | Option<[**models::IssuedDocumentPreCreateInfoDefaultValues**](IssuedDocumentPreCreateInfo_default_values.md)> |  | [optional]
**extra_data_default_values** | Option<[**models::IssuedDocumentPreCreateInfoExtraDataDefaultValues**](IssuedDocumentPreCreateInfo_extra_data_default_values.md)> |  | [optional]
**items_default_values** | Option<[**models::IssuedDocumentPreCreateInfoItemsDefaultValues**](IssuedDocumentPreCreateInfo_items_default_values.md)> |  | [optional]
**countries_list** | Option<**Vec<String>**> | Countries list | [optional]
**currencies_list** | Option<[**Vec<models::Currency>**](Currency.md)> | Currencies list | [optional]
**templates_list** | Option<[**Vec<models::DocumentTemplate>**](DocumentTemplate.md)> | Document templates list | [optional]
**dn_templates_list** | Option<[**Vec<models::DocumentTemplate>**](DocumentTemplate.md)> | Delivery note templates list | [optional]
**ai_templates_list** | Option<[**Vec<models::DocumentTemplate>**](DocumentTemplate.md)> | Accompanying invoice templates list | [optional]
**payment_methods_list** | Option<[**Vec<models::PaymentMethod>**](PaymentMethod.md)> | Payment methods list | [optional]
**payment_accounts_list** | Option<[**Vec<models::PaymentAccount>**](PaymentAccount.md)> | Payment accounts list | [optional]
**vat_types_list** | Option<[**Vec<models::VatType>**](VatType.md)> | Vat types list | [optional]
**languages_list** | Option<[**Vec<models::Language>**](Language.md)> | Languages list | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


