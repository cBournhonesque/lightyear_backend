# SessionGet

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**session_id** | **String** | Unique UUID | 
**custom_id** | **String** | Custom ID if Available | 
**status** | **String** | Current status of the session | 
**ready** | **bool** | If the session is linked to a ready deployment | 
**linked** | **bool** | If the session is linked to a deployment | 
**kind** | **String** | Type of session created | 
**user_count** | **i32** | Count of user this session currently have | 
**app_id** | **i32** | App version linked to the session | 
**create_time** | **String** | Session created at | 
**elapsed** | **i32** | Elapsed time | 
**error** | Option<**String**> | Error Detail | [optional]
**session_users** | Option<[**Vec<crate::models::SessionUser>**](SessionUser.md)> | Users in the session | [optional]
**session_ips** | Option<[**Vec<crate::models::SessionUser>**](SessionUser.md)> | IPS in the session | [optional]
**deployment** | [**crate::models::Deployment**](Deployment.md) |  | 
**webhook_url** | Option<**String**> | When your Session is Linked, Unprocessable or in Error, we will POST the session's details on the webhook_url  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


