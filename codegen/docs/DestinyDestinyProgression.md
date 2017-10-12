# DestinyDestinyProgression

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**progression_hash** | **i32** | The hash identifier of the Progression in question. Use it to look up the DestinyProgressionDefinition in static data. | [optional] [default to null]
**daily_progress** | **i32** | The amount of progress earned today for this progression. | [optional] [default to null]
**daily_limit** | **i32** | If this progression has a daily limit, this is that limit. | [optional] [default to null]
**weekly_progress** | **i32** | The amount of progress earned toward this progression in the current week. | [optional] [default to null]
**weekly_limit** | **i32** | If this progression has a weekly limit, this is that limit. | [optional] [default to null]
**current_progress** | **i32** | This is the total amount of progress obtained overall for this progression (for instance, the total amount of Character Level experience earned) | [optional] [default to null]
**level** | **i32** | This is the level of the progression (for instance, the Character Level). | [optional] [default to null]
**level_cap** | **i32** | This is the maximum possible level you can achieve for this progression (for example, the maximum character level obtainable) | [optional] [default to null]
**step_index** | **i32** | Progressions define their levels in \&quot;steps\&quot;. Since the last step may be repeatable, the user may be at a higher level than the actual Step achieved in the progression. Not necessarily useful, but potentially interesting for those cruising the API. Relate this to the \&quot;steps\&quot; property of the DestinyProgression to see which step the user is on, if you care about that. (Note that this is Content Version dependent since it refers to indexes.) | [optional] [default to null]
**progress_to_next_level** | **i32** | The amount of progression (i.e. \&quot;Experience\&quot;) needed to reach the next level of this Progression. Jeez, progression is such an overloaded word. | [optional] [default to null]
**next_level_at** | **i32** | The total amount of progression (i.e. \&quot;Experience\&quot;) needed in order to reach the next level. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


