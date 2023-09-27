# \ProductsApi

All URIs are relative to *https://api-v2.fattureincloud.it*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_product**](ProductsApi.md#create_product) | **POST** /c/{company_id}/products | Create Product
[**delete_product**](ProductsApi.md#delete_product) | **DELETE** /c/{company_id}/products/{product_id} | Delete Product
[**get_product**](ProductsApi.md#get_product) | **GET** /c/{company_id}/products/{product_id} | Get Product
[**list_products**](ProductsApi.md#list_products) | **GET** /c/{company_id}/products | List Products
[**modify_product**](ProductsApi.md#modify_product) | **PUT** /c/{company_id}/products/{product_id} | Modify Product



## create_product

> crate::models::CreateProductResponse create_product(company_id, create_product_request)
Create Product

Creates a new product.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | The ID of the company. | [required] |
**create_product_request** | Option<[**CreateProductRequest**](CreateProductRequest.md)> |  |  |

### Return type

[**crate::models::CreateProductResponse**](CreateProductResponse.md)

### Authorization

[OAuth2AuthenticationCodeFlow](../README.md#OAuth2AuthenticationCodeFlow), [ManualAuth](../README.md#ManualAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_product

> delete_product(company_id, product_id)
Delete Product

Deletes the specified product.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | The ID of the company. | [required] |
**product_id** | **i32** | The ID of the product. | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2AuthenticationCodeFlow](../README.md#OAuth2AuthenticationCodeFlow), [ManualAuth](../README.md#ManualAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_product

> crate::models::GetProductResponse get_product(company_id, product_id, fields, fieldset)
Get Product

Gets the specified product.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | The ID of the company. | [required] |
**product_id** | **i32** | The ID of the product. | [required] |
**fields** | Option<**String**> | List of comma-separated fields. |  |
**fieldset** | Option<**String**> | Name of the fieldset. |  |

### Return type

[**crate::models::GetProductResponse**](GetProductResponse.md)

### Authorization

[OAuth2AuthenticationCodeFlow](../README.md#OAuth2AuthenticationCodeFlow), [ManualAuth](../README.md#ManualAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_products

> crate::models::ListProductsResponse list_products(company_id, fields, fieldset, sort, page, per_page, q)
List Products

Lists the products.

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

[**crate::models::ListProductsResponse**](ListProductsResponse.md)

### Authorization

[OAuth2AuthenticationCodeFlow](../README.md#OAuth2AuthenticationCodeFlow), [ManualAuth](../README.md#ManualAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## modify_product

> crate::models::ModifyProductResponse modify_product(company_id, product_id, modify_product_request)
Modify Product

Modifies the specified product.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | The ID of the company. | [required] |
**product_id** | **i32** | The ID of the product. | [required] |
**modify_product_request** | Option<[**ModifyProductRequest**](ModifyProductRequest.md)> | Modified product details. |  |

### Return type

[**crate::models::ModifyProductResponse**](ModifyProductResponse.md)

### Authorization

[OAuth2AuthenticationCodeFlow](../README.md#OAuth2AuthenticationCodeFlow), [ManualAuth](../README.md#ManualAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

