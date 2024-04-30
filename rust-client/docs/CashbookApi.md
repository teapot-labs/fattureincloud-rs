# \CashbookApi

All URIs are relative to *https://api-v2.fattureincloud.it*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_cashbook_entry**](CashbookApi.md#create_cashbook_entry) | **POST** /c/{company_id}/cashbook | Create Cashbook Entry
[**delete_cashbook_entry**](CashbookApi.md#delete_cashbook_entry) | **DELETE** /c/{company_id}/cashbook/{document_id} | Delete Cashbook Entry
[**get_cashbook_entry**](CashbookApi.md#get_cashbook_entry) | **GET** /c/{company_id}/cashbook/{document_id} | Get Cashbook Entry
[**list_cashbook_entries**](CashbookApi.md#list_cashbook_entries) | **GET** /c/{company_id}/cashbook | List Cashbook Entries
[**modify_cashbook_entry**](CashbookApi.md#modify_cashbook_entry) | **PUT** /c/{company_id}/cashbook/{document_id} | Modify Cashbook Entry



## create_cashbook_entry

> models::CreateCashbookEntryResponse create_cashbook_entry(company_id, create_cashbook_entry_request)
Create Cashbook Entry

Creates a new cashbook entry.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | The ID of the company. | [required] |
**create_cashbook_entry_request** | Option<[**CreateCashbookEntryRequest**](CreateCashbookEntryRequest.md)> | Cashbook entry.  |  |

### Return type

[**models::CreateCashbookEntryResponse**](CreateCashbookEntryResponse.md)

### Authorization

[OAuth2AuthenticationCodeFlow](../README.md#OAuth2AuthenticationCodeFlow), [ManualAuth](../README.md#ManualAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_cashbook_entry

> delete_cashbook_entry(company_id, document_id)
Delete Cashbook Entry

Deletes the specified cashbook entry.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | The ID of the company. | [required] |
**document_id** | **String** | The ID of the document. | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2AuthenticationCodeFlow](../README.md#OAuth2AuthenticationCodeFlow), [ManualAuth](../README.md#ManualAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_cashbook_entry

> models::GetCashbookEntryResponse get_cashbook_entry(company_id, document_id, fields, fieldset)
Get Cashbook Entry

Gets the specified cashbook entry.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | The ID of the company. | [required] |
**document_id** | **String** | The ID of the document. | [required] |
**fields** | Option<**String**> | List of comma-separated fields. |  |
**fieldset** | Option<**String**> | Name of the fieldset. |  |

### Return type

[**models::GetCashbookEntryResponse**](GetCashbookEntryResponse.md)

### Authorization

[OAuth2AuthenticationCodeFlow](../README.md#OAuth2AuthenticationCodeFlow), [ManualAuth](../README.md#ManualAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_cashbook_entries

> models::ListCashbookEntriesResponse list_cashbook_entries(company_id, date_from, date_to, year, r#type, payment_account_id)
List Cashbook Entries

Lists the cashbook entries.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | The ID of the company. | [required] |
**date_from** | **String** | Start date. | [required] |
**date_to** | **String** | End date. | [required] |
**year** | Option<**i32**> | Filter cashbook by year. |  |
**r#type** | Option<**String**> | Filter cashbook by type. |  |
**payment_account_id** | Option<**i32**> | Filter by payment account. |  |

### Return type

[**models::ListCashbookEntriesResponse**](ListCashbookEntriesResponse.md)

### Authorization

[OAuth2AuthenticationCodeFlow](../README.md#OAuth2AuthenticationCodeFlow), [ManualAuth](../README.md#ManualAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## modify_cashbook_entry

> models::ModifyCashbookEntryResponse modify_cashbook_entry(company_id, document_id, modify_cashbook_entry_request)
Modify Cashbook Entry

Modifies the specified cashbook entry.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | The ID of the company. | [required] |
**document_id** | **String** | The ID of the document. | [required] |
**modify_cashbook_entry_request** | Option<[**ModifyCashbookEntryRequest**](ModifyCashbookEntryRequest.md)> | Cashbook Entry |  |

### Return type

[**models::ModifyCashbookEntryResponse**](ModifyCashbookEntryResponse.md)

### Authorization

[OAuth2AuthenticationCodeFlow](../README.md#OAuth2AuthenticationCodeFlow), [ManualAuth](../README.md#ManualAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

