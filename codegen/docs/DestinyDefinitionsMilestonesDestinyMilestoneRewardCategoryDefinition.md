# DestinyDefinitionsMilestonesDestinyMilestoneRewardCategoryDefinition

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**category_hash** | **i32** | Identifies the reward category. Only guaranteed unique within this specific component! | [optional] [default to null]
**category_identifier** | **String** | The string identifier for the category, if you want to use it for some end. Guaranteed unique within the specific component. | [optional] [default to null]
**display_properties** | [***Object**](Object.md) | Hopefully this is obvious by now. | [optional] [default to null]
**reward_entries** | [**::std::collections::HashMap<String, ::models::DestinyDefinitionsMilestonesDestinyMilestoneRewardEntryDefinition>**](Destiny.Definitions.Milestones.DestinyMilestoneRewardEntryDefinition.md) | If this milestone can provide rewards, this will define the sets of rewards that can be earned, the conditions under which they can be acquired, internal data that we&#39;ll use at runtime to determine whether you&#39;ve already earned or redeemed this set of rewards, and the category that this reward should be placed under. | [optional] [default to null]
**order** | **i32** | If you want to use BNet&#39;s recommended order for rendering categories programmatically, use this value and compare it to other categories to determine the order in which they should be rendered. I don&#39;t feel great about putting this here, I won&#39;t lie. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


