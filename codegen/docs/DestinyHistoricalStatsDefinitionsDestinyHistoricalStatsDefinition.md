# DestinyHistoricalStatsDefinitionsDestinyHistoricalStatsDefinition

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**stat_id** | **String** | Unique programmer friendly ID for this stat | [optional] [default to null]
**group** | [***Object**](Object.md) | Statistic group | [optional] [default to null]
**period_types** | [**Vec<::models::DestinyHistoricalStatsDefinitionsPeriodType>**](Destiny.HistoricalStats.Definitions.PeriodType.md) | Time periods the statistic covers | [optional] [default to null]
**modes** | [**Vec<::models::DestinyHistoricalStatsDefinitionsDestinyActivityModeType>**](Destiny.HistoricalStats.Definitions.DestinyActivityModeType.md) | Game modes where this statistic can be reported. | [optional] [default to null]
**category** | [***Object**](Object.md) | Category for the stat. | [optional] [default to null]
**stat_name** | **String** | Display name | [optional] [default to null]
**stat_description** | **String** | Description of a stat if applicable. | [optional] [default to null]
**unit_type** | [***Object**](Object.md) | Unit, if any, for the statistic | [optional] [default to null]
**icon_image** | **String** | Optional URI to an icon for the statistic | [optional] [default to null]
**merge_method** | **i32** | Optional icon for the statistic | [optional] [default to null]
**unit_label** | **String** | Localized Unit Name for the stat. | [optional] [default to null]
**weight** | **i32** | Weight assigned to this stat indicating its relative impressiveness. | [optional] [default to null]
**medal_tier_hash** | **i32** | The tier associated with this medal - be it implicitly or explicitly. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


