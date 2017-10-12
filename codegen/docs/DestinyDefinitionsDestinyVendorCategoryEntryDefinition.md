# DestinyDefinitionsDestinyVendorCategoryEntryDefinition

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**category_index** | **i32** | The index of the category in the original category definitions for the vendor. | [optional] [default to null]
**category_id** | **String** | The string identifier of the category. | [optional] [default to null]
**category_hash** | **i32** | The hashed identifier for the category. (note that this is NOT pointing to a DestinyVendorCategoryDefinition, it&#39;s confusing but this is a sale item category in a vendor, not a categorization of vendors themselves) | [optional] [default to null]
**quantity_available** | **i32** | The amount of items that will be available when this category is shown. | [optional] [default to null]
**show_unavailable_items** | **bool** | If items aren&#39;t up for sale in this category, should we still show them (greyed out)? | [optional] [default to null]
**hide_if_no_currency** | **bool** | If you don&#39;t have the currency required to buy items from this category, should the items be hidden? | [optional] [default to null]
**hide_from_regular_purchase** | **bool** | True if this category doesn&#39;t allow purchases. | [optional] [default to null]
**buy_string_override** | **String** | The localized string for making purchases from this category, if it is different from the vendor&#39;s string for purchasing. | [optional] [default to null]
**disabled_description** | **String** | If the category is disabled, this is the localized description to show. | [optional] [default to null]
**display_title** | **String** | The localized title of the category. | [optional] [default to null]
**overlay** | [***Object**](Object.md) | If this category has an overlay prompt that should appear, this contains the details of that prompt. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


