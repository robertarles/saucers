# \ActivityApi

All URIs are relative to *https://saucelabs.com/rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_user_activity**](ActivityApi.md#get_user_activity) | **get** /v1/users/{username}/activity | Get currently running job counts broken down by account and job status
[**get_users_activity**](ActivityApi.md#get_users_activity) | **get** /v1/users_activity | Get job statistics for usernames
[**users_last_job**](ActivityApi.md#users_last_job) | **get** /v1/users_last_job | The result returns dict of usersnames and time when they started last job.



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

