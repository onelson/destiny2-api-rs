# DestinyDefinitionsDestinyInventoryItemStatDefinition

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**stat_hash** | **i32** | The hash for the DestinyStatDefinition representing this stat. | [optional] [default to null]
**value** | **i32** | This value represents the stat value assuming the minimum possible roll but accounting for any mandatory bonuses that should be applied to the stat on item creation.  In Destiny 1, this was different from the \&quot;minimum\&quot; value because there were certain conditions where an item could be theoretically lower level/value than the initial roll.   In Destiny 2, this is not possible unless Talent Grids begin to be used again for these purposes or some other system change occurs... thus in practice, value and minimum should be the same in Destiny 2. Good riddance. | [optional] [default to null]
**minimum** | **i32** | The minimum possible value for this stat that we think the item can roll. | [optional] [default to null]
**maximum** | **i32** | The maximum possible value for this stat that we think the item can roll. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


