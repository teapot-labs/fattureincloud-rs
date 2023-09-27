# \ArchiveApi

All URIs are relative to *https://api-v2.fattureincloud.it*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_archive_document**](ArchiveApi.md#create_archive_document) | **POST** /c/{company_id}/archive | Create Archive Document
[**delete_archive_document**](ArchiveApi.md#delete_archive_document) | **DELETE** /c/{company_id}/archive/{document_id} | Delete Archive Document
[**get_archive_document**](ArchiveApi.md#get_archive_document) | **GET** /c/{company_id}/archive/{document_id} | Get Archive Document
[**list_archive_documents**](ArchiveApi.md#list_archive_documents) | **GET** /c/{company_id}/archive | List Archive Documents
[**modify_archive_document**](ArchiveApi.md#modify_archive_document) | **PUT** /c/{company_id}/archive/{document_id} | Modify Archive Document
[**upload_archive_document_attachment**](ArchiveApi.md#upload_archive_document_attachment) | **POST** /c/{company_id}/archive/attachment | Upload Archive Document Attachment



## create_archive_document

> crate::models::CreateArchiveDocumentResponse create_archive_document(company_id, create_archive_document_request)
Create Archive Document

Creates a new archive document.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | The ID of the company. | [required] |
**create_archive_document_request** | Option<[**CreateArchiveDocumentRequest**](CreateArchiveDocumentRequest.md)> | The Archive Document. |  |

### Return type

[**crate::models::CreateArchiveDocumentResponse**](CreateArchiveDocumentResponse.md)

### Authorization

[OAuth2AuthenticationCodeFlow](../README.md#OAuth2AuthenticationCodeFlow), [ManualAuth](../README.md#ManualAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_archive_document

> delete_archive_document(company_id, document_id)
Delete Archive Document

Deletes the specified archive document.

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


## get_archive_document

> crate::models::GetArchiveDocumentResponse get_archive_document(company_id, document_id, fields, fieldset)
Get Archive Document

Gets the specified archive document.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | The ID of the company. | [required] |
**document_id** | **i32** | The ID of the document. | [required] |
**fields** | Option<**String**> | List of comma-separated fields. |  |
**fieldset** | Option<**String**> | Name of the fieldset. |  |

### Return type

[**crate::models::GetArchiveDocumentResponse**](GetArchiveDocumentResponse.md)

### Authorization

[OAuth2AuthenticationCodeFlow](../README.md#OAuth2AuthenticationCodeFlow), [ManualAuth](../README.md#ManualAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_archive_documents

> crate::models::ListArchiveDocumentsResponse list_archive_documents(company_id, fields, fieldset, sort, page, per_page, q)
List Archive Documents

Lists the archive documents.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | The ID of the company. | [required] |
**fields** | Option<**String**> | List of comma-separated fields. |  |
**fieldset** | Option<**String**> | Name of the fieldset. |  |
**sort** | Option<**String**> | List of comma-separated fields for result sorting (minus for desc sorting). |  |
**page** | Option<**i32**> | The page to retrieve. |  |[default to 1]
**per_page** | Option<**u8**> | The size of the page. |  |[default to 5]
**q** | Option<**String**> | Query for filtering the results. |  |

### Return type

[**crate::models::ListArchiveDocumentsResponse**](ListArchiveDocumentsResponse.md)

### Authorization

[OAuth2AuthenticationCodeFlow](../README.md#OAuth2AuthenticationCodeFlow), [ManualAuth](../README.md#ManualAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## modify_archive_document

> crate::models::ModifyArchiveDocumentResponse modify_archive_document(company_id, document_id, modify_archive_document_request)
Modify Archive Document

Modifies the specified archive document.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | The ID of the company. | [required] |
**document_id** | **i32** | The ID of the document. | [required] |
**modify_archive_document_request** | Option<[**ModifyArchiveDocumentRequest**](ModifyArchiveDocumentRequest.md)> | Modified Archive Document |  |

### Return type

[**crate::models::ModifyArchiveDocumentResponse**](ModifyArchiveDocumentResponse.md)

### Authorization

[OAuth2AuthenticationCodeFlow](../README.md#OAuth2AuthenticationCodeFlow), [ManualAuth](../README.md#ManualAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upload_archive_document_attachment

> crate::models::UploadArchiveAttachmentResponse upload_archive_document_attachment(company_id, filename, attachment)
Upload Archive Document Attachment

Uploads an attachment destined to an archive document. The actual association between the document and the attachment must be implemented separately, using the returned token.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | The ID of the company. | [required] |
**filename** | Option<**String**> | Attachment file name |  |
**attachment** | Option<**std::path::PathBuf**> | Attachment file [.png, .jpg, .gif, .pdf, .zip, .xls, .xlsx, .doc, .docx] |  |

### Return type

[**crate::models::UploadArchiveAttachmentResponse**](UploadArchiveAttachmentResponse.md)

### Authorization

[OAuth2AuthenticationCodeFlow](../README.md#OAuth2AuthenticationCodeFlow), [ManualAuth](../README.md#ManualAuth)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

