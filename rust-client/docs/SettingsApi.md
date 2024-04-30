# \SettingsApi

All URIs are relative to *https://api-v2.fattureincloud.it*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_payment_account**](SettingsApi.md#create_payment_account) | **POST** /c/{company_id}/settings/payment_accounts | Create Payment Account
[**create_payment_method**](SettingsApi.md#create_payment_method) | **POST** /c/{company_id}/settings/payment_methods | Create Payment Method
[**create_vat_type**](SettingsApi.md#create_vat_type) | **POST** /c/{company_id}/settings/vat_types | Create Vat Type
[**delete_payment_account**](SettingsApi.md#delete_payment_account) | **DELETE** /c/{company_id}/settings/payment_accounts/{payment_account_id} | Delete Payment Account
[**delete_payment_method**](SettingsApi.md#delete_payment_method) | **DELETE** /c/{company_id}/settings/payment_methods/{payment_method_id} | Delete Payment Method
[**delete_vat_type**](SettingsApi.md#delete_vat_type) | **DELETE** /c/{company_id}/settings/vat_types/{vat_type_id} | Delete Vat Type
[**get_payment_account**](SettingsApi.md#get_payment_account) | **GET** /c/{company_id}/settings/payment_accounts/{payment_account_id} | Get Payment Account
[**get_payment_method**](SettingsApi.md#get_payment_method) | **GET** /c/{company_id}/settings/payment_methods/{payment_method_id} | Get Payment Method
[**get_vat_type**](SettingsApi.md#get_vat_type) | **GET** /c/{company_id}/settings/vat_types/{vat_type_id} | Get Vat Type
[**modify_payment_account**](SettingsApi.md#modify_payment_account) | **PUT** /c/{company_id}/settings/payment_accounts/{payment_account_id} | Modify Payment Account
[**modify_payment_method**](SettingsApi.md#modify_payment_method) | **PUT** /c/{company_id}/settings/payment_methods/{payment_method_id} | Modify Payment Method
[**modify_vat_type**](SettingsApi.md#modify_vat_type) | **PUT** /c/{company_id}/settings/vat_types/{vat_type_id} | Modify Vat Type



## create_payment_account

> models::CreatePaymentAccountResponse create_payment_account(company_id, create_payment_account_request)
Create Payment Account

Creates a new payment account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | The ID of the company. | [required] |
**create_payment_account_request** | Option<[**CreatePaymentAccountRequest**](CreatePaymentAccountRequest.md)> |  |  |

### Return type

[**models::CreatePaymentAccountResponse**](CreatePaymentAccountResponse.md)

### Authorization

[OAuth2AuthenticationCodeFlow](../README.md#OAuth2AuthenticationCodeFlow), [ManualAuth](../README.md#ManualAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_payment_method

> models::CreatePaymentMethodResponse create_payment_method(company_id, create_payment_method_request)
Create Payment Method

Creates a new payment method.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | The ID of the company. | [required] |
**create_payment_method_request** | Option<[**CreatePaymentMethodRequest**](CreatePaymentMethodRequest.md)> |  |  |

### Return type

[**models::CreatePaymentMethodResponse**](CreatePaymentMethodResponse.md)

### Authorization

[OAuth2AuthenticationCodeFlow](../README.md#OAuth2AuthenticationCodeFlow), [ManualAuth](../README.md#ManualAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_vat_type

> models::CreateVatTypeResponse create_vat_type(company_id, create_vat_type_request)
Create Vat Type

Creates a vat type.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | The ID of the company. | [required] |
**create_vat_type_request** | Option<[**CreateVatTypeRequest**](CreateVatTypeRequest.md)> |  |  |

### Return type

[**models::CreateVatTypeResponse**](CreateVatTypeResponse.md)

### Authorization

[OAuth2AuthenticationCodeFlow](../README.md#OAuth2AuthenticationCodeFlow), [ManualAuth](../README.md#ManualAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_payment_account

> delete_payment_account(company_id, payment_account_id)
Delete Payment Account

Deletes the specified payment account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | The ID of the company. | [required] |
**payment_account_id** | **i32** | The Referred Payment Account Id. | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2AuthenticationCodeFlow](../README.md#OAuth2AuthenticationCodeFlow), [ManualAuth](../README.md#ManualAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_payment_method

> delete_payment_method(company_id, payment_method_id)
Delete Payment Method

Deletes the specified payment method.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | The ID of the company. | [required] |
**payment_method_id** | **i32** | The Referred Payment Method Id. | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2AuthenticationCodeFlow](../README.md#OAuth2AuthenticationCodeFlow), [ManualAuth](../README.md#ManualAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_vat_type

> delete_vat_type(company_id, vat_type_id)
Delete Vat Type

Deletes the specified vat type.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | The ID of the company. | [required] |
**vat_type_id** | **i32** | The Referred Vat Type Id. | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2AuthenticationCodeFlow](../README.md#OAuth2AuthenticationCodeFlow), [ManualAuth](../README.md#ManualAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_payment_account

> models::GetPaymentAccountResponse get_payment_account(company_id, payment_account_id, fields, fieldset)
Get Payment Account

Gets the specified payment account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | The ID of the company. | [required] |
**payment_account_id** | **i32** | The Referred Payment Account Id. | [required] |
**fields** | Option<**String**> | List of comma-separated fields. |  |
**fieldset** | Option<**String**> | Name of the fieldset. |  |

### Return type

[**models::GetPaymentAccountResponse**](GetPaymentAccountResponse.md)

### Authorization

[OAuth2AuthenticationCodeFlow](../README.md#OAuth2AuthenticationCodeFlow), [ManualAuth](../README.md#ManualAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_payment_method

> models::GetPaymentMethodResponse get_payment_method(company_id, payment_method_id, fields, fieldset)
Get Payment Method

Gets the specified payment method.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | The ID of the company. | [required] |
**payment_method_id** | **i32** | The Referred Payment Method Id. | [required] |
**fields** | Option<**String**> | List of comma-separated fields. |  |
**fieldset** | Option<**String**> | Name of the fieldset. |  |

### Return type

[**models::GetPaymentMethodResponse**](GetPaymentMethodResponse.md)

### Authorization

[OAuth2AuthenticationCodeFlow](../README.md#OAuth2AuthenticationCodeFlow), [ManualAuth](../README.md#ManualAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_vat_type

> models::GetVatType get_vat_type(company_id, vat_type_id)
Get Vat Type

Gets the specified vat type.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | The ID of the company. | [required] |
**vat_type_id** | **i32** | The Referred Vat Type Id. | [required] |

### Return type

[**models::GetVatType**](GetVatType.md)

### Authorization

[OAuth2AuthenticationCodeFlow](../README.md#OAuth2AuthenticationCodeFlow), [ManualAuth](../README.md#ManualAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## modify_payment_account

> models::ModifyPaymentAccountResponse modify_payment_account(company_id, payment_account_id, modify_payment_account_request)
Modify Payment Account

Modifies the specified payment account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | The ID of the company. | [required] |
**payment_account_id** | **i32** | The Referred Payment Account Id. | [required] |
**modify_payment_account_request** | Option<[**ModifyPaymentAccountRequest**](ModifyPaymentAccountRequest.md)> |  |  |

### Return type

[**models::ModifyPaymentAccountResponse**](ModifyPaymentAccountResponse.md)

### Authorization

[OAuth2AuthenticationCodeFlow](../README.md#OAuth2AuthenticationCodeFlow), [ManualAuth](../README.md#ManualAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## modify_payment_method

> models::ModifyPaymentMethodResponse modify_payment_method(company_id, payment_method_id, modify_payment_method_request)
Modify Payment Method

Modifies the specified payment method.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | The ID of the company. | [required] |
**payment_method_id** | **i32** | The Referred Payment Method Id. | [required] |
**modify_payment_method_request** | Option<[**ModifyPaymentMethodRequest**](ModifyPaymentMethodRequest.md)> |  |  |

### Return type

[**models::ModifyPaymentMethodResponse**](ModifyPaymentMethodResponse.md)

### Authorization

[OAuth2AuthenticationCodeFlow](../README.md#OAuth2AuthenticationCodeFlow), [ManualAuth](../README.md#ManualAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## modify_vat_type

> models::ModifyVatTypeResponse modify_vat_type(company_id, vat_type_id, modify_vat_type_request)
Modify Vat Type

Modifies the specified vat type.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | The ID of the company. | [required] |
**vat_type_id** | **i32** | The Referred Vat Type Id. | [required] |
**modify_vat_type_request** | Option<[**ModifyVatTypeRequest**](ModifyVatTypeRequest.md)> |  |  |

### Return type

[**models::ModifyVatTypeResponse**](ModifyVatTypeResponse.md)

### Authorization

[OAuth2AuthenticationCodeFlow](../README.md#OAuth2AuthenticationCodeFlow), [ManualAuth](../README.md#ManualAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

