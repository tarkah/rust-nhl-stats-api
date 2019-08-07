# \ConferencesApi

All URIs are relative to *https://statsapi.web.nhl.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_conference**](ConferencesApi.md#get_conference) | **Get** /conferences/{id} | Get an NHL conference.
[**get_conferences**](ConferencesApi.md#get_conferences) | **Get** /conferences | Get all current NHL conferences.



## get_conference

> crate::models::Division get_conference(id)
Get an NHL conference.

You can use this to also retrieve inactive conferences. For example, the ID for the World Cup of Hockey is `7`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **f32** | The ID of the conference. | Required | 

### Return type

[**crate::models::Division**](Division.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_conferences

> crate::models::Conferences get_conferences()
Get all current NHL conferences.

This only retrieves active conferences. For inactive conferences, use `/conferences/{id}`.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::Conferences**](Conferences.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

