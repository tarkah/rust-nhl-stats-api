# \DraftApi

All URIs are relative to *https://statsapi.web.nhl.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_draft**](DraftApi.md#get_draft) | **Get** /draft | Get round-by-round data for current year's NHL Entry Draft.
[**get_draft_by_year**](DraftApi.md#get_draft_by_year) | **Get** /draft/{year} | Get round-by-round data for a specific year's NHL Entry Draft.
[**get_draft_prospect**](DraftApi.md#get_draft_prospect) | **Get** /draft/prospects/{id} | Get an NHL Entry Draft prospect.
[**get_draft_prospects**](DraftApi.md#get_draft_prospects) | **Get** /draft/prospects | Get all NHL Entry Draft prospects.



## get_draft

> crate::models::Draft get_draft()
Get round-by-round data for current year's NHL Entry Draft.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::Draft**](Draft.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_draft_by_year

> crate::models::Draft get_draft_by_year(year)
Get round-by-round data for a specific year's NHL Entry Draft.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**year** | **f32** | The draft year. | Required | 

### Return type

[**crate::models::Draft**](Draft.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_draft_prospect

> crate::models::DraftProspects get_draft_prospect(id)
Get an NHL Entry Draft prospect.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **f32** | The prospect ID. | Required | 

### Return type

[**crate::models::DraftProspects**](DraftProspects.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_draft_prospects

> crate::models::DraftProspects get_draft_prospects()
Get all NHL Entry Draft prospects.

Be forewarned that this endpoint returns a **lot** of data and there does not appear to be a way to paginate results.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::DraftProspects**](DraftProspects.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

