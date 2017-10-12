# ContentContentItemPublicContract

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**content_id** | **i64** |  | [optional] [default to null]
**c_type** | **String** |  | [optional] [default to null]
**cms_path** | **String** |  | [optional] [default to null]
**creation_date** | **String** |  | [optional] [default to null]
**modify_date** | **String** |  | [optional] [default to null]
**allow_comments** | **bool** |  | [optional] [default to null]
**has_age_gate** | **bool** |  | [optional] [default to null]
**minimum_age** | **i32** |  | [optional] [default to null]
**rating_image_path** | **String** |  | [optional] [default to null]
**author** | [***::models::UserGeneralUser**](User.GeneralUser.md) |  | [optional] [default to null]
**auto_english_property_fallback** | **bool** |  | [optional] [default to null]
**properties** | [**::std::collections::HashMap<String, Object>**](Object.md) | Firehose content is really a collection of metadata and \&quot;properties\&quot;, which are the potentially-but-not-strictly localizable data that comprises the meat of whatever content is being shown.  As Cole Porter would have crooned, \&quot;Anything Goes\&quot; with Firehose properties. They are most often strings, but they can theoretically be anything. They are JSON encoded, and could be JSON structures, simple strings, numbers etc... The Content Type of the item (cType) will describe the properties, and thus how they ought to be deserialized. | [optional] [default to null]
**representations** | [**Vec<::models::ContentContentRepresentation>**](Content.ContentRepresentation.md) |  | [optional] [default to null]
**tags** | **Vec<String>** |  | [optional] [default to null]
**comment_summary** | [***::models::ContentCommentSummary**](Content.CommentSummary.md) |  | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


