# DestinyEntitiesVendorsDestinyVendorComponent

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**vendor_hash** | **i32** | The unique identifier for the vendor. Use it to look up their DestinyVendorDefinition. | [optional] [default to null]
**ack_state** | [***Object**](Object.md) | Long ago, we thought it would be a good idea to have special UI that showed whether or not you&#39;ve seen a Vendor&#39;s inventory after cycling.   For now, we don&#39;t have that UI anymore. This property still exists for historical purposes. Don&#39;t worry about it. | [optional] [default to null]
**next_refresh_date** | **String** | The date when this vendor&#39;s inventory will next rotate/refresh. | [optional] [default to null]
**enabled** | **bool** | If True, the Vendor is currently accessible.   If False, they may not actually be visible in the world at the moment. | [optional] [default to null]
**can_purchase** | **bool** | If True, you can purchase from the Vendor.  Theoretically, Vendors can be restricted from selling items. In practice, none do that (yet?). | [optional] [default to null]
**progression** | [***Object**](Object.md) | If the Vendor has a related Reputation, this is the Progression data that represents the character&#39;s Reputation level with this Vendor. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


