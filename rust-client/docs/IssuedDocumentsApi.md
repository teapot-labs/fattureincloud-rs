# \IssuedDocumentsApi

All URIs are relative to *https://api-v2.fattureincloud.it*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_issued_document**](IssuedDocumentsApi.md#create_issued_document) | **POST** /c/{company_id}/issued_documents | Create Issued Document
[**delete_issued_document**](IssuedDocumentsApi.md#delete_issued_document) | **DELETE** /c/{company_id}/issued_documents/{document_id} | Delete Issued Document
[**delete_issued_document_attachment**](IssuedDocumentsApi.md#delete_issued_document_attachment) | **DELETE** /c/{company_id}/issued_documents/{document_id}/attachment | Delete Issued Document Attachment
[**get_email_data**](IssuedDocumentsApi.md#get_email_data) | **GET** /c/{company_id}/issued_documents/{document_id}/email | Get Email Data
[**get_existing_issued_document_totals**](IssuedDocumentsApi.md#get_existing_issued_document_totals) | **POST** /c/{company_id}/issued_documents/{document_id}/totals | Get Existing Issued Document Totals
[**get_issued_document**](IssuedDocumentsApi.md#get_issued_document) | **GET** /c/{company_id}/issued_documents/{document_id} | Get Issued Document
[**get_issued_document_pre_create_info**](IssuedDocumentsApi.md#get_issued_document_pre_create_info) | **GET** /c/{company_id}/issued_documents/info | Get Issued Document Pre-Create Info
[**get_new_issued_document_totals**](IssuedDocumentsApi.md#get_new_issued_document_totals) | **POST** /c/{company_id}/issued_documents/totals | Get New Issued Document Totals
[**join_issued_documents**](IssuedDocumentsApi.md#join_issued_documents) | **GET** /c/{company_id}/issued_documents/join | Join Issued Documents
[**list_issued_documents**](IssuedDocumentsApi.md#list_issued_documents) | **GET** /c/{company_id}/issued_documents | List Issued Documents
[**modify_issued_document**](IssuedDocumentsApi.md#modify_issued_document) | **PUT** /c/{company_id}/issued_documents/{document_id} | Modify Issued Document
[**schedule_email**](IssuedDocumentsApi.md#schedule_email) | **POST** /c/{company_id}/issued_documents/{document_id}/email | Schedule Email
[**transform_issued_document**](IssuedDocumentsApi.md#transform_issued_document) | **GET** /c/{company_id}/issued_documents/transform | Transform Issued Document
[**upload_issued_document_attachment**](IssuedDocumentsApi.md#upload_issued_document_attachment) | **POST** /c/{company_id}/issued_documents/attachment | Upload Issued Document Attachment



## create_issued_document

> models::CreateIssuedDocumentResponse create_issued_document(company_id, create_issued_document_request)
Create Issued Document

Creates a new document.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | The ID of the company. | [required] |
**create_issued_document_request** | Option<[**CreateIssuedDocumentRequest**](CreateIssuedDocumentRequest.md)> | The Issued Document |  |

### Return type

[**models::CreateIssuedDocumentResponse**](CreateIssuedDocumentResponse.md)

### Authorization

[OAuth2AuthenticationCodeFlow](../README.md#OAuth2AuthenticationCodeFlow), [ManualAuth](../README.md#ManualAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_issued_document

> delete_issued_document(company_id, document_id)
Delete Issued Document

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


## delete_issued_document_attachment

> delete_issued_document_attachment(company_id, document_id)
Delete Issued Document Attachment

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


## get_email_data

> models::GetEmailDataResponse get_email_data(company_id, document_id)
Get Email Data

Gets the pre-compiled email details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | The ID of the company. | [required] |
**document_id** | **i32** | The ID of the document. | [required] |

### Return type

[**models::GetEmailDataResponse**](GetEmailDataResponse.md)

### Authorization

[OAuth2AuthenticationCodeFlow](../README.md#OAuth2AuthenticationCodeFlow), [ManualAuth](../README.md#ManualAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_existing_issued_document_totals

> models::GetExistingIssuedDocumentTotalsResponse get_existing_issued_document_totals(company_id, document_id, get_existing_issued_document_totals_request)
Get Existing Issued Document Totals

Returns the totals for a specified document.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | The ID of the company. | [required] |
**document_id** | **i32** | The ID of the document. | [required] |
**get_existing_issued_document_totals_request** | Option<[**GetExistingIssuedDocumentTotalsRequest**](GetExistingIssuedDocumentTotalsRequest.md)> |  |  |

### Return type

[**models::GetExistingIssuedDocumentTotalsResponse**](GetExistingIssuedDocumentTotalsResponse.md)

### Authorization

[OAuth2AuthenticationCodeFlow](../README.md#OAuth2AuthenticationCodeFlow), [ManualAuth](../README.md#ManualAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_issued_document

> models::GetIssuedDocumentResponse get_issued_document(company_id, document_id, fields, fieldset)
Get Issued Document

Gets the specified document. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | The ID of the company. | [required] |
**document_id** | **i32** | The ID of the document. | [required] |
**fields** | Option<**String**> | List of comma-separated fields. |  |
**fieldset** | Option<**String**> | Name of the fieldset. |  |

### Return type

[**models::GetIssuedDocumentResponse**](GetIssuedDocumentResponse.md)

### Authorization

[OAuth2AuthenticationCodeFlow](../README.md#OAuth2AuthenticationCodeFlow), [ManualAuth](../README.md#ManualAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_issued_document_pre_create_info

> models::GetIssuedDocumentPreCreateInfoResponse get_issued_document_pre_create_info(company_id, r#type)
Get Issued Document Pre-Create Info

Retrieves the information useful while creating a new document.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | The ID of the company. | [required] |
**r#type** | **String** | The type of the issued document. | [required] |

### Return type

[**models::GetIssuedDocumentPreCreateInfoResponse**](GetIssuedDocumentPreCreateInfoResponse.md)

### Authorization

[OAuth2AuthenticationCodeFlow](../README.md#OAuth2AuthenticationCodeFlow), [ManualAuth](../README.md#ManualAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_new_issued_document_totals

> models::GetNewIssuedDocumentTotalsResponse get_new_issued_document_totals(company_id, get_new_issued_document_totals_request)
Get New Issued Document Totals

Returns the totals for a new document.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | The ID of the company. | [required] |
**get_new_issued_document_totals_request** | Option<[**GetNewIssuedDocumentTotalsRequest**](GetNewIssuedDocumentTotalsRequest.md)> |  |  |

### Return type

[**models::GetNewIssuedDocumentTotalsResponse**](GetNewIssuedDocumentTotalsResponse.md)

### Authorization

[OAuth2AuthenticationCodeFlow](../README.md#OAuth2AuthenticationCodeFlow), [ManualAuth](../README.md#ManualAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## join_issued_documents

> models::JoinIssuedDocumentsResponse join_issued_documents(company_id, ids, group, e_invoice)
Join Issued Documents

Joins issued documents.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | The ID of the company. | [required] |
**ids** | **String** | Ids of the documents. | [required] |
**group** | Option<**i32**> | Group items. |  |
**e_invoice** | Option<**i32**> | New document e_invoice. |  |

### Return type

[**models::JoinIssuedDocumentsResponse**](JoinIssuedDocumentsResponse.md)

### Authorization

[OAuth2AuthenticationCodeFlow](../README.md#OAuth2AuthenticationCodeFlow), [ManualAuth](../README.md#ManualAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_issued_documents

> models::ListIssuedDocumentsResponse list_issued_documents(company_id, r#type, fields, fieldset, sort, page, per_page, q, inclusive)
List Issued Documents

Lists the issued documents.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | The ID of the company. | [required] |
**r#type** | **String** | The type of the issued document. | [required] |
**fields** | Option<**String**> | List of comma-separated fields. |  |
**fieldset** | Option<**String**> | Name of the fieldset. |  |
**sort** | Option<**String**> | List of comma-separated fields for result sorting (minus for desc sorting). |  |
**page** | Option<**i32**> | The page to retrieve. |  |[default to 1]
**per_page** | Option<**u8**> | The size of the page. |  |[default to 5]
**q** | Option<**String**> | Query for filtering the results. |  |
**inclusive** | Option<**i32**> | (Only for type = delivery_notes) Include invoices delivery notes. |  |

### Return type

[**models::ListIssuedDocumentsResponse**](ListIssuedDocumentsResponse.md)

### Authorization

[OAuth2AuthenticationCodeFlow](../README.md#OAuth2AuthenticationCodeFlow), [ManualAuth](../README.md#ManualAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## modify_issued_document

> models::ModifyIssuedDocumentResponse modify_issued_document(company_id, document_id, modify_issued_document_request)
Modify Issued Document

Modifies the specified document.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | The ID of the company. | [required] |
**document_id** | **i32** | The ID of the document. | [required] |
**modify_issued_document_request** | Option<[**ModifyIssuedDocumentRequest**](ModifyIssuedDocumentRequest.md)> | The modified document |  |

### Return type

[**models::ModifyIssuedDocumentResponse**](ModifyIssuedDocumentResponse.md)

### Authorization

[OAuth2AuthenticationCodeFlow](../README.md#OAuth2AuthenticationCodeFlow), [ManualAuth](../README.md#ManualAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## schedule_email

> schedule_email(company_id, document_id, schedule_email_request)
Schedule Email

Schedules the sending of a document by email.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | The ID of the company. | [required] |
**document_id** | **i32** | The ID of the document. | [required] |
**schedule_email_request** | Option<[**ScheduleEmailRequest**](ScheduleEmailRequest.md)> | Email Schedule |  |

### Return type

 (empty response body)

### Authorization

[OAuth2AuthenticationCodeFlow](../README.md#OAuth2AuthenticationCodeFlow), [ManualAuth](../README.md#ManualAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## transform_issued_document

> models::TransformIssuedDocumentResponse transform_issued_document(company_id, original_document_id, new_type, e_invoice, transform_keep_copy)
Transform Issued Document

Transforms the document.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | The ID of the company. | [required] |
**original_document_id** | **i32** | Original document id. | [required] |
**new_type** | **String** | New document type. | [required] |
**e_invoice** | Option<**i32**> | New document e_invoice. |  |
**transform_keep_copy** | Option<**i32**> | Keep the old document. |  |

### Return type

[**models::TransformIssuedDocumentResponse**](TransformIssuedDocumentResponse.md)

### Authorization

[OAuth2AuthenticationCodeFlow](../README.md#OAuth2AuthenticationCodeFlow), [ManualAuth](../README.md#ManualAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upload_issued_document_attachment

> models::UploadIssuedDocumentAttachmentResponse upload_issued_document_attachment(company_id, filename, attachment)
Upload Issued Document Attachment

Uploads an attachment destined to an issued document. The actual association between the document and the attachment must be implemented separately, using the returned token.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | The ID of the company. | [required] |
**filename** | Option<**String**> | Attachment file name |  |
**attachment** | Option<**std::path::PathBuf**> | Attachment file [.png, .jpg, .gif, .pdf, .zip, .xls, .xlsx, .doc, .docx] |  |

### Return type

[**models::UploadIssuedDocumentAttachmentResponse**](UploadIssuedDocumentAttachmentResponse.md)

### Authorization

[OAuth2AuthenticationCodeFlow](../README.md#OAuth2AuthenticationCodeFlow), [ManualAuth](../README.md#ManualAuth)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

