# \ClientsApi

All URIs are relative to *https://api-v2.fattureincloud.it*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_client**](ClientsApi.md#create_client) | **POST** /c/{company_id}/entities/clients | Create Client
[**delete_client**](ClientsApi.md#delete_client) | **DELETE** /c/{company_id}/entities/clients/{client_id} | Delete Client
[**get_client**](ClientsApi.md#get_client) | **GET** /c/{company_id}/entities/clients/{client_id} | Get Client
[**list_clients**](ClientsApi.md#list_clients) | **GET** /c/{company_id}/entities/clients | List Clients
[**modify_client**](ClientsApi.md#modify_client) | **PUT** /c/{company_id}/entities/clients/{client_id} | Modify Client



## create_client

> models::CreateClientResponse create_client(company_id, create_client_request)
Create Client

Creates a new client.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | The ID of the company. | [required] |
**create_client_request** | Option<[**CreateClientRequest**](CreateClientRequest.md)> | The client to create |  |

### Return type

[**models::CreateClientResponse**](CreateClientResponse.md)

### Authorization

[OAuth2AuthenticationCodeFlow](../README.md#OAuth2AuthenticationCodeFlow), [ManualAuth](../README.md#ManualAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_client

> delete_client(company_id, client_id)
Delete Client

Deletes the specified client.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | The ID of the company. | [required] |
**client_id** | **i32** | The ID of the client. | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2AuthenticationCodeFlow](../README.md#OAuth2AuthenticationCodeFlow), [ManualAuth](../README.md#ManualAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_client

> models::GetClientResponse get_client(company_id, client_id, fields, fieldset)
Get Client

Gets the specified client.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | The ID of the company. | [required] |
**client_id** | **i32** | The ID of the client. | [required] |
**fields** | Option<**String**> | List of comma-separated fields. |  |
**fieldset** | Option<**String**> | Name of the fieldset. |  |

### Return type

[**models::GetClientResponse**](GetClientResponse.md)

### Authorization

[OAuth2AuthenticationCodeFlow](../README.md#OAuth2AuthenticationCodeFlow), [ManualAuth](../README.md#ManualAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_clients

> models::ListClientsResponse list_clients(company_id, fields, fieldset, sort, page, per_page, q)
List Clients

Lists the clients.

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

[**models::ListClientsResponse**](ListClientsResponse.md)

### Authorization

[OAuth2AuthenticationCodeFlow](../README.md#OAuth2AuthenticationCodeFlow), [ManualAuth](../README.md#ManualAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## modify_client

> models::ModifyClientResponse modify_client(company_id, client_id, modify_client_request)
Modify Client

Modifies the specified client.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | The ID of the company. | [required] |
**client_id** | **i32** | The ID of the client. | [required] |
**modify_client_request** | Option<[**ModifyClientRequest**](ModifyClientRequest.md)> | The modified Client. First level parameters are managed in delta mode. |  |

### Return type

[**models::ModifyClientResponse**](ModifyClientResponse.md)

### Authorization

[OAuth2AuthenticationCodeFlow](../README.md#OAuth2AuthenticationCodeFlow), [ManualAuth](../README.md#ManualAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

