# \SuppliersApi

All URIs are relative to *https://api-v2.fattureincloud.it*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_supplier**](SuppliersApi.md#create_supplier) | **POST** /c/{company_id}/entities/suppliers | Create Supplier
[**delete_supplier**](SuppliersApi.md#delete_supplier) | **DELETE** /c/{company_id}/entities/suppliers/{supplier_id} | Delete Supplier
[**get_supplier**](SuppliersApi.md#get_supplier) | **GET** /c/{company_id}/entities/suppliers/{supplier_id} | Get Supplier
[**list_suppliers**](SuppliersApi.md#list_suppliers) | **GET** /c/{company_id}/entities/suppliers | List Suppliers
[**modify_supplier**](SuppliersApi.md#modify_supplier) | **PUT** /c/{company_id}/entities/suppliers/{supplier_id} | Modify Supplier



## create_supplier

> crate::models::CreateSupplierResponse create_supplier(company_id, create_supplier_request)
Create Supplier

Creates a new supplier.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | The ID of the company. | [required] |
**create_supplier_request** | Option<[**CreateSupplierRequest**](CreateSupplierRequest.md)> | The supplier to create |  |

### Return type

[**crate::models::CreateSupplierResponse**](CreateSupplierResponse.md)

### Authorization

[OAuth2AuthenticationCodeFlow](../README.md#OAuth2AuthenticationCodeFlow), [ManualAuth](../README.md#ManualAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_supplier

> delete_supplier(company_id, supplier_id)
Delete Supplier

Deletes the specified supplier.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | The ID of the company. | [required] |
**supplier_id** | **i32** | The ID of the supplier. | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2AuthenticationCodeFlow](../README.md#OAuth2AuthenticationCodeFlow), [ManualAuth](../README.md#ManualAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_supplier

> crate::models::GetSupplierResponse get_supplier(company_id, supplier_id, fields, fieldset)
Get Supplier

Gets the specified supplier.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | The ID of the company. | [required] |
**supplier_id** | **i32** | The ID of the supplier. | [required] |
**fields** | Option<**String**> | List of comma-separated fields. |  |
**fieldset** | Option<**String**> | Name of the fieldset. |  |

### Return type

[**crate::models::GetSupplierResponse**](GetSupplierResponse.md)

### Authorization

[OAuth2AuthenticationCodeFlow](../README.md#OAuth2AuthenticationCodeFlow), [ManualAuth](../README.md#ManualAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_suppliers

> crate::models::ListSuppliersResponse list_suppliers(company_id, fields, fieldset, sort, page, per_page, q)
List Suppliers

Lists the suppliers.

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

[**crate::models::ListSuppliersResponse**](ListSuppliersResponse.md)

### Authorization

[OAuth2AuthenticationCodeFlow](../README.md#OAuth2AuthenticationCodeFlow), [ManualAuth](../README.md#ManualAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## modify_supplier

> crate::models::ModifySupplierResponse modify_supplier(company_id, supplier_id, modify_supplier_request)
Modify Supplier

Modifies the specified supplier.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | The ID of the company. | [required] |
**supplier_id** | **i32** | The ID of the supplier. | [required] |
**modify_supplier_request** | Option<[**ModifySupplierRequest**](ModifySupplierRequest.md)> | The modified Supplier. First level parameters are managed in delta mode. |  |

### Return type

[**crate::models::ModifySupplierResponse**](ModifySupplierResponse.md)

### Authorization

[OAuth2AuthenticationCodeFlow](../README.md#OAuth2AuthenticationCodeFlow), [ManualAuth](../README.md#ManualAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

