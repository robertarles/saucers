# \InfoApi

All URIs are relative to *https://saucelabs.com/rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_status**](InfoApi.md#get_status) | **get** /v1/info/status | Sauce Labs Status
[**list_platforms**](InfoApi.md#list_platforms) | **get** /v1/info/platforms/{platform} | Appium Platform Selections



## get_status

> crate::models::SauceStatus get_status()
Sauce Labs Status

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::SauceStatus**](SauceStatus.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_platforms

> Vec<crate::models::Platform> list_platforms(platform)
Appium Platform Selections

returns a list of supported platforms in the Sauce cloud

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**platform** | **String** | username | [required] |

### Return type

[**Vec<crate::models::Platform>**](Platform.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

