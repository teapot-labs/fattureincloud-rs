# \ReceiptsApi

All URIs are relative to *https://api-v2.fattureincloud.it*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_receipt**](ReceiptsApi.md#create_receipt) | **POST** /c/{company_id}/receipts | Create Receipt
[**delete_receipt**](ReceiptsApi.md#delete_receipt) | **DELETE** /c/{company_id}/receipts/{document_id} | Delete Receipt
[**get_receipt**](ReceiptsApi.md#get_receipt) | **GET** /c/{company_id}/receipts/{document_id} | Get Receipt
[**get_receipt_pre_create_info**](ReceiptsApi.md#get_receipt_pre_create_info) | **GET** /c/{company_id}/receipts/info | Get Receipt Pre-Create Info
[**get_receipts_monthly_totals**](ReceiptsApi.md#get_receipts_monthly_totals) | **GET** /c/{company_id}/receipts/monthly_totals | Get Receipts Monthly Totals
[**list_receipts**](ReceiptsApi.md#list_receipts) | **GET** /c/{company_id}/receipts | List Receipts
[**modify_receipt**](ReceiptsApi.md#modify_receipt) | **PUT** /c/{company_id}/receipts/{document_id} | Modify Receipt



## create_receipt

> crate::models::CreateReceiptResponse create_receipt(company_id, create_receipt_request)
Create Receipt

Creates a new receipt.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | The ID of the company. | [required] |
**create_receipt_request** | Option<[**CreateReceiptRequest**](CreateReceiptRequest.md)> | The Receipt to create. |  |

### Return type

[**crate::models::CreateReceiptResponse**](CreateReceiptResponse.md)

### Authorization

[OAuth2AuthenticationCodeFlow](../README.md#OAuth2AuthenticationCodeFlow), [ManualAuth](../README.md#ManualAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_receipt

> delete_receipt(company_id, document_id)
Delete Receipt

Deletes the specified receipt.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | The ID of the company. | [required] |
**document_id** | **i32** | The ID of the document. | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2AuthenticationCodeFlow](../README.md#OAuth2AuthenticationCodeFlow), [ManualAuth](../README.md#ManualAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_receipt

> crate::models::GetReceiptResponse get_receipt(company_id, document_id, fields, fieldset)
Get Receipt

Gets the specified receipt.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | The ID of the company. | [required] |
**document_id** | **i32** | The ID of the document. | [required] |
**fields** | Option<**String**> | List of comma-separated fields. |  |
**fieldset** | Option<**String**> | Name of the fieldset. |  |

### Return type

[**crate::models::GetReceiptResponse**](GetReceiptResponse.md)

### Authorization

[OAuth2AuthenticationCodeFlow](../README.md#OAuth2AuthenticationCodeFlow), [ManualAuth](../README.md#ManualAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_receipt_pre_create_info

> crate::models::GetReceiptPreCreateInfoResponse get_receipt_pre_create_info(company_id)
Get Receipt Pre-Create Info

Retrieves the information useful while creating a new receipt.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | The ID of the company. | [required] |

### Return type

[**crate::models::GetReceiptPreCreateInfoResponse**](GetReceiptPreCreateInfoResponse.md)

### Authorization

[OAuth2AuthenticationCodeFlow](../README.md#OAuth2AuthenticationCodeFlow), [ManualAuth](../README.md#ManualAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_receipts_monthly_totals

> crate::models::GetReceiptsMonthlyTotalsResponse get_receipts_monthly_totals(company_id, r#type, year)
Get Receipts Monthly Totals

Returns the monthly totals by year and receipt type.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | The ID of the company. | [required] |
**r#type** | **String** | Receipt Type | [required] |
**year** | **String** | Year for which you want monthly totals | [required] |

### Return type

[**crate::models::GetReceiptsMonthlyTotalsResponse**](GetReceiptsMonthlyTotalsResponse.md)

### Authorization

[OAuth2AuthenticationCodeFlow](../README.md#OAuth2AuthenticationCodeFlow), [ManualAuth](../README.md#ManualAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_receipts

> crate::models::ListReceiptsResponse list_receipts(company_id, fields, fieldset, page, per_page, sort, q)
List Receipts

Lists the receipts.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | The ID of the company. | [required] |
**fields** | Option<**String**> | List of comma-separated fields. |  |
**fieldset** | Option<**String**> | Name of the fieldset. |  |
**page** | Option<**i32**> | The page to retrieve. |  |[default to 1]
**per_page** | Option<**u8**> | The size of the page. |  |[default to 5]
**sort** | Option<**String**> | List of comma-separated fields for result sorting (minus for desc sorting). |  |
**q** | Option<**String**> | Query for filtering the results. |  |

### Return type

[**crate::models::ListReceiptsResponse**](ListReceiptsResponse.md)

### Authorization

[OAuth2AuthenticationCodeFlow](../README.md#OAuth2AuthenticationCodeFlow), [ManualAuth](../README.md#ManualAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## modify_receipt

> crate::models::ModifyReceiptResponse modify_receipt(company_id, document_id, modify_receipt_request)
Modify Receipt

Modifies the specified receipt.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | The ID of the company. | [required] |
**document_id** | **i32** | The ID of the document. | [required] |
**modify_receipt_request** | Option<[**ModifyReceiptRequest**](ModifyReceiptRequest.md)> | Modified receipt. |  |

### Return type

[**crate::models::ModifyReceiptResponse**](ModifyReceiptResponse.md)

### Authorization

[OAuth2AuthenticationCodeFlow](../README.md#OAuth2AuthenticationCodeFlow), [ManualAuth](../README.md#ManualAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

