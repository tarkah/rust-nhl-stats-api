# \TeamsApi

All URIs are relative to *https://statsapi.web.nhl.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_team**](TeamsApi.md#get_team) | **Get** /teams/{id} | Get an NHL team.
[**get_team_roster**](TeamsApi.md#get_team_roster) | **Get** /teams/{id}/roster | Get an NHL team's roster.
[**get_team_stats**](TeamsApi.md#get_team_stats) | **Get** /teams/{id}/stats | Get all statistics for an NHL team.
[**get_teams**](TeamsApi.md#get_teams) | **Get** /teams | Get all NHL teams.



## get_team

> crate::models::Team get_team(id, expand, season)
Get an NHL team.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **f32** | The ID of the team. | Required | 
**expand** | [**crate::models::EnumExpandTeams**](.md) | Expand explanations:   * `team.roster` - Shows roster of active players for the specified team.   * `person.names` - Same as above, but gives less info.   * `team.schedule.next` - Returns details of the upcoming game for a team.   * `team.schedule.previous` - Same as above but for the last game played.   * `team.stats` - Returns the teams stats for the season.  |  | 
**season** | **String** | Return a team's specific season. |  | 

### Return type

[**crate::models::Team**](Team.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_team_roster

> crate::models::Rosters get_team_roster(id, season)
Get an NHL team's roster.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **f32** | The ID of the team. | Required | 
**season** | **String** | Return a team's specific season. |  | 

### Return type

[**crate::models::Rosters**](Rosters.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_team_stats

> crate::models::TeamStats get_team_stats(id)
Get all statistics for an NHL team.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **f32** | The ID of the team. | Required | 

### Return type

[**crate::models::TeamStats**](TeamStats.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_teams

> crate::models::Teams get_teams(expand, season)
Get all NHL teams.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**expand** | [**crate::models::EnumExpandTeams**](.md) | Expand explanations:   * `team.roster` - Shows roster of active players for the specified team.   * `person.names` - Same as above, but gives less info.   * `team.schedule.next` - Returns details of the upcoming game for a team.   * `team.schedule.previous` - Same as above but for the last game played.   * `team.stats` - Returns the teams stats for the season.  |  | 
**season** | **String** | Return a team's specific season. |  | 

### Return type

[**crate::models::Teams**](Teams.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

