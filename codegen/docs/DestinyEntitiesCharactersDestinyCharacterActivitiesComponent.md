# DestinyEntitiesCharactersDestinyCharacterActivitiesComponent

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**date_activity_started** | **String** | The last date that the user started playing an activity. | [optional] [default to null]
**available_activities** | [**Vec<::models::DestinyDestinyActivity>**](Destiny.DestinyActivity.md) | The list of activities that the user can play. | [optional] [default to null]
**current_activity_hash** | **i32** | If the user is in an activity, this will be the hash of the Activity being played. Note that you must combine this info with currentActivityModeHash to get a real picture of what the user is doing right now. For instance, PVP \&quot;Activities\&quot; are just maps: it&#39;s the ActivityMode that determines what type of PVP game they&#39;re playing. | [optional] [default to null]
**current_activity_mode_hash** | **i32** | If the user is in an activity, this will be the hash of the activity mode being played. Combine with currentActivityHash to give a person a full picture of what they&#39;re doing right now. | [optional] [default to null]
**current_activity_mode_type** | **i32** |  | [optional] [default to null]
**current_activity_mode_hashes** | **Vec<i32>** |  | [optional] [default to null]
**current_activity_mode_types** | [**Vec<::models::DestinyHistoricalStatsDefinitionsDestinyActivityModeType>**](Destiny.HistoricalStats.Definitions.DestinyActivityModeType.md) |  | [optional] [default to null]
**current_playlist_activity_hash** | **i32** |  | [optional] [default to null]
**last_completed_story_hash** | **i32** | This will have the activity hash of the last completed story/campaign mission, in case you care about that. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


