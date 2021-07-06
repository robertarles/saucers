# \PlatformApi

All URIs are relative to *https://saucelabs.com/rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_manual_platforms**](PlatformApi.md#list_manual_platforms) | **get** /v1/manual/options/ | Platform list with options for desired capabilities



## list_manual_platforms

> ::std::collections::HashMap<String, ::std::collections::HashMap<String, serde_json::Value>> list_manual_platforms()
Platform list with options for desired capabilities

returns a list of supported platforms in the Sauce cloud

### Parameters

This endpoint does not need any parameter.

### Return type

[**::std::collections::HashMap<String, ::std::collections::HashMap<String, serde_json::Value>>**](map.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

