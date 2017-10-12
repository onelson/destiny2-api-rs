# DestinyDefinitionsDestinyProgressionStepDefinition

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**step_name** | **String** | Very rarely, Progressions will have localized text describing the Level of the progression. This will be that localized text, if it exists. Otherwise, the standard appears to be to simply show the level numerically. | [optional] [default to null]
**display_effect_type** | [***Object**](Object.md) | This appears to be, when you \&quot;level up\&quot;, whether a visual effect will display and on what entity. See DestinyProgressionStepDisplayEffect for slightly more info. | [optional] [default to null]
**progress_total** | **i32** | The total amount of progression points/\&quot;experience\&quot; you will need to initially reach this step. If this is the last step and the progression is repeating indefinitely (DestinyProgressionDefinition.repeatLastStep), this will also be the progress needed to level it up further by repeating this step again. | [optional] [default to null]
**reward_items** | [**Vec<::models::DestinyDestinyItemQuantity>**](Destiny.DestinyItemQuantity.md) | A listing of items rewarded as a result of reaching this level. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


