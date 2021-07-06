# \TunnelApi

All URIs are relative to *https://saucelabs.com/rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_tunnel**](TunnelApi.md#delete_tunnel) | **delete** /v1/{username}/tunnels/{id} | Delete a Tunnel
[**get_tunnel**](TunnelApi.md#get_tunnel) | **get** /v1/{username}/tunnels/{id} | Get Tunnels
[**list_all_tunnels**](TunnelApi.md#list_all_tunnels) | **get** /v1/{username}/all_tunnels | Get all Tunnels
[**list_available_tunnels**](TunnelApi.md#list_available_tunnels) | **get** /v1.1/{username}/available_tunnels | Get Tunnels
[**list_tunnels**](TunnelApi.md#list_tunnels) | **get** /v1/{username}/tunnels | Get Tunnels



## delete_tunnel

> delete_tunnel(username, id)
Delete a Tunnel

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
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tunnel

> crate::models::Tunnel get_tunnel(username, id)
Get Tunnels

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | username | [required] |
**id** | **String** | job id | [required] |

### Return type

[**crate::models::Tunnel**](Tunnel.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_all_tunnels

> crate::models::InlineResponse2007 list_all_tunnels(username)
Get all Tunnels

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | username | [required] |

### Return type

[**crate::models::InlineResponse2007**](inline_response_200_7.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_available_tunnels

> ::std::collections::HashMap<String, Vec<crate::models::Tunnel>> list_available_tunnels(username)
Get Tunnels

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | username | [required] |

### Return type

[**::std::collections::HashMap<String, Vec<crate::models::Tunnel>>**](array.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_tunnels

> Vec<String> list_tunnels(username, full)
Get Tunnels

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | username | [required] |
**full** | Option<**bool**> | Should the return result contain everything or just the basics |  |

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

