# DestinyDefinitionsDestinyItemPreviewBlockDefinition

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**preview_vendor_hash** | **i32** | If the preview data is derived from a fake \&quot;Preview\&quot; Vendor, this will be the hash identifier for the DestinyVendorDefinition of that fake vendor. | [optional] [default to null]
**preview_action_string** | **String** | If the preview has an associated action (like \&quot;Open\&quot;), this will be the localized string for that action. | [optional] [default to null]
**derived_item_categories** | [**Vec<::models::DestinyDefinitionsItemsDestinyDerivedItemCategoryDefinition>**](Destiny.Definitions.Items.DestinyDerivedItemCategoryDefinition.md) | This is a list of the items being previewed, categorized in the same way as they are in the preview UI. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


