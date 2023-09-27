# \UserApi

All URIs are relative to *https://api-v2.fattureincloud.it*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_user_info**](UserApi.md#get_user_info) | **GET** /user/info | Get User Info
[**list_user_companies**](UserApi.md#list_user_companies) | **GET** /user/companies | List User Companies



## get_user_info

> crate::models::GetUserInfoResponse get_user_info()
Get User Info

Gets the current user's info.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GetUserInfoResponse**](GetUserInfoResponse.md)

### Authorization

[OAuth2AuthenticationCodeFlow](../README.md#OAuth2AuthenticationCodeFlow), [ManualAuth](../README.md#ManualAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_user_companies

> crate::models::ListUserCompaniesResponse list_user_companies()
List User Companies

Lists the companies controlled by the current user.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ListUserCompaniesResponse**](ListUserCompaniesResponse.md)

### Authorization

[OAuth2AuthenticationCodeFlow](../README.md#OAuth2AuthenticationCodeFlow), [ManualAuth](../README.md#ManualAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

