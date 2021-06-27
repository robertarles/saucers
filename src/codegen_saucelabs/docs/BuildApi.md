# \BuildApi

All URIs are relative to *https://saucelabs.com/rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_build_failed_jobs**](BuildApi.md#list_build_failed_jobs) | **get** /v1/{username}/builds/{id}/failed-jobs | Get all of the jobs associated with a build that have failed
[**list_build_jobs**](BuildApi.md#list_build_jobs) | **get** /v1/builds/{id}/jobs | Get all of the jobs associated with a build
[**list_builds**](BuildApi.md#list_builds) | **get** /v1.1/{username}/builds | Get all of a users builds



## list_build_failed_jobs

> Vec<crate::models::Job> list_build_failed_jobs(username, id)
Get all of the jobs associated with a build that have failed

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | username | [required] |
**id** | **String** | job id | [required] |

### Return type

[**Vec<crate::models::Job>**](Job.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_build_jobs

> Vec<crate::models::Job> list_build_jobs(id, full)
Get all of the jobs associated with a build

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | job id | [required] |
**full** | Option<**bool**> | Should the return result contain everything or just the basics |  |

### Return type

[**Vec<crate::models::Job>**](Job.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_builds

> Vec<crate::models::Build> list_builds(username, limit, subaccounts)
Get all of a users builds

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | username | [required] |
**limit** | Option<**f32**> | Number of results to return |  |[default to 50.0]
**subaccounts** | Option<**bool**> | Include subaccounts in list of jobs |  |[default to false]

### Return type

[**Vec<crate::models::Build>**](Build.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

