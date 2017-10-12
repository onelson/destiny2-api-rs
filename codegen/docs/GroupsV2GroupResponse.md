# GroupsV2GroupResponse

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**detail** | [***::models::GroupsV2GroupV2**](GroupsV2.GroupV2.md) |  | [optional] [default to null]
**founder** | [***::models::GroupsV2GroupMember**](GroupsV2.GroupMember.md) |  | [optional] [default to null]
**allied_ids** | **Vec<i64>** |  | [optional] [default to null]
**parent_group** | [***::models::GroupsV2GroupV2**](GroupsV2.GroupV2.md) |  | [optional] [default to null]
**alliance_status** | [***::models::GroupsV2GroupAllianceStatus**](GroupsV2.GroupAllianceStatus.md) |  | [optional] [default to null]
**group_join_invite_count** | **i32** |  | [optional] [default to null]
**current_user_member_map** | [**::std::collections::HashMap<String, ::models::GroupsV2GroupMember>**](GroupsV2.GroupMember.md) | This property will be populated if the authenticated user is a member of the group. Note that because of account linking, a user can sometimes be part of a clan more than once. As such, this returns the highest member type available. | [optional] [default to null]
**current_user_potential_member_map** | [**::std::collections::HashMap<String, ::models::GroupsV2GroupPotentialMember>**](GroupsV2.GroupPotentialMember.md) | This property will be populated if the authenticated user is an applicant or has an outstanding invitation to join. Note that because of account linking, a user can sometimes be part of a clan more than once. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


