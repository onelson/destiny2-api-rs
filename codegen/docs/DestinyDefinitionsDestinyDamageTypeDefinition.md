# DestinyDefinitionsDestinyDamageTypeDefinition

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**display_properties** | [***Object**](Object.md) | The description of the damage type, icon etc... | [optional] [default to null]
**transparent_icon_path** | **String** | A variant of the icon that is transparent and colorless. | [optional] [default to null]
**show_icon** | **bool** | If TRUE, the game shows this damage type&#39;s icon. Otherwise, it doesn&#39;t. Whether you show it or not is up to you. | [optional] [default to null]
**enum_value** | [***Object**](Object.md) | We have an enumeration for damage types for quick reference. This is the current definition&#39;s damage type enum value. | [optional] [default to null]
**hash** | **i32** | The unique identifier for this entity. Guaranteed to be unique for the type of entity, but not globally.  When entities refer to each other in Destiny content, it is this hash that they are referring to. | [optional] [default to null]
**index** | **i32** | The index of the entity as it was found in the investment tables. | [optional] [default to null]
**redacted** | **bool** | If this is true, then there is an entity with this identifier/type combination, but BNet is not yet allowed to show it. Sorry! | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


