# \AssetApi

All URIs are relative to *https://saucelabs.com/rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**download_job_asset**](AssetApi.md#download_job_asset) | **get** /v1/jobs/{id}/{assetName} | Get job asset



## download_job_asset

> crate::models::File download_job_asset(id, filename, filepath)
Get job asset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | job id | [required] |
**filename** | **String** | filename | [required] |
**filepath** | **String** | file path to store the asset at | [required] |

### Return type

[**crate::models::File**](File.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

