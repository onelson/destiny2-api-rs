# DestinyDefinitionsDestinyTalentNodeCategory

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**identifier** | **String** | Mostly just for debug purposes, but if you find it useful you can have it. This is BNet&#39;s manually created identifier for this category. | [optional] [default to null]
**is_lore_driven** | **bool** | If true, we found the localized content in a related DestinyLoreDefinition instead of local BNet localization files. This is mostly for ease of my own future investigations. | [optional] [default to null]
**display_properties** | [***Object**](Object.md) | Will contain at least the \&quot;name\&quot;, which will be the title of the category. We will likely not have description and an icon yet, but I&#39;m going to keep my options open. | [optional] [default to null]
**node_hashes** | **Vec<i32>** | The set of all hash identifiers for Talent Nodes (DestinyTalentNodeDefinition) in this Talent Grid that are part of this Category. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


