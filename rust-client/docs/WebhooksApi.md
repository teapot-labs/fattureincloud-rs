# \WebhooksApi

All URIs are relative to *https://api-v2.fattureincloud.it*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_webhooks_subscription**](WebhooksApi.md#create_webhooks_subscription) | **POST** /c/{company_id}/subscriptions | Create a Webhook Subscription
[**delete_webhooks_subscription**](WebhooksApi.md#delete_webhooks_subscription) | **DELETE** /c/{company_id}/subscriptions/{subscription_id} | Delete Webhooks Subscription
[**get_webhooks_subscription**](WebhooksApi.md#get_webhooks_subscription) | **GET** /c/{company_id}/subscriptions/{subscription_id} | Get Webhooks Subscription
[**list_webhooks_subscriptions**](WebhooksApi.md#list_webhooks_subscriptions) | **GET** /c/{company_id}/subscriptions | List Webhooks Subscriptions
[**modify_webhooks_subscription**](WebhooksApi.md#modify_webhooks_subscription) | **PUT** /c/{company_id}/subscriptions/{subscription_id} | Modify Webhooks Subscription



## create_webhooks_subscription

> models::CreateWebhooksSubscriptionResponse create_webhooks_subscription(company_id, create_webhooks_subscription_request)
Create a Webhook Subscription

Register some webhooks Subscriptions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | The ID of the company. | [required] |
**create_webhooks_subscription_request** | Option<[**CreateWebhooksSubscriptionRequest**](CreateWebhooksSubscriptionRequest.md)> |  |  |

### Return type

[**models::CreateWebhooksSubscriptionResponse**](CreateWebhooksSubscriptionResponse.md)

### Authorization

[OAuth2AuthenticationCodeFlow](../README.md#OAuth2AuthenticationCodeFlow), [ManualAuth](../README.md#ManualAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_webhooks_subscription

> delete_webhooks_subscription(company_id, subscription_id)
Delete Webhooks Subscription

Delete a webhooks subscription.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | The ID of the company. | [required] |
**subscription_id** | **String** | The ID of the subscription. | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2AuthenticationCodeFlow](../README.md#OAuth2AuthenticationCodeFlow), [ManualAuth](../README.md#ManualAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_webhooks_subscription

> models::GetWebhooksSubscriptionResponse get_webhooks_subscription(company_id, subscription_id)
Get Webhooks Subscription

Get a webhooks subscription.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | The ID of the company. | [required] |
**subscription_id** | **String** | The ID of the subscription. | [required] |

### Return type

[**models::GetWebhooksSubscriptionResponse**](GetWebhooksSubscriptionResponse.md)

### Authorization

[OAuth2AuthenticationCodeFlow](../README.md#OAuth2AuthenticationCodeFlow), [ManualAuth](../README.md#ManualAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_webhooks_subscriptions

> models::ListWebhooksSubscriptionsResponse list_webhooks_subscriptions(company_id)
List Webhooks Subscriptions

List active webhooks subscriptions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | The ID of the company. | [required] |

### Return type

[**models::ListWebhooksSubscriptionsResponse**](ListWebhooksSubscriptionsResponse.md)

### Authorization

[OAuth2AuthenticationCodeFlow](../README.md#OAuth2AuthenticationCodeFlow), [ManualAuth](../README.md#ManualAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## modify_webhooks_subscription

> models::CreateWebhooksSubscriptionRequest1 modify_webhooks_subscription(company_id, subscription_id, modify_webhooks_subscription_request)
Modify Webhooks Subscription

Edit a webhooks subscription.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company_id** | **i32** | The ID of the company. | [required] |
**subscription_id** | **String** | The ID of the subscription. | [required] |
**modify_webhooks_subscription_request** | Option<[**ModifyWebhooksSubscriptionRequest**](ModifyWebhooksSubscriptionRequest.md)> |  |  |

### Return type

[**models::CreateWebhooksSubscriptionRequest1**](CreateWebhooksSubscriptionRequest_1.md)

### Authorization

[OAuth2AuthenticationCodeFlow](../README.md#OAuth2AuthenticationCodeFlow), [ManualAuth](../README.md#ManualAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

