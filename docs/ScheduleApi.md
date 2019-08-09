# \ScheduleApi

All URIs are relative to *https://statsapi.web.nhl.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_schedule**](ScheduleApi.md#get_schedule) | **Get** /schedule | Get the NHL game schedule.



## get_schedule

> crate::models::Schedule get_schedule(expand, team_id, start_date, end_date)
Get the NHL game schedule.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**expand** | [**crate::models::EnumExpandSchedule**](.md) | Expand explanations:   * `schedule.brodcasts` - Shows the broadcasts of the game.   * `schedule.linescore` - Linescore for completed games.   * `schedule.ticket` - Provides the different places to buy tickets for the upcoming games.  |  | 
**team_id** | **String** | Limit results to a specific team. Team ids can be found through the teams endpoint |  | 
**start_date** | **String** | Start date for the search. |  | 
**end_date** | **String** | End date for the search. |  | 

### Return type

[**crate::models::Schedule**](Schedule.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

