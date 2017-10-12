# DestinyDefinitionsDestinyLocationReleaseDefinition

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**display_properties** | [***Object**](Object.md) | Sadly, these don&#39;t appear to be populated anymore (ever?) | [optional] [default to null]
**spawn_point** | **i32** | If we had map information, this spawnPoint would be interesting. But sadly, we don&#39;t have that info. | [optional] [default to null]
**destination_hash** | **i32** | The Destination being pointed to by this location. | [optional] [default to null]
**activity_hash** | **i32** | The Activity being pointed to by this location. | [optional] [default to null]
**activity_graph_hash** | **i32** | The Activity Graph being pointed to by this location. | [optional] [default to null]
**activity_graph_node_hash** | **i32** | The Activity Graph Node being pointed to by this location. (Remember that Activity Graph Node hashes are only unique within an Activity Graph: so use the combination to find the node being spoken of) | [optional] [default to null]
**activity_bubble_name** | **i32** | The Activity Bubble within the Destination. Look this up in the DestinyDestinationDefinition&#39;s bubbles and bubbleSettings properties. | [optional] [default to null]
**activity_path_bundle** | **i32** | If we had map information, this would tell us something cool about the path this location wants you to take. I wish we had map information. | [optional] [default to null]
**activity_path_destination** | **i32** | If we had map information, this would tell us about path information related to destination on the map. Sad. Maybe you can do something cool with it. Go to town man. | [optional] [default to null]
**nav_point_type** | [***Object**](Object.md) | The type of Nav Point that this represents. See the enumeration for more info. | [optional] [default to null]
**world_position** | **Vec<i32>** | Looks like it should be the position on the map, but sadly it does not look populated... yet? | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


