# \ContextApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**context_create_deployment_tag**](ContextApi.md#context_create_deployment_tag) | **POST** /v1/context/{request_id}/{security_number}/tags | Create a Tag for a Running Deployment
[**context_delete_deployment_tag**](ContextApi.md#context_delete_deployment_tag) | **DELETE** /v1/context/{request_id}/{security_number}/tags/{tag_name} | Delete a Tag from a Running Deployment
[**context_get**](ContextApi.md#context_get) | **GET** /v1/context/{request_id}/{security_number} | Get the Context of a Deployment



## context_create_deployment_tag

> crate::models::ContextDeploymentTagResponse context_create_deployment_tag(request_id, security_number, authorization, payload)
Create a Tag for a Running Deployment

Create a tag for a running deployment. You should use this URL inside your deployment container. The URL is injected in your deployment and accessible via the environment variable ARBITRIUM_CONTEXT_URL and you need to append \"/tags\" at the end of this URL.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_id** | **String** | Unique Identifier to keep track of your request across all Arbitrium ecosystem.  | [required] |
**security_number** | **i32** | Random Security number generate to validate the request id. | [required] |
**authorization** | **String** | Auto Generated token. This token is injected in your deployment and can be found via the environment variable named ARBITRIUM_CONTEXT_TOKEN  | [required] |
**payload** | [**InlineObject**](InlineObject.md) |  | [required] |

### Return type

[**crate::models::ContextDeploymentTagResponse**](ContextDeploymentTagResponse.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## context_delete_deployment_tag

> context_delete_deployment_tag(tag_name, request_id, security_number, authorization)
Delete a Tag from a Running Deployment

Delete a tag from a running deployment. You should use this URL inside your deployment container. The URL is injected in your deployment and accessible via the environment variable ARBITRIUM_CONTEXT_URL and you need to append \"/tags/{tag_name}\" at the end of this URL.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tag_name** | **String** |  | [required] |
**request_id** | **String** | Unique Identifier to keep track of your request across all Arbitrium ecosystem.  | [required] |
**security_number** | **i32** | Random Security number generate to validate the request id. | [required] |
**authorization** | **String** | Auto Generated token. This token is injected in your deployment and can be found via the environment variable named ARBITRIUM_CONTEXT_TOKEN  | [required] |

### Return type

 (empty response body)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## context_get

> crate::models::Deployment context_get(request_id, security_number, authorization)
Get the Context of a Deployment

Retrieve contextual information about your live deployment. You should use this URL inside your deployment container. The URL is injected in your deployment and accessible via the environment variable ARBITRIUM_CONTEXT_URL.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_id** | **String** | Unique Identifier to keep track of your request across all Arbitrium ecosystem.  | [required] |
**security_number** | **i32** | Random Security number generate to validate the request id. | [required] |
**authorization** | **String** | Auto Generated token. This token is injected in your deployment and can be found via the environment variable named ARBITRIUM_CONTEXT_TOKEN  | [required] |

### Return type

[**crate::models::Deployment**](Deployment.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

