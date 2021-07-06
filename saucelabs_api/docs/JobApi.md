# \JobApi

All URIs are relative to *https://saucelabs.com/rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_job**](JobApi.md#get_job) | **get** /v1/{username}/jobs/{id} | Get Job Information
[**get_job_v1_1**](JobApi.md#get_job_v1_1) | **get** /v1.1/jobs/{id} | Get Job Information
[**list_build_failed_jobs**](JobApi.md#list_build_failed_jobs) | **get** /v1/{username}/builds/{id}/failed-jobs | Get all of the jobs associated with a build that have failed
[**list_build_jobs**](JobApi.md#list_build_jobs) | **get** /v1/builds/{id}/jobs | Get all of the jobs associated with a build
[**list_jobs**](JobApi.md#list_jobs) | **get** /v1.1/{username}/jobs | Get all of a users jobs
[**stop_job**](JobApi.md#stop_job) | **put** /v1/{username}/jobs/{id}/stop | Stop Job Information
[**update_job**](JobApi.md#update_job) | **put** /v1/{username}/jobs/{id} | Update Job Information



## get_job

> crate::models::Job get_job(username, id)
Get Job Information

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | username | [required] |
**id** | **String** | job id | [required] |

### Return type

[**crate::models::Job**](Job.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_job_v1_1

> crate::models::Job get_job_v1_1(id)
Get Job Information

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | job id | [required] |

### Return type

[**crate::models::Job**](Job.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


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


## list_jobs

> crate::models::InlineResponse2001 list_jobs(username, limit, subaccounts, full, manual_only, auto_only, name, owner_type, owner, from, to)
Get all of a users jobs

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | username | [required] |
**limit** | Option<**f32**> | Number of results to return |  |[default to 50.0]
**subaccounts** | Option<**bool**> | Include subaccounts in list of jobs |  |[default to false]
**full** | Option<**bool**> | Should the return result contain everything or just the basics |  |
**manual_only** | Option<**bool**> | Only return manual jobs |  |[default to false]
**auto_only** | Option<**bool**> |  |  |[default to false]
**name** | Option<**String**> | name of the job |  |
**owner_type** | Option<**String**> | owner type for jobs |  |
**owner** | Option<**String**> | username of owner of the jobs |  |
**from** | Option<**f32**> | receive jobs beginning of a specific timestamp |  |
**to** | Option<**f32**> | receive jobs until specific timestamp |  |

### Return type

[**crate::models::InlineResponse2001**](inline_response_200_1.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stop_job

> stop_job(username, id)
Stop Job Information

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | username | [required] |
**id** | **String** | job id | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_job

> crate::models::Job update_job(username, id, body)
Update Job Information

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | username | [required] |
**id** | **String** | job id | [required] |
**body** | [**Job**](Job.md) |  | [required] |

### Return type

[**crate::models::Job**](Job.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

