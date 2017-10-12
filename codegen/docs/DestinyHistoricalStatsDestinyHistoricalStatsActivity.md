# DestinyHistoricalStatsDestinyHistoricalStatsActivity

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**reference_id** | **i32** | Hash ID that can be looked up in the DestinyActivityTable. | [optional] [default to null]
**director_activity_hash** | **i32** |  | [optional] [default to null]
**instance_id** | **i64** | This value can be used to get additional data about this activity such as who else was playing. | [optional] [default to null]
**mode** | [***Object**](Object.md) | Indicates the game mode of the activity. | [optional] [default to null]
**modes** | [**Vec<::models::DestinyHistoricalStatsDefinitionsDestinyActivityModeType>**](Destiny.HistoricalStats.Definitions.DestinyActivityModeType.md) |  | [optional] [default to null]
**is_private** | **bool** | Whether or not the match was a private match. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


