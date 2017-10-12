# \TrendingApi

All URIs are relative to *https://www.bungie.net/Platform*

Method | HTTP request | Description
------------- | ------------- | -------------
[**trending_get_trending_categories**](TrendingApi.md#trending_get_trending_categories) | **Get** /Trending/Categories/ | 
[**trending_get_trending_category**](TrendingApi.md#trending_get_trending_category) | **Get** /Trending/Categories/{categoryId}/{pageNumber}/ | 
[**trending_get_trending_entry_detail**](TrendingApi.md#trending_get_trending_entry_detail) | **Get** /Trending/Details/{trendingEntryType}/{identifier}/ | 


# **trending_get_trending_categories**
> ::models::InlineResponse20052 trending_get_trending_categories()


Returns trending items for Bungie.net, collapsed into the first page of items per category. For pagination within a category, call GetTrendingCategory.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::InlineResponse20052**](inline_response_200_52.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **trending_get_trending_category**
> ::models::InlineResponse20053 trending_get_trending_category(category_id, page_number)


Returns paginated lists of trending items for a category.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **category_id** | **String**| The ID of the category for whom you want additional results. | 
  **page_number** | **i32**| The page # of results to return. | 

### Return type

[**::models::InlineResponse20053**](inline_response_200_53.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **trending_get_trending_entry_detail**
> ::models::InlineResponse20054 trending_get_trending_entry_detail(identifier, trending_entry_type)


Returns the detailed results for a specific trending entry. Note that trending entries are uniquely identified by a combination of *both* the TrendingEntryType *and* the identifier: the identifier alone is not guaranteed to be globally unique.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **identifier** | **String**| The identifier for the entity to be returned. | 
  **trending_entry_type** | **i32**| The type of entity to be returned. | 

### Return type

[**::models::InlineResponse20054**](inline_response_200_54.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

