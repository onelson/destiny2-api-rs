# DestinyVendorsDestinyVendorReceipt

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**currency_paid** | [**Vec<::models::DestinyDestinyItemQuantity>**](Destiny.DestinyItemQuantity.md) | The amount paid for the item, in terms of items that were consumed in the purchase and their quantity. | [optional] [default to null]
**item_received** | [***Object**](Object.md) | The item that was received, and its quantity. | [optional] [default to null]
**license_unlock_hash** | **i32** | The unlock flag used to determine whether you still have the purchased item. | [optional] [default to null]
**purchased_by_character_id** | **i64** | The ID of the character who made the purchase. | [optional] [default to null]
**refund_policy** | [***Object**](Object.md) | Whether you can get a refund, and what happens in order for the refund to be received. See the DestinyVendorItemRefundPolicy enum for details. | [optional] [default to null]
**sequence_number** | **i32** | The identifier of this receipt. | [optional] [default to null]
**time_to_expiration** | **i64** | The seconds since epoch at which this receipt is rendered invalid. | [optional] [default to null]
**expires_on** | **String** | The date at which this receipt is rendered invalid. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


