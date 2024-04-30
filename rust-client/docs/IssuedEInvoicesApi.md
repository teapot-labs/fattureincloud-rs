# \IssuedEInvoicesApi

All URIs are relative to *https://api-v2.fattureincloud.it*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_e_invoice_rejection_reason**](IssuedEInvoicesApi.md#get_e_invoice_rejection_reason) | **GET** /c/{company_id}/issued_documents/{document_id}/e_invoice/error_reason | Get E-Invoice Rejection Reason
[**get_e_invoice_xml**](IssuedEInvoicesApi.md#get_e_invoice_xml) | **GET** /c/{company_id}/issued_documents/{document_id}/e_invoice/xml | Get E-Invoice XML
[**send_e_invoice**](IssuedEInvoicesApi.md#send_e_invoice) | **POST** /c/{company_id}/issued_documents/{document_id}/e_invoice/send | Send E-Invoice
[**verify_e_invoice_xml**](IssuedEInvoicesApi.md#verify_e_invoice_xml) | **GET** /c/{company_id}/issued_documents/{document_id}/e_invoice/xml_verify | Verify E-Invoice XML



## get_e_invoice_rejection_reason

> models::GetEInvoiceRejectionReason get_e_invoice_rejection_reason(company_id, document_id)
Get E-Invoice Rejection Reason

Get e-invoice rejection reason

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | The ID of the company. | [required] |
**document_id** | **i32** | The ID of the document. | [required] |

### Return type

[**models::GetEInvoiceRejectionReason**](GetEInvoiceRejectionReason.md)

### Authorization

[OAuth2AuthenticationCodeFlow](../README.md#OAuth2AuthenticationCodeFlow), [ManualAuth](../README.md#ManualAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_e_invoice_xml

> String get_e_invoice_xml(company_id, document_id, include_attachment)
Get E-Invoice XML

Downloads the e-invoice in XML format.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | The ID of the company. | [required] |
**document_id** | **i32** | The ID of the document. | [required] |
**include_attachment** | Option<**bool**> | Include the attachment to the XML e-invoice. |  |

### Return type

**String**

### Authorization

[OAuth2AuthenticationCodeFlow](../README.md#OAuth2AuthenticationCodeFlow), [ManualAuth](../README.md#ManualAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_e_invoice

> models::SendEInvoiceResponse send_e_invoice(company_id, document_id, send_e_invoice_request)
Send E-Invoice

Sends the e-invoice to SDI.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | The ID of the company. | [required] |
**document_id** | **i32** | The ID of the document. | [required] |
**send_e_invoice_request** | Option<[**SendEInvoiceRequest**](SendEInvoiceRequest.md)> |  |  |

### Return type

[**models::SendEInvoiceResponse**](SendEInvoiceResponse.md)

### Authorization

[OAuth2AuthenticationCodeFlow](../README.md#OAuth2AuthenticationCodeFlow), [ManualAuth](../README.md#ManualAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## verify_e_invoice_xml

> models::VerifyEInvoiceXmlResponse verify_e_invoice_xml(company_id, document_id)
Verify E-Invoice XML

Verifies the e-invoice XML format. Checks if all of the mandatory fields are filled and compliant to the right format.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | The ID of the company. | [required] |
**document_id** | **i32** | The ID of the document. | [required] |

### Return type

[**models::VerifyEInvoiceXmlResponse**](VerifyEInvoiceXmlResponse.md)

### Authorization

[OAuth2AuthenticationCodeFlow](../README.md#OAuth2AuthenticationCodeFlow), [ManualAuth](../README.md#ManualAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

