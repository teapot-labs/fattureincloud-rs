# \ReceivedDocumentsApi

All URIs are relative to *https://api-v2.fattureincloud.it*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_received_document**](ReceivedDocumentsApi.md#create_received_document) | **POST** /c/{company_id}/received_documents | Create Received Document
[**delete_received_document**](ReceivedDocumentsApi.md#delete_received_document) | **DELETE** /c/{company_id}/received_documents/{document_id} | Delete Received Document
[**delete_received_document_attachment**](ReceivedDocumentsApi.md#delete_received_document_attachment) | **DELETE** /c/{company_id}/received_documents/{document_id}/attachment | Delete Received Document Attachment
[**get_existing_received_document_totals**](ReceivedDocumentsApi.md#get_existing_received_document_totals) | **POST** /c/{company_id}/received_documents/{document_id}/totals | Get Existing Received Document Totals
[**get_new_received_document_totals**](ReceivedDocumentsApi.md#get_new_received_document_totals) | **POST** /c/{company_id}/received_documents/totals | Get New Received Document Totals
[**get_received_document**](ReceivedDocumentsApi.md#get_received_document) | **GET** /c/{company_id}/received_documents/{document_id} | Get Received Document
[**get_received_document_pre_create_info**](ReceivedDocumentsApi.md#get_received_document_pre_create_info) | **GET** /c/{company_id}/received_documents/info | Get Received Document Pre-Create Info
[**list_received_documents**](ReceivedDocumentsApi.md#list_received_documents) | **GET** /c/{company_id}/received_documents | List Received Documents
[**modify_received_document**](ReceivedDocumentsApi.md#modify_received_document) | **PUT** /c/{company_id}/received_documents/{document_id} | Modify Received Document
[**upload_received_document_attachment**](ReceivedDocumentsApi.md#upload_received_document_attachment) | **POST** /c/{company_id}/received_documents/attachment | Upload Received Document Attachment



## create_received_document

> crate::models::CreateReceivedDocumentResponse create_received_document(company_id, create_received_document_request)
Create Received Document

Creates a new document.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | The ID of the company. | [required] |
**create_received_document_request** | Option<[**CreateReceivedDocumentRequest**](CreateReceivedDocumentRequest.md)> | Document to create |  |

### Return type

[**crate::models::CreateReceivedDocumentResponse**](CreateReceivedDocumentResponse.md)

### Authorization

[OAuth2AuthenticationCodeFlow](../README.md#OAuth2AuthenticationCodeFlow), [ManualAuth](../README.md#ManualAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_received_document

> delete_received_document(company_id, document_id)
Delete Received Document

Deletes the specified document.

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


## delete_received_document_attachment

> delete_received_document_attachment(company_id, document_id)
Delete Received Document Attachment

Removes the attachment of the specified document.

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


## get_existing_received_document_totals

> crate::models::GetExistingReceivedDocumentTotalsResponse get_existing_received_document_totals(company_id, document_id, get_existing_received_document_totals)
Get Existing Received Document Totals

Returns the totals for the specified document.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | The ID of the company. | [required] |
**document_id** | **i32** | The ID of the document. | [required] |
**get_existing_received_document_totals** | Option<[**GetExistingReceivedDocumentTotals**](GetExistingReceivedDocumentTotals.md)> | Received document. |  |

### Return type

[**crate::models::GetExistingReceivedDocumentTotalsResponse**](GetExistingReceivedDocumentTotalsResponse.md)

### Authorization

[OAuth2AuthenticationCodeFlow](../README.md#OAuth2AuthenticationCodeFlow), [ManualAuth](../README.md#ManualAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_new_received_document_totals

> crate::models::GetNewReceivedDocumentTotalsResponse get_new_received_document_totals(company_id, get_new_received_document_totals_request)
Get New Received Document Totals

Returns the totals for a new document.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | The ID of the company. | [required] |
**get_new_received_document_totals_request** | Option<[**GetNewReceivedDocumentTotalsRequest**](GetNewReceivedDocumentTotalsRequest.md)> | Received document. |  |

### Return type

[**crate::models::GetNewReceivedDocumentTotalsResponse**](GetNewReceivedDocumentTotalsResponse.md)

### Authorization

[OAuth2AuthenticationCodeFlow](../README.md#OAuth2AuthenticationCodeFlow), [ManualAuth](../README.md#ManualAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_received_document

> crate::models::GetReceivedDocumentResponse get_received_document(company_id, document_id, fields, fieldset)
Get Received Document

Gets the specified document.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | The ID of the company. | [required] |
**document_id** | **i32** | The ID of the document. | [required] |
**fields** | Option<**String**> | List of comma-separated fields. |  |
**fieldset** | Option<**String**> | Name of the fieldset. |  |

### Return type

[**crate::models::GetReceivedDocumentResponse**](GetReceivedDocumentResponse.md)

### Authorization

[OAuth2AuthenticationCodeFlow](../README.md#OAuth2AuthenticationCodeFlow), [ManualAuth](../README.md#ManualAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_received_document_pre_create_info

> crate::models::GetReceivedDocumentPreCreateInfoResponse get_received_document_pre_create_info(company_id, r#type)
Get Received Document Pre-Create Info

Retrieves the information useful while creating a new document.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | The ID of the company. | [required] |
**r#type** | **String** | The type of the received document. | [required] |

### Return type

[**crate::models::GetReceivedDocumentPreCreateInfoResponse**](GetReceivedDocumentPreCreateInfoResponse.md)

### Authorization

[OAuth2AuthenticationCodeFlow](../README.md#OAuth2AuthenticationCodeFlow), [ManualAuth](../README.md#ManualAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_received_documents

> crate::models::ListReceivedDocumentsResponse list_received_documents(company_id, r#type, fields, fieldset, sort, page, per_page, q)
List Received Documents

Lists the received documents.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | The ID of the company. | [required] |
**r#type** | **String** | The type of the received document. | [required] |
**fields** | Option<**String**> | List of comma-separated fields. |  |
**fieldset** | Option<**String**> | Name of the fieldset. |  |
**sort** | Option<**String**> | List of comma-separated fields for result sorting (minus for desc sorting). |  |
**page** | Option<**i32**> | The page to retrieve. |  |[default to 1]
**per_page** | Option<**u8**> | The size of the page. |  |[default to 5]
**q** | Option<**String**> | Query for filtering the results. |  |

### Return type

[**crate::models::ListReceivedDocumentsResponse**](ListReceivedDocumentsResponse.md)

### Authorization

[OAuth2AuthenticationCodeFlow](../README.md#OAuth2AuthenticationCodeFlow), [ManualAuth](../README.md#ManualAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## modify_received_document

> crate::models::ModifyReceivedDocumentResponse modify_received_document(company_id, document_id, modify_received_document_request)
Modify Received Document

Modifies the specified document.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | The ID of the company. | [required] |
**document_id** | **i32** | The ID of the document. | [required] |
**modify_received_document_request** | Option<[**ModifyReceivedDocumentRequest**](ModifyReceivedDocumentRequest.md)> | Modified document. |  |

### Return type

[**crate::models::ModifyReceivedDocumentResponse**](ModifyReceivedDocumentResponse.md)

### Authorization

[OAuth2AuthenticationCodeFlow](../README.md#OAuth2AuthenticationCodeFlow), [ManualAuth](../README.md#ManualAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upload_received_document_attachment

> crate::models::UploadReceivedDocumentAttachmentResponse upload_received_document_attachment(company_id, filename, attachment)
Upload Received Document Attachment

Uploads an attachment destined to a received document. The actual association between the document and the attachment must be implemented separately, using the returned token.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | The ID of the company. | [required] |
**filename** | Option<**String**> | Attachment file name |  |
**attachment** | Option<**std::path::PathBuf**> | Attachment file [.png, .jpg, .gif, .pdf, .zip, .xls, .xlsx, .doc, .docx] |  |

### Return type

[**crate::models::UploadReceivedDocumentAttachmentResponse**](UploadReceivedDocumentAttachmentResponse.md)

### Authorization

[OAuth2AuthenticationCodeFlow](../README.md#OAuth2AuthenticationCodeFlow), [ManualAuth](../README.md#ManualAuth)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

