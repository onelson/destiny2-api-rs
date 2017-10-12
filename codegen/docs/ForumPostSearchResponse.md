# ForumPostSearchResponse

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**related_posts** | [**Vec<::models::ForumPostResponse>**](Forum.PostResponse.md) |  | [optional] [default to null]
**authors** | [**Vec<::models::UserGeneralUser>**](User.GeneralUser.md) |  | [optional] [default to null]
**groups** | [**Vec<::models::GroupsV2GroupResponse>**](GroupsV2.GroupResponse.md) |  | [optional] [default to null]
**searched_tags** | [**Vec<::models::TagsModelsContractsTagResponse>**](Tags.Models.Contracts.TagResponse.md) |  | [optional] [default to null]
**polls** | [**Vec<::models::ForumPollResponse>**](Forum.PollResponse.md) |  | [optional] [default to null]
**recruitment_details** | [**Vec<::models::ForumForumRecruitmentDetail>**](Forum.ForumRecruitmentDetail.md) |  | [optional] [default to null]
**available_pages** | **i32** |  | [optional] [default to null]
**results** | [**Vec<::models::ForumPostResponse>**](Forum.PostResponse.md) |  | [optional] [default to null]
**total_results** | **i32** |  | [optional] [default to null]
**has_more** | **bool** |  | [optional] [default to null]
**query** | [***::models::QueriesPagedQuery**](Queries.PagedQuery.md) |  | [optional] [default to null]
**replacement_continuation_token** | **String** |  | [optional] [default to null]
**use_total_results** | **bool** | If useTotalResults is true, then totalResults represents an accurate count.  If False, it does not, and may be estimated/only the size of the current page.  Either way, you should probably always only trust hasMore.  This is a long-held historical throwback to when we used to do paging with known total results. Those queries toasted our database, and we were left to hastily alter our endpoints and create backward- compatible shims, of which useTotalResults is one. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


