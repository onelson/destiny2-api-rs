# DestinyDefinitionsDestinyActivityModeDefinition

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**display_properties** | [***::models::DestinyDefinitionsCommonDestinyDisplayPropertiesDefinition**](Destiny.Definitions.Common.DestinyDisplayPropertiesDefinition.md) |  | [optional] [default to null]
**pgcr_image** | **String** |  | [optional] [default to null]
**mode_type** | [***::models::DestinyHistoricalStatsDefinitionsDestinyActivityModeType**](Destiny.HistoricalStats.Definitions.DestinyActivityModeType.md) |  | [optional] [default to null]
**activity_mode_category** | [***::models::DestinyDestinyActivityModeCategory**](Destiny.DestinyActivityModeCategory.md) |  | [optional] [default to null]
**is_team_based** | **bool** |  | [optional] [default to null]
**is_aggregate_mode** | **bool** |  | [optional] [default to null]
**parent_hashes** | **Vec<i32>** |  | [optional] [default to null]
**friendly_name** | **String** |  | [optional] [default to null]
**activity_mode_mappings** | [**::std::collections::HashMap<String, ::models::DestinyHistoricalStatsDefinitionsDestinyActivityModeType>**](Destiny.HistoricalStats.Definitions.DestinyActivityModeType.md) |  | [optional] [default to null]
**display** | **bool** | If FALSE, we want to ignore this type when we&#39;re showing activity modes in BNet UI. It will still be returned in case 3rd parties want to use it for any purpose. | [optional] [default to null]
**order** | **i32** | The relative ordering of activity modes. | [optional] [default to null]
**hash** | **i32** | The unique identifier for this entity. Guaranteed to be unique for the type of entity, but not globally.  When entities refer to each other in Destiny content, it is this hash that they are referring to. | [optional] [default to null]
**index** | **i32** | The index of the entity as it was found in the investment tables. | [optional] [default to null]
**redacted** | **bool** | If this is true, then there is an entity with this identifier/type combination, but BNet is not yet allowed to show it. Sorry! | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


