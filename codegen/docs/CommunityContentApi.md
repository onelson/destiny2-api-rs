# \CommunityContentApi

All URIs are relative to *https://www.bungie.net/Platform*

Method | HTTP request | Description
------------- | ------------- | -------------
[**community_content_get_community_content**](CommunityContentApi.md#community_content_get_community_content) | **Get** /CommunityContent/Get/{sort}/{mediaFilter}/{page}/ | 
[**community_content_get_community_live_statuses**](CommunityContentApi.md#community_content_get_community_live_statuses) | **Get** /CommunityContent/Live/All/{partnershipType}/{sort}/{page}/ | 
[**community_content_get_community_live_statuses_for_clanmates**](CommunityContentApi.md#community_content_get_community_live_statuses_for_clanmates) | **Get** /CommunityContent/Live/Clan/{partnershipType}/{sort}/{page}/ | 
[**community_content_get_community_live_statuses_for_friends**](CommunityContentApi.md#community_content_get_community_live_statuses_for_friends) | **Get** /CommunityContent/Live/Friends/{partnershipType}/{sort}/{page}/ | 
[**community_content_get_featured_community_live_statuses**](CommunityContentApi.md#community_content_get_featured_community_live_statuses) | **Get** /CommunityContent/Live/Featured/{partnershipType}/{sort}/{page}/ | 
[**community_content_get_streaming_status_for_member**](CommunityContentApi.md#community_content_get_streaming_status_for_member) | **Get** /CommunityContent/Live/Users/{partnershipType}/{membershipType}/{membershipId}/ | 


# **community_content_get_community_content**
> ::models::InlineResponse2006 community_content_get_community_content(media_filter, page, sort)


Returns community content.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **media_filter** | **i32**| The type of media to get | 
  **page** | **i32**| Zero based page | 
  **sort** | **i32**| The sort mode. | 

### Return type

[**::models::InlineResponse2006**](inline_response_200_6.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **community_content_get_community_live_statuses**
> ::models::InlineResponse20050 community_content_get_community_live_statuses(page, partnership_type, sort, optional)


Returns info about community members who are live streaming.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **page** | **i32**| Zero based page. | 
  **partnership_type** | **i32**| The type of partnership for which the status should be returned. | 
  **sort** | **i32**| The sort mode. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **page** | **i32**| Zero based page. | 
 **partnership_type** | **i32**| The type of partnership for which the status should be returned. | 
 **sort** | **i32**| The sort mode. | 
 **mode_hash** | **i32**| The hash of the Activity Mode for which streams should be retrieved. Don&#39;t pass it in or pass 0 to not filter by mode. | 
 **stream_locale** | **String**| The locale for streams you&#39;d like to see. Don&#39;t pass this to fall back on your BNet locale. Pass &#39;ALL&#39; to not filter by locale. | 

### Return type

[**::models::InlineResponse20050**](inline_response_200_50.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **community_content_get_community_live_statuses_for_clanmates**
> ::models::InlineResponse20050 community_content_get_community_live_statuses_for_clanmates(page, partnership_type, sort)


Returns info about community members who are live streaming in your clans.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **page** | **i32**| Zero based page. | 
  **partnership_type** | **i32**| The type of partnership for which the status should be returned. | 
  **sort** | **i32**| The sort mode. | 

### Return type

[**::models::InlineResponse20050**](inline_response_200_50.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **community_content_get_community_live_statuses_for_friends**
> ::models::InlineResponse20050 community_content_get_community_live_statuses_for_friends(page, partnership_type, sort)


Returns info about community members who are live streaming among your friends.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **page** | **i32**| Zero based page. | 
  **partnership_type** | **i32**| The type of partnership for which the status should be returned. | 
  **sort** | **i32**| The sort mode. | 

### Return type

[**::models::InlineResponse20050**](inline_response_200_50.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **community_content_get_featured_community_live_statuses**
> ::models::InlineResponse20050 community_content_get_featured_community_live_statuses(page, partnership_type, sort, optional)


Returns info about Featured live streams.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **page** | **i32**| Zero based page. | 
  **partnership_type** | **i32**| The type of partnership for which the status should be returned. | 
  **sort** | **i32**| The sort mode. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **page** | **i32**| Zero based page. | 
 **partnership_type** | **i32**| The type of partnership for which the status should be returned. | 
 **sort** | **i32**| The sort mode. | 
 **stream_locale** | **String**| The locale for streams you&#39;d like to see. Don&#39;t pass this to fall back on your BNet locale. Pass &#39;ALL&#39; to not filter by locale. | 

### Return type

[**::models::InlineResponse20050**](inline_response_200_50.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **community_content_get_streaming_status_for_member**
> ::models::InlineResponse20051 community_content_get_streaming_status_for_member(membership_id, membership_type, partnership_type)


Gets the Live Streaming status of a particular Account and Membership Type.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **membership_id** | **i64**| The membershipId related to that type. | 
  **membership_type** | **i32**| The type of account for which info will be extracted. | 
  **partnership_type** | **i32**| The type of partnership for which info will be extracted. | 

### Return type

[**::models::InlineResponse20051**](inline_response_200_51.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

