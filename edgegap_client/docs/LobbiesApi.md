# \LobbiesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**lobby_create**](LobbiesApi.md#lobby_create) | **POST** /v1/lobbies | Create a Lobby
[**lobby_deploy**](LobbiesApi.md#lobby_deploy) | **POST** /v1/lobbies:deploy | Deploy a Lobby
[**lobby_get**](LobbiesApi.md#lobby_get) | **GET** /v1/lobbies/{lobby_name} | Get a Lobby
[**lobby_terminate**](LobbiesApi.md#lobby_terminate) | **POST** /v1/lobbies:terminate | Terminate a Lobby



## lobby_create

> crate::models::LobbyReadResponse lobby_create(payload)
Create a Lobby

Create a named lobby.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payload** | [**LobbyCreatePayload**](LobbyCreatePayload.md) |  | [required] |

### Return type

[**crate::models::LobbyReadResponse**](LobbyReadResponse.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lobby_deploy

> crate::models::LobbyReadResponse lobby_deploy(payload)
Deploy a Lobby

Deploy the lobby with the given name.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payload** | [**LobbyDeployPayload**](LobbyDeployPayload.md) |  | [required] |

### Return type

[**crate::models::LobbyReadResponse**](LobbyReadResponse.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lobby_get

> crate::models::LobbyReadResponse lobby_get(lobby_name)
Get a Lobby

Get a named lobby.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lobby_name** | **String** |  | [required] |

### Return type

[**crate::models::LobbyReadResponse**](LobbyReadResponse.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lobby_terminate

> crate::models::LobbyReadResponse lobby_terminate(payload)
Terminate a Lobby

Terminate the lobby with the given name.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payload** | [**LobbyTerminatePayload**](LobbyTerminatePayload.md) |  | [required] |

### Return type

[**crate::models::LobbyReadResponse**](LobbyReadResponse.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

