# DestinyDefinitionsDestinyVendorInventoryFlyoutDefinition

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**locked_description** | **String** | If the flyout is locked, this is the reason why. | [optional] [default to null]
**display_properties** | [***Object**](Object.md) | The title and other common properties of the flyout. | [optional] [default to null]
**buckets** | [**Vec<::models::DestinyDefinitionsDestinyVendorInventoryFlyoutBucketDefinition>**](Destiny.Definitions.DestinyVendorInventoryFlyoutBucketDefinition.md) | A list of inventory buckets and other metadata to show on the screen. | [optional] [default to null]
**flyout_id** | **i32** | An identifier for the flyout, in case anything else needs to refer to them. | [optional] [default to null]
**suppress_newness** | **bool** | If this is true, don&#39;t show any of the glistening \&quot;this is a new item\&quot; UI elements, like we show on the inventory items themselves in in-game UI. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


