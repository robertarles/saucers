# \ManualApi

All URIs are relative to *https://saucelabs.com/rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_manual_job**](ManualApi.md#create_manual_job) | **post** /v1/manual | Manual job creation
[**create_manual_job_legacy**](ManualApi.md#create_manual_job_legacy) | **post** /v1/tasks | Manual job creation
[**create_manual_job_screenshot**](ManualApi.md#create_manual_job_screenshot) | **post** /v1/manual/{taskId}/screenshot | Take screenshot in manual session
[**delete_manual_job**](ManualApi.md#delete_manual_job) | **delete** /v1/manual | complete manual task
[**delete_manual_job_legacy**](ManualApi.md#delete_manual_job_legacy) | **delete** /v1/tasks | complete manual task
[**get_manual_job**](ManualApi.md#get_manual_job) | **get** /v1/manual/{taskId} | get manual task
[**list_manual_platforms**](ManualApi.md#list_manual_platforms) | **get** /v1/manual/options/ | Platform list with options for desired capabilities



## create_manual_job

> crate::models::InlineResponse2003 create_manual_job(capabilities)
Manual job creation

Creates a manual job

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**capabilities** | [**InlineObject**](InlineObject.md) |  | [required] |

### Return type

[**crate::models::InlineResponse2003**](inline_response_200_3.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_manual_job_legacy

> crate::models::InlineResponse2003 create_manual_job_legacy(capabilities)
Manual job creation

Creates a manual job

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**capabilities** | [**InlineObject1**](InlineObject1.md) |  | [required] |

### Return type

[**crate::models::InlineResponse2003**](inline_response_200_3.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_manual_job_screenshot

> create_manual_job_screenshot(task_id)
Take screenshot in manual session

Take screenshot in manual session

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**task_id** | **String** | task id of manual task | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_manual_job

> delete_manual_job(ids)
complete manual task

complete manual task

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | list of task ids that to complete | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_manual_job_legacy

> delete_manual_job_legacy(ids)
complete manual task

complete manual task

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | list of task ids that to complete | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_manual_job

> crate::models::InlineResponse2004 get_manual_job(task_id)
get manual task

get manual task

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**task_id** | **String** | task id of manual task | [required] |

### Return type

[**crate::models::InlineResponse2004**](inline_response_200_4.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


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

