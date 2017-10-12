# \PreviewApi

All URIs are relative to *https://www.bungie.net/Platform*

Method | HTTP request | Description
------------- | ------------- | -------------
[**destiny2_activate_talent_node**](PreviewApi.md#destiny2_activate_talent_node) | **Post** /Destiny2/Actions/Items/ActivateTalentNode/ | 
[**destiny2_get_activity_history**](PreviewApi.md#destiny2_get_activity_history) | **Get** /Destiny2/{membershipType}/Account/{destinyMembershipId}/Character/{characterId}/Stats/Activities/ | 
[**destiny2_get_clan_aggregate_stats**](PreviewApi.md#destiny2_get_clan_aggregate_stats) | **Get** /Destiny2/Stats/AggregateClanStats/{groupId}/ | 
[**destiny2_get_clan_leaderboards**](PreviewApi.md#destiny2_get_clan_leaderboards) | **Get** /Destiny2/Stats/Leaderboards/Clans/{groupId}/ | 
[**destiny2_get_destiny_aggregate_activity_stats**](PreviewApi.md#destiny2_get_destiny_aggregate_activity_stats) | **Get** /Destiny2/{membershipType}/Account/{destinyMembershipId}/Character/{characterId}/Stats/AggregateActivityStats/ | 
[**destiny2_get_historical_stats**](PreviewApi.md#destiny2_get_historical_stats) | **Get** /Destiny2/{membershipType}/Account/{destinyMembershipId}/Character/{characterId}/Stats/ | 
[**destiny2_get_historical_stats_for_account**](PreviewApi.md#destiny2_get_historical_stats_for_account) | **Get** /Destiny2/{membershipType}/Account/{destinyMembershipId}/Stats/ | 
[**destiny2_get_leaderboards**](PreviewApi.md#destiny2_get_leaderboards) | **Get** /Destiny2/{membershipType}/Account/{destinyMembershipId}/Stats/Leaderboards/ | 
[**destiny2_get_leaderboards_for_character**](PreviewApi.md#destiny2_get_leaderboards_for_character) | **Get** /Destiny2/Stats/Leaderboards/{membershipType}/{destinyMembershipId}/{characterId}/ | 
[**destiny2_get_unique_weapon_history**](PreviewApi.md#destiny2_get_unique_weapon_history) | **Get** /Destiny2/{membershipType}/Account/{destinyMembershipId}/Character/{characterId}/Stats/UniqueWeapons/ | 
[**destiny2_get_vendor**](PreviewApi.md#destiny2_get_vendor) | **Get** /Destiny2/{membershipType}/Profile/{destinyMembershipId}/Character/{characterId}/Vendors/{vendorHash}/ | 
[**destiny2_get_vendors**](PreviewApi.md#destiny2_get_vendors) | **Get** /Destiny2/{membershipType}/Profile/{destinyMembershipId}/Character/{characterId}/Vendors/ | 
[**destiny2_insert_socket_plug**](PreviewApi.md#destiny2_insert_socket_plug) | **Post** /Destiny2/Actions/Items/InsertSocketPlug/ | 
[**destiny2_search_destiny_entities**](PreviewApi.md#destiny2_search_destiny_entities) | **Get** /Destiny2/Armory/Search/{type}/{searchTerm}/ | 


# **destiny2_activate_talent_node**
> ::models::InlineResponse20015 destiny2_activate_talent_node(ctx, )


Activate a Talent Node. Chill out, everyone: we haven't decided yet whether this will be able to activate nodes with costs, but if we do it will require special scope permission for an application attempting to do so. You must have a valid Destiny Account, and either be in a social space, in orbit, or offline. PREVIEW: This service is not actually implemented yet, but we are returning the planned schema of the endpoint for review, comment, and preparation for its eventual implementation.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::InlineResponse20015**](inline_response_200_15.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **destiny2_get_activity_history**
> ::models::InlineResponse20045 destiny2_get_activity_history(character_id, destiny_membership_id, membership_type, optional)


Gets activity history stats for indicated character. PREVIEW: This endpoint is still in beta, and may experience rough edges. The schema is in final form, but there may be bugs that prevent desirable operation.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **character_id** | **i64**| The id of the character to retrieve. | 
  **destiny_membership_id** | **i64**| The Destiny membershipId of the user to retrieve. | 
  **membership_type** | **i32**| A valid non-BungieNet membership type. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **character_id** | **i64**| The id of the character to retrieve. | 
 **destiny_membership_id** | **i64**| The Destiny membershipId of the user to retrieve. | 
 **membership_type** | **i32**| A valid non-BungieNet membership type. | 
 **count** | **i32**| Number of rows to return | 
 **mode** | **i32**| A filter for the activity mode to be returned. None returns all activities. See the documentation for DestinyActivityModeType for valid values, and pass in string representation. | 
 **page** | **i32**| Page number to return, starting with 0. | 

### Return type

[**::models::InlineResponse20045**](inline_response_200_45.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **destiny2_get_clan_aggregate_stats**
> ::models::InlineResponse20041 destiny2_get_clan_aggregate_stats(group_id, optional)


Gets aggregated stats for a clan using the same categories as the clan leaderboards. PREVIEW: This endpoint is still in beta, and may experience rough edges. The schema is in final form, but there may be bugs that prevent desirable operation.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **group_id** | **i64**| Group ID of the clan whose leaderboards you wish to fetch. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **group_id** | **i64**| Group ID of the clan whose leaderboards you wish to fetch. | 
 **modes** | **String**| List of game modes for which to get leaderboards. See the documentation for DestinyActivityModeType for valid values, and pass in string representation, comma delimited. | 

### Return type

[**::models::InlineResponse20041**](inline_response_200_41.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **destiny2_get_clan_leaderboards**
> ::models::InlineResponse20040 destiny2_get_clan_leaderboards(group_id, optional)


Gets leaderboards with the signed in user's friends and the supplied destinyMembershipId as the focus. PREVIEW: This endpoint is still in beta, and may experience rough edges. The schema is in final form, but there may be bugs that prevent desirable operation.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **group_id** | **i64**| Group ID of the clan whose leaderboards you wish to fetch. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **group_id** | **i64**| Group ID of the clan whose leaderboards you wish to fetch. | 
 **maxtop** | **i32**| Maximum number of top players to return. Use a large number to get entire leaderboard. | 
 **modes** | **String**| List of game modes for which to get leaderboards. See the documentation for DestinyActivityModeType for valid values, and pass in string representation, comma delimited. | 
 **statid** | **String**| ID of stat to return rather than returning all Leaderboard stats. | 

### Return type

[**::models::InlineResponse20040**](inline_response_200_40.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **destiny2_get_destiny_aggregate_activity_stats**
> ::models::InlineResponse20047 destiny2_get_destiny_aggregate_activity_stats(character_id, destiny_membership_id, membership_type)


Gets all activities the character has participated in together with aggregate statistics for those activities. PREVIEW: This endpoint is still in beta, and may experience rough edges. The schema is in final form, but there may be bugs that prevent desirable operation.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **character_id** | **i64**| The specific character whose activities should be returned. | 
  **destiny_membership_id** | **i64**| The Destiny membershipId of the user to retrieve. | 
  **membership_type** | **i32**| A valid non-BungieNet membership type. | 

### Return type

[**::models::InlineResponse20047**](inline_response_200_47.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **destiny2_get_historical_stats**
> ::models::InlineResponse20043 destiny2_get_historical_stats(character_id, destiny_membership_id, membership_type, optional)


Gets historical stats for indicated character. PREVIEW: This endpoint is still in beta, and may experience rough edges. The schema is in final form, but there may be bugs that prevent desirable operation.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **character_id** | **i64**| The id of the character to retrieve. You can omit this character ID or set it to 0 to get aggregate stats across all characters. | 
  **destiny_membership_id** | **i64**| The Destiny membershipId of the user to retrieve. | 
  **membership_type** | **i32**| A valid non-BungieNet membership type. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **character_id** | **i64**| The id of the character to retrieve. You can omit this character ID or set it to 0 to get aggregate stats across all characters. | 
 **destiny_membership_id** | **i64**| The Destiny membershipId of the user to retrieve. | 
 **membership_type** | **i32**| A valid non-BungieNet membership type. | 
 **dayend** | **String**| Last day to return when daily stats are requested. Use the format YYYY-MM-DD. | 
 **daystart** | **String**| First day to return when daily stats are requested. Use the format YYYY-MM-DD | 
 **groups** | [**Vec&lt;::models::DestinyHistoricalStatsDefinitionsDestinyStatsGroupType&gt;**](::models::DestinyHistoricalStatsDefinitionsDestinyStatsGroupType.md)| Group of stats to include, otherwise only general stats are returned. Comma separated list is allowed. Values: General, Weapons, Medals | 
 **modes** | [**Vec&lt;::models::DestinyHistoricalStatsDefinitionsDestinyActivityModeType&gt;**](::models::DestinyHistoricalStatsDefinitionsDestinyActivityModeType.md)| Game modes to return. See the documentation for DestinyActivityModeType for valid values, and pass in string representation, comma delimited. | 
 **period_type** | **i32**| Indicates a specific period type to return. Optional. May be: Daily, AllTime, or Activity | 

### Return type

[**::models::InlineResponse20043**](inline_response_200_43.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **destiny2_get_historical_stats_for_account**
> ::models::InlineResponse20044 destiny2_get_historical_stats_for_account(destiny_membership_id, membership_type, optional)


Gets aggregate historical stats organized around each character for a given account. PREVIEW: This endpoint is still in beta, and may experience rough edges. The schema is in final form, but there may be bugs that prevent desirable operation.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **destiny_membership_id** | **i64**| The Destiny membershipId of the user to retrieve. | 
  **membership_type** | **i32**| A valid non-BungieNet membership type. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **destiny_membership_id** | **i64**| The Destiny membershipId of the user to retrieve. | 
 **membership_type** | **i32**| A valid non-BungieNet membership type. | 
 **groups** | [**Vec&lt;::models::DestinyHistoricalStatsDefinitionsDestinyStatsGroupType&gt;**](::models::DestinyHistoricalStatsDefinitionsDestinyStatsGroupType.md)| Groups of stats to include, otherwise only general stats are returned. Comma separated list is allowed. Values: General, Weapons, Medals. | 

### Return type

[**::models::InlineResponse20044**](inline_response_200_44.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **destiny2_get_leaderboards**
> ::models::InlineResponse20040 destiny2_get_leaderboards(destiny_membership_id, membership_type, optional)


Gets leaderboards with the signed in user's friends and the supplied destinyMembershipId as the focus. PREVIEW: This endpoint has not yet been implemented. It is being returned for a preview of future functionality, and for public comment/suggestion/preparation.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **destiny_membership_id** | **i64**| The Destiny membershipId of the user to retrieve. | 
  **membership_type** | **i32**| A valid non-BungieNet membership type. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **destiny_membership_id** | **i64**| The Destiny membershipId of the user to retrieve. | 
 **membership_type** | **i32**| A valid non-BungieNet membership type. | 
 **maxtop** | **i32**| Maximum number of top players to return. Use a large number to get entire leaderboard. | 
 **modes** | **String**| List of game modes for which to get leaderboards. See the documentation for DestinyActivityModeType for valid values, and pass in string representation, comma delimited. | 
 **statid** | **String**| ID of stat to return rather than returning all Leaderboard stats. | 

### Return type

[**::models::InlineResponse20040**](inline_response_200_40.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **destiny2_get_leaderboards_for_character**
> ::models::InlineResponse20040 destiny2_get_leaderboards_for_character(character_id, destiny_membership_id, membership_type, optional)


Gets leaderboards with the signed in user's friends and the supplied destinyMembershipId as the focus. PREVIEW: This endpoint is still in beta, and may experience rough edges. The schema is in final form, but there may be bugs that prevent desirable operation.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **character_id** | **i64**| The specific character to build the leaderboard around for the provided Destiny Membership. | 
  **destiny_membership_id** | **i64**| The Destiny membershipId of the user to retrieve. | 
  **membership_type** | **i32**| A valid non-BungieNet membership type. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **character_id** | **i64**| The specific character to build the leaderboard around for the provided Destiny Membership. | 
 **destiny_membership_id** | **i64**| The Destiny membershipId of the user to retrieve. | 
 **membership_type** | **i32**| A valid non-BungieNet membership type. | 
 **maxtop** | **i32**| Maximum number of top players to return. Use a large number to get entire leaderboard. | 
 **modes** | **String**| List of game modes for which to get leaderboards. See the documentation for DestinyActivityModeType for valid values, and pass in string representation, comma delimited. | 
 **statid** | **String**| ID of stat to return rather than returning all Leaderboard stats. | 

### Return type

[**::models::InlineResponse20040**](inline_response_200_40.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **destiny2_get_unique_weapon_history**
> ::models::InlineResponse20046 destiny2_get_unique_weapon_history(character_id, destiny_membership_id, membership_type)


Gets details about unique weapon usage, including all exotic weapons. PREVIEW: This endpoint is still in beta, and may experience rough edges. The schema is in final form, but there may be bugs that prevent desirable operation.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **character_id** | **i64**| The id of the character to retrieve. | 
  **destiny_membership_id** | **i64**| The Destiny membershipId of the user to retrieve. | 
  **membership_type** | **i32**| A valid non-BungieNet membership type. | 

### Return type

[**::models::InlineResponse20046**](inline_response_200_46.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **destiny2_get_vendor**
> ::models::InlineResponse20036 destiny2_get_vendor(character_id, destiny_membership_id, membership_type, vendor_hash, optional)


Get the details of a specific Vendor. PREVIEW: This service is not yet active, but we are returning the planned schema of the endpoint for review, comment, and preparation for its eventual implementation.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **character_id** | **i64**| The Destiny Character ID of the character for whom we&#39;re getting vendor info. | 
  **destiny_membership_id** | **i64**| Destiny membership ID of another user. You may be denied. | 
  **membership_type** | **i32**| A valid non-BungieNet membership type. | 
  **vendor_hash** | **i32**| The Hash identifier of the Vendor to be returned. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **character_id** | **i64**| The Destiny Character ID of the character for whom we&#39;re getting vendor info. | 
 **destiny_membership_id** | **i64**| Destiny membership ID of another user. You may be denied. | 
 **membership_type** | **i32**| A valid non-BungieNet membership type. | 
 **vendor_hash** | **i32**| The Hash identifier of the Vendor to be returned. | 
 **components** | [**Vec&lt;::models::DestinyDestinyComponentType&gt;**](::models::DestinyDestinyComponentType.md)| A comma separated list of components to return (as strings or numeric values). See the DestinyComponentType enum for valid components to request. You must request at least one component to receive results. | 

### Return type

[**::models::InlineResponse20036**](inline_response_200_36.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **destiny2_get_vendors**
> ::models::InlineResponse20035 destiny2_get_vendors(character_id, destiny_membership_id, membership_type, optional)


Get currently available vendors. PREVIEW: This service is not yet active, but we are returning the planned schema of the endpoint for review, comment, and preparation for its eventual implementation.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **character_id** | **i64**| The Destiny Character ID of the character for whom we&#39;re getting vendor info. | 
  **destiny_membership_id** | **i64**| Destiny membership ID of another user. You may be denied. | 
  **membership_type** | **i32**| A valid non-BungieNet membership type. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **character_id** | **i64**| The Destiny Character ID of the character for whom we&#39;re getting vendor info. | 
 **destiny_membership_id** | **i64**| Destiny membership ID of another user. You may be denied. | 
 **membership_type** | **i32**| A valid non-BungieNet membership type. | 
 **components** | [**Vec&lt;::models::DestinyDestinyComponentType&gt;**](::models::DestinyDestinyComponentType.md)| A comma separated list of components to return (as strings or numeric values). See the DestinyComponentType enum for valid components to request. You must request at least one component to receive results. | 

### Return type

[**::models::InlineResponse20035**](inline_response_200_35.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **destiny2_insert_socket_plug**
> ::models::InlineResponse20015 destiny2_insert_socket_plug(ctx, )


Insert a plug into a socketed item. I know how it sounds, but I assure you it's much more G-rated than you might be guessing. We haven't decided yet whether this will be able to insert plugs that have side effects, but if we do it will require special scope permission for an application attempting to do so. You must have a valid Destiny Account, and either be in a social space, in orbit, or offline. PREVIEW: This service is not yet active, but we are returning the planned schema of the endpoint for review, comment, and preparation for its eventual implementation.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::InlineResponse20015**](inline_response_200_15.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **destiny2_search_destiny_entities**
> ::models::InlineResponse20042 destiny2_search_destiny_entities(search_term, _type, optional)


Gets a page list of Destiny items.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **search_term** | **String**| The string to use when searching for Destiny entities. | 
  **_type** | **String**| The type of entity for whom you would like results. These correspond to the entity&#39;s definition contract name. For instance, if you are looking for items, this property should be &#39;DestinyInventoryItemDefinition&#39;. PREVIEW: This endpoint is still in beta, and may experience rough edges. The schema is tentatively in final form, but there may be bugs that prevent desirable operation. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **search_term** | **String**| The string to use when searching for Destiny entities. | 
 **_type** | **String**| The type of entity for whom you would like results. These correspond to the entity&#39;s definition contract name. For instance, if you are looking for items, this property should be &#39;DestinyInventoryItemDefinition&#39;. PREVIEW: This endpoint is still in beta, and may experience rough edges. The schema is tentatively in final form, but there may be bugs that prevent desirable operation. | 
 **page** | **i32**| Page number to return, starting with 0. | 

### Return type

[**::models::InlineResponse20042**](inline_response_200_42.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

