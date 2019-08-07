# \DivisionsApi

All URIs are relative to *https://statsapi.web.nhl.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_division**](DivisionsApi.md#get_division) | **Get** /divisions/{id} | Get an NHL division.
[**get_divisions**](DivisionsApi.md#get_divisions) | **Get** /divisions | Get all current NHL divisions.



## get_division

> crate::models::Division get_division(id)
Get an NHL division.

You can use this to also retrieve inactive divisions. For example, the ID for the old Patrick division is `13`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **f32** | The ID of the division. | Required | 

### Return type

[**crate::models::Division**](Division.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_divisions

> crate::models::Divisions get_divisions()
Get all current NHL divisions.

This only retrieves active divisions. For inactive divisions, use `/divisions/{id}`.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::Divisions**](Divisions.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

