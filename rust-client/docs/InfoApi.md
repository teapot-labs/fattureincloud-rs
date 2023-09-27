# \InfoApi

All URIs are relative to *https://api-v2.fattureincloud.it*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_archive_categories**](InfoApi.md#list_archive_categories) | **GET** /c/{company_id}/info/archive_categories | List Archive Categories
[**list_cities**](InfoApi.md#list_cities) | **GET** /info/cities | List Cities
[**list_cost_centers**](InfoApi.md#list_cost_centers) | **GET** /c/{company_id}/info/cost_centers | List Cost Centers
[**list_countries**](InfoApi.md#list_countries) | **GET** /info/countries | List Countries
[**list_currencies**](InfoApi.md#list_currencies) | **GET** /info/currencies | List Currencies
[**list_delivery_notes_default_causals**](InfoApi.md#list_delivery_notes_default_causals) | **GET** /info/dn_causals | List Delivery Notes Default Causals
[**list_detailed_countries**](InfoApi.md#list_detailed_countries) | **GET** /info/detailed_countries | List Detailed Countries
[**list_languages**](InfoApi.md#list_languages) | **GET** /info/languages | List Languages
[**list_payment_accounts**](InfoApi.md#list_payment_accounts) | **GET** /c/{company_id}/info/payment_accounts | List Payment Accounts
[**list_payment_methods**](InfoApi.md#list_payment_methods) | **GET** /c/{company_id}/info/payment_methods | List Payment Methods
[**list_product_categories**](InfoApi.md#list_product_categories) | **GET** /c/{company_id}/info/product_categories | List Product Categories
[**list_received_document_categories**](InfoApi.md#list_received_document_categories) | **GET** /c/{company_id}/info/received_document_categories | List Received Document Categories
[**list_revenue_centers**](InfoApi.md#list_revenue_centers) | **GET** /c/{company_id}/info/revenue_centers | List Revenue Centers
[**list_templates**](InfoApi.md#list_templates) | **GET** /info/templates | List Templates
[**list_units_of_measure**](InfoApi.md#list_units_of_measure) | **GET** /info/measures | List Units of Measure
[**list_vat_types**](InfoApi.md#list_vat_types) | **GET** /c/{company_id}/info/vat_types | List Vat Types



## list_archive_categories

> crate::models::ListArchiveCategoriesResponse list_archive_categories(company_id)
List Archive Categories

Lists the archive categories.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | The ID of the company. | [required] |

### Return type

[**crate::models::ListArchiveCategoriesResponse**](ListArchiveCategoriesResponse.md)

### Authorization

[OAuth2AuthenticationCodeFlow](../README.md#OAuth2AuthenticationCodeFlow), [ManualAuth](../README.md#ManualAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_cities

> crate::models::ListCitiesResponse list_cities(postal_code, city)
List Cities

Lists the Italian cities.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**postal_code** | Option<**String**> | Postal code for filtering. |  |
**city** | Option<**String**> | City for filtering (ignored if postal_code is passed). |  |

### Return type

[**crate::models::ListCitiesResponse**](ListCitiesResponse.md)

### Authorization

[OAuth2AuthenticationCodeFlow](../README.md#OAuth2AuthenticationCodeFlow), [ManualAuth](../README.md#ManualAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_cost_centers

> crate::models::ListCostCentersResponse list_cost_centers(company_id)
List Cost Centers

Lists the cost centers.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | The ID of the company. | [required] |

### Return type

[**crate::models::ListCostCentersResponse**](ListCostCentersResponse.md)

### Authorization

[OAuth2AuthenticationCodeFlow](../README.md#OAuth2AuthenticationCodeFlow), [ManualAuth](../README.md#ManualAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_countries

> crate::models::ListCountriesResponse list_countries()
List Countries

Lists the supported countries.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ListCountriesResponse**](ListCountriesResponse.md)

### Authorization

[OAuth2AuthenticationCodeFlow](../README.md#OAuth2AuthenticationCodeFlow), [ManualAuth](../README.md#ManualAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_currencies

> crate::models::ListCurrenciesResponse list_currencies()
List Currencies

Lists the supported currencies.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ListCurrenciesResponse**](ListCurrenciesResponse.md)

### Authorization

[OAuth2AuthenticationCodeFlow](../README.md#OAuth2AuthenticationCodeFlow), [ManualAuth](../README.md#ManualAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_delivery_notes_default_causals

> crate::models::ListDeliveryNotesDefaultCasualsResponse list_delivery_notes_default_causals()
List Delivery Notes Default Causals

Lists the delivery note default causals.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ListDeliveryNotesDefaultCasualsResponse**](ListDeliveryNotesDefaultCasualsResponse.md)

### Authorization

[OAuth2AuthenticationCodeFlow](../README.md#OAuth2AuthenticationCodeFlow), [ManualAuth](../README.md#ManualAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_detailed_countries

> crate::models::ListDetailedCountriesResponse list_detailed_countries()
List Detailed Countries

Lists the supported countries.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ListDetailedCountriesResponse**](ListDetailedCountriesResponse.md)

### Authorization

[OAuth2AuthenticationCodeFlow](../README.md#OAuth2AuthenticationCodeFlow), [ManualAuth](../README.md#ManualAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_languages

> crate::models::ListLanguagesResponse list_languages()
List Languages

Lists the supported languages.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ListLanguagesResponse**](ListLanguagesResponse.md)

### Authorization

[OAuth2AuthenticationCodeFlow](../README.md#OAuth2AuthenticationCodeFlow), [ManualAuth](../README.md#ManualAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_payment_accounts

> crate::models::ListPaymentAccountsResponse list_payment_accounts(company_id, fields, fieldset, sort)
List Payment Accounts

Lists the available payment accounts.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | The ID of the company. | [required] |
**fields** | Option<**String**> | List of comma-separated fields. |  |
**fieldset** | Option<**String**> | Name of the fieldset. |  |
**sort** | Option<**String**> | List of comma-separated fields for result sorting (minus for desc sorting). |  |

### Return type

[**crate::models::ListPaymentAccountsResponse**](ListPaymentAccountsResponse.md)

### Authorization

[OAuth2AuthenticationCodeFlow](../README.md#OAuth2AuthenticationCodeFlow), [ManualAuth](../README.md#ManualAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_payment_methods

> crate::models::ListPaymentMethodsResponse list_payment_methods(company_id, fields, fieldset, sort)
List Payment Methods

Lists the available payment methods.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | The ID of the company. | [required] |
**fields** | Option<**String**> | List of comma-separated fields. |  |
**fieldset** | Option<**String**> | Name of the fieldset. |  |
**sort** | Option<**String**> | List of comma-separated fields for result sorting (minus for desc sorting). |  |

### Return type

[**crate::models::ListPaymentMethodsResponse**](ListPaymentMethodsResponse.md)

### Authorization

[OAuth2AuthenticationCodeFlow](../README.md#OAuth2AuthenticationCodeFlow), [ManualAuth](../README.md#ManualAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_product_categories

> crate::models::ListProductCategoriesResponse list_product_categories(company_id, context)
List Product Categories

Lists the product categories.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | The ID of the company. | [required] |
**context** | **String** |  | [required] |

### Return type

[**crate::models::ListProductCategoriesResponse**](ListProductCategoriesResponse.md)

### Authorization

[OAuth2AuthenticationCodeFlow](../README.md#OAuth2AuthenticationCodeFlow), [ManualAuth](../README.md#ManualAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_received_document_categories

> crate::models::ListReceivedDocumentCategoriesResponse list_received_document_categories(company_id)
List Received Document Categories

Lists the received document categories.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | The ID of the company. | [required] |

### Return type

[**crate::models::ListReceivedDocumentCategoriesResponse**](ListReceivedDocumentCategoriesResponse.md)

### Authorization

[OAuth2AuthenticationCodeFlow](../README.md#OAuth2AuthenticationCodeFlow), [ManualAuth](../README.md#ManualAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_revenue_centers

> crate::models::ListRevenueCentersResponse list_revenue_centers(company_id)
List Revenue Centers

Lists the revenue centers.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | The ID of the company. | [required] |

### Return type

[**crate::models::ListRevenueCentersResponse**](ListRevenueCentersResponse.md)

### Authorization

[OAuth2AuthenticationCodeFlow](../README.md#OAuth2AuthenticationCodeFlow), [ManualAuth](../README.md#ManualAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_templates

> crate::models::ListTemplatesResponse list_templates(r#type, by_type)
List Templates

Lists the available templates.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#type** | Option<**String**> | Type of the templates. |  |[default to all]
**by_type** | Option<**bool**> | [Only if type=all] If true, splits the list in objects, grouping templates by type. |  |[default to false]

### Return type

[**crate::models::ListTemplatesResponse**](ListTemplatesResponse.md)

### Authorization

[OAuth2AuthenticationCodeFlow](../README.md#OAuth2AuthenticationCodeFlow), [ManualAuth](../README.md#ManualAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_units_of_measure

> crate::models::ListUnitsOfMeasureResponse list_units_of_measure()
List Units of Measure

Lists the units of measure.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ListUnitsOfMeasureResponse**](ListUnitsOfMeasureResponse.md)

### Authorization

[OAuth2AuthenticationCodeFlow](../README.md#OAuth2AuthenticationCodeFlow), [ManualAuth](../README.md#ManualAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_vat_types

> crate::models::ListVatTypesResponse list_vat_types(company_id, fieldset)
List Vat Types

Lists the available vat types.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | The ID of the company. | [required] |
**fieldset** | Option<**String**> | Name of the fieldset. |  |

### Return type

[**crate::models::ListVatTypesResponse**](ListVatTypesResponse.md)

### Authorization

[OAuth2AuthenticationCodeFlow](../README.md#OAuth2AuthenticationCodeFlow), [ManualAuth](../README.md#ManualAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

