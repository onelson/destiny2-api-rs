# DestinyDefinitionsMilestonesDestinyMilestoneQuestRewardItem

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**vendor_hash** | **i32** | The quest reward item *may* be associated with a vendor. If so, this is that vendor. Use this hash to look up the DestinyVendorDefinition. | [optional] [default to null]
**vendor_item_index** | **i32** | The quest reward item *may* be associated with a vendor. If so, this is the index of the item being sold, which we can use at runtime to find instanced item information for the reward item. | [optional] [default to null]
**item_hash** | **i32** | The hash identifier for the item in question. Use it to look up the item&#39;s DestinyInventoryItemDefinition. | [optional] [default to null]
**item_instance_id** | **i64** | If this quantity is referring to a specific instance of an item, this will have the item&#39;s instance ID. Normally, this will be null. | [optional] [default to null]
**quantity** | **i32** | The amount of the item needed/available depending on the context of where DestinyItemQuantity is being used. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


