# \UserApi

All URIs are relative to *https://saucelabs.com/rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_current_user_full**](UserApi.md#get_current_user_full) | **get** /v1/whoami | Authenticated user information
[**get_dashboard_message_for_user**](UserApi.md#get_dashboard_message_for_user) | **get** /v1/dashboard_messages/{username} | Dashboard messages from Django
[**get_subaccounts**](UserApi.md#get_subaccounts) | **get** /v1/users/{username}/subaccounts | User information
[**get_user**](UserApi.md#get_user) | **get** /v1/users/{username} | User information
[**get_user_activity**](UserApi.md#get_user_activity) | **get** /v1/users/{username}/activity | Get currently running job counts broken down by account and job status
[**get_user_concurrency**](UserApi.md#get_user_concurrency) | **get** /v1.2/users/{username}/concurrency | User concurrency
[**get_user_minutes**](UserApi.md#get_user_minutes) | **get** /v1/users/{username}/monthly-minutes | User's monthly-minutes
[**get_users_activity**](UserApi.md#get_users_activity) | **get** /v1/users_activity | Get job statistics for usernames
[**list_user_organization**](UserApi.md#list_user_organization) | **get** /v1.1/users/{username}/organization | Org information
[**users_last_job**](UserApi.md#users_last_job) | **get** /v1/users_last_job | The result returns dict of usersnames and time when they started last job.



## get_current_user_full

> crate::models::WhoamiUser get_current_user_full()
Authenticated user information

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::WhoamiUser**](WhoamiUser.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dashboard_message_for_user

> crate::models::InlineResponse2002 get_dashboard_message_for_user(username)
Dashboard messages from Django

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | username | [required] |

### Return type

[**crate::models::InlineResponse2002**](inline_response_200_2.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_subaccounts

> crate::models::User get_subaccounts(username)
User information

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | username | [required] |

### Return type

[**crate::models::User**](User.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user

> crate::models::User get_user(username)
User information

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | username | [required] |

### Return type

[**crate::models::User**](User.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_activity

> crate::models::Activity get_user_activity(username)
Get currently running job counts broken down by account and job status

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | username | [required] |

### Return type

[**crate::models::Activity**](Activity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_concurrency

> crate::models::Concurrency get_user_concurrency(username)
User concurrency

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | username | [required] |

### Return type

[**crate::models::Concurrency**](Concurrency.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_minutes

> crate::models::InlineResponse2006 get_user_minutes(username)
User's monthly-minutes

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | username | [required] |

### Return type

[**crate::models::InlineResponse2006**](inline_response_200_6.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_users_activity

> serde_json::Value get_users_activity()
Get job statistics for usernames

### Parameters

This endpoint does not need any parameter.

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_user_organization

> crate::models::InlineResponse200 list_user_organization(username)
Org information

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | username | [required] |

### Return type

[**crate::models::InlineResponse200**](inline_response_200.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_last_job

> serde_json::Value users_last_job()
The result returns dict of usersnames and time when they started last job.

### Parameters

This endpoint does not need any parameter.

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

