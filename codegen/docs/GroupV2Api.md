# \GroupV2Api

All URIs are relative to *https://www.bungie.net/Platform*

Method | HTTP request | Description
------------- | ------------- | -------------
[**group_v2_abdicate_foundership**](GroupV2Api.md#group_v2_abdicate_foundership) | **Post** /GroupV2/{groupId}/Admin/AbdicateFoundership/{membershipType}/{founderIdNew}/ | 
[**group_v2_add_optional_conversation**](GroupV2Api.md#group_v2_add_optional_conversation) | **Post** /GroupV2/{groupId}/OptionalConversations/Add/ | 
[**group_v2_approve_all_pending**](GroupV2Api.md#group_v2_approve_all_pending) | **Post** /GroupV2/{groupId}/Members/ApproveAll/ | 
[**group_v2_approve_pending_for_list**](GroupV2Api.md#group_v2_approve_pending_for_list) | **Post** /GroupV2/{groupId}/Members/ApproveList/ | 
[**group_v2_ban_member**](GroupV2Api.md#group_v2_ban_member) | **Post** /GroupV2/{groupId}/Members/{membershipType}/{membershipId}/Ban/ | 
[**group_v2_create_group**](GroupV2Api.md#group_v2_create_group) | **Post** /GroupV2/Create/ | 
[**group_v2_deny_all_pending**](GroupV2Api.md#group_v2_deny_all_pending) | **Post** /GroupV2/{groupId}/Members/DenyAll/ | 
[**group_v2_deny_pending_for_list**](GroupV2Api.md#group_v2_deny_pending_for_list) | **Post** /GroupV2/{groupId}/Members/DenyList/ | 
[**group_v2_edit_clan_banner**](GroupV2Api.md#group_v2_edit_clan_banner) | **Post** /GroupV2/{groupId}/EditClanBanner/ | 
[**group_v2_edit_founder_options**](GroupV2Api.md#group_v2_edit_founder_options) | **Post** /GroupV2/{groupId}/EditFounderOptions/ | 
[**group_v2_edit_group**](GroupV2Api.md#group_v2_edit_group) | **Post** /GroupV2/{groupId}/Edit/ | 
[**group_v2_edit_group_membership**](GroupV2Api.md#group_v2_edit_group_membership) | **Post** /GroupV2/{groupId}/Members/{membershipType}/{membershipId}/SetMembershipType/{memberType}/ | 
[**group_v2_edit_optional_conversation**](GroupV2Api.md#group_v2_edit_optional_conversation) | **Post** /GroupV2/{groupId}/OptionalConversations/Edit/{conversationId}/ | 
[**group_v2_get_admins_and_founder_of_group**](GroupV2Api.md#group_v2_get_admins_and_founder_of_group) | **Get** /GroupV2/{groupId}/AdminsAndFounder/ | 
[**group_v2_get_available_avatars**](GroupV2Api.md#group_v2_get_available_avatars) | **Get** /GroupV2/GetAvailableAvatars/ | 
[**group_v2_get_available_themes**](GroupV2Api.md#group_v2_get_available_themes) | **Get** /GroupV2/GetAvailableThemes/ | 
[**group_v2_get_banned_members_of_group**](GroupV2Api.md#group_v2_get_banned_members_of_group) | **Get** /GroupV2/{groupId}/Banned/ | 
[**group_v2_get_group**](GroupV2Api.md#group_v2_get_group) | **Get** /GroupV2/{groupId}/ | 
[**group_v2_get_group_by_name**](GroupV2Api.md#group_v2_get_group_by_name) | **Get** /GroupV2/Name/{groupName}/{groupType}/ | 
[**group_v2_get_group_optional_conversations**](GroupV2Api.md#group_v2_get_group_optional_conversations) | **Get** /GroupV2/{groupId}/OptionalConversations/ | 
[**group_v2_get_groups_for_member**](GroupV2Api.md#group_v2_get_groups_for_member) | **Get** /GroupV2/User/{membershipType}/{membershipId}/{filter}/{groupType}/ | 
[**group_v2_get_invited_individuals**](GroupV2Api.md#group_v2_get_invited_individuals) | **Get** /GroupV2/{groupId}/Members/InvitedIndividuals/ | 
[**group_v2_get_members_of_group**](GroupV2Api.md#group_v2_get_members_of_group) | **Get** /GroupV2/{groupId}/Members/ | 
[**group_v2_get_pending_memberships**](GroupV2Api.md#group_v2_get_pending_memberships) | **Get** /GroupV2/{groupId}/Members/Pending/ | 
[**group_v2_get_potential_groups_for_member**](GroupV2Api.md#group_v2_get_potential_groups_for_member) | **Get** /GroupV2/User/Potential/{membershipType}/{membershipId}/{filter}/{groupType}/ | 
[**group_v2_get_recommended_groups**](GroupV2Api.md#group_v2_get_recommended_groups) | **Post** /GroupV2/Recommended/ | 
[**group_v2_get_user_clan_invite_setting**](GroupV2Api.md#group_v2_get_user_clan_invite_setting) | **Get** /GroupV2/GetUserClanInviteSetting/{mType}/ | 
[**group_v2_group_search**](GroupV2Api.md#group_v2_group_search) | **Post** /GroupV2/Search/ | 
[**group_v2_individual_group_invite**](GroupV2Api.md#group_v2_individual_group_invite) | **Post** /GroupV2/{groupId}/Members/IndividualInvite/{membershipType}/{membershipId}/ | 
[**group_v2_individual_group_invite_cancel**](GroupV2Api.md#group_v2_individual_group_invite_cancel) | **Post** /GroupV2/{groupId}/Members/IndividualInviteCancel/{membershipType}/{membershipId}/ | 
[**group_v2_kick_member**](GroupV2Api.md#group_v2_kick_member) | **Post** /GroupV2/{groupId}/Members/{membershipType}/{membershipId}/Kick/ | 
[**group_v2_request_group_membership**](GroupV2Api.md#group_v2_request_group_membership) | **Post** /GroupV2/{groupId}/Members/Apply/{membershipType}/ | 
[**group_v2_rescind_group_membership**](GroupV2Api.md#group_v2_rescind_group_membership) | **Post** /GroupV2/{groupId}/Members/Rescind/{membershipType}/ | 
[**group_v2_set_user_clan_invite_setting**](GroupV2Api.md#group_v2_set_user_clan_invite_setting) | **Post** /GroupV2/SetUserClanInviteSetting/{mType}/{allowInvites}/ | 
[**group_v2_unban_member**](GroupV2Api.md#group_v2_unban_member) | **Post** /GroupV2/{groupId}/Members/{membershipType}/{membershipId}/Unban/ | 


# **group_v2_abdicate_foundership**
> ::models::InlineResponse20014 group_v2_abdicate_foundership(founder_id_new, group_id, membership_type)


An administrative method to allow the founder of a group or clan to give up their position to another admin permanently.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **founder_id_new** | **i64**| The new founder for this group. Must already be a group admin. | 
  **group_id** | **i64**| The target group id. | 
  **membership_type** | **i32**| Membership type of the provided founderIdNew. | 

### Return type

[**::models::InlineResponse20014**](inline_response_200_14.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **group_v2_add_optional_conversation**
> ::models::InlineResponse2007 group_v2_add_optional_conversation(ctx, group_id)


Add a new optional conversation/chat channel. Requires admin permissions to the group.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **group_id** | **i64**| Group ID of the group to edit. | 

### Return type

[**::models::InlineResponse2007**](inline_response_200_7.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **group_v2_approve_all_pending**
> ::models::InlineResponse20025 group_v2_approve_all_pending(ctx, group_id)


Approve all of the pending users for the given group.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **group_id** | **i64**| ID of the group. | 

### Return type

[**::models::InlineResponse20025**](inline_response_200_25.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **group_v2_approve_pending_for_list**
> ::models::InlineResponse20025 group_v2_approve_pending_for_list(ctx, group_id)


Approve all of the pending users for the given group.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **group_id** | **i64**| ID of the group. | 

### Return type

[**::models::InlineResponse20025**](inline_response_200_25.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **group_v2_ban_member**
> ::models::InlineResponse20015 group_v2_ban_member(ctx, group_id, membership_id, membership_type)


Bans the requested member from the requested group for the specified period of time.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **group_id** | **i64**| Group ID that has the member to ban. | 
  **membership_id** | **i64**| Membership ID of the member to ban from the group. | 
  **membership_type** | **i32**| Membership type of the provided membership ID. | 

### Return type

[**::models::InlineResponse20015**](inline_response_200_15.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **group_v2_create_group**
> ::models::InlineResponse20019 group_v2_create_group(ctx, )


Create a new group.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::InlineResponse20019**](inline_response_200_19.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **group_v2_deny_all_pending**
> ::models::InlineResponse20025 group_v2_deny_all_pending(ctx, group_id)


Deny all of the pending users for the given group.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **group_id** | **i64**| ID of the group. | 

### Return type

[**::models::InlineResponse20025**](inline_response_200_25.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **group_v2_deny_pending_for_list**
> ::models::InlineResponse20025 group_v2_deny_pending_for_list(ctx, group_id)


Deny all of the pending users for the given group that match the passed-in .

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **group_id** | **i64**| ID of the group. | 

### Return type

[**::models::InlineResponse20025**](inline_response_200_25.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **group_v2_edit_clan_banner**
> ::models::InlineResponse20015 group_v2_edit_clan_banner(ctx, group_id)


Edit an existing group's clan banner. You must have suitable permissions in the group to perform this operation. All fields are required.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **group_id** | **i64**| Group ID of the group to edit. | 

### Return type

[**::models::InlineResponse20015**](inline_response_200_15.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **group_v2_edit_founder_options**
> ::models::InlineResponse20015 group_v2_edit_founder_options(ctx, group_id)


Edit group options only available to a founder. You must have suitable permissions in the group to perform this operation.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **group_id** | **i64**| Group ID of the group to edit. | 

### Return type

[**::models::InlineResponse20015**](inline_response_200_15.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **group_v2_edit_group**
> ::models::InlineResponse20015 group_v2_edit_group(ctx, group_id)


Edit an existing group. You must have suitable permissions in the group to perform this operation. This latest revision will only edit the fields you pass in - pass null for properties you want to leave unaltered.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **group_id** | **i64**| Group ID of the group to edit. | 

### Return type

[**::models::InlineResponse20015**](inline_response_200_15.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **group_v2_edit_group_membership**
> ::models::InlineResponse20015 group_v2_edit_group_membership(ctx, group_id, membership_id, membership_type, member_type)


Edit the membership type of a given member. You must have suitable permissions in the group to perform this operation.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **group_id** | **i64**| ID of the group to which the member belongs. | 
  **membership_id** | **i64**| Membership ID to modify. | 
  **membership_type** | **i32**| Membership type of the provide membership ID. | 
  **member_type** | **i32**| New membertype for the specified member. | 

### Return type

[**::models::InlineResponse20015**](inline_response_200_15.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **group_v2_edit_optional_conversation**
> ::models::InlineResponse2007 group_v2_edit_optional_conversation(ctx, conversation_id, group_id)


Edit the settings of an optional conversation/chat channel. Requires admin permissions to the group.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **conversation_id** | **i64**| Conversation Id of the channel being edited. | 
  **group_id** | **i64**| Group ID of the group to edit. | 

### Return type

[**::models::InlineResponse2007**](inline_response_200_7.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **group_v2_get_admins_and_founder_of_group**
> ::models::InlineResponse20020 group_v2_get_admins_and_founder_of_group(currentpage, group_id)


Get the list of members in a given group who are of admin level or higher.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **currentpage** | **i32**| Page number (starting with 1). Each page has a fixed size of 50 items per page. | 
  **group_id** | **i64**| The ID of the group. | 

### Return type

[**::models::InlineResponse20020**](inline_response_200_20.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **group_v2_get_available_avatars**
> ::models::InlineResponse20012 group_v2_get_available_avatars()


Returns a list of all available group avatars for the signed-in user.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::InlineResponse20012**](inline_response_200_12.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **group_v2_get_available_themes**
> ::models::InlineResponse20013 group_v2_get_available_themes()


Returns a list of all available group themes.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::InlineResponse20013**](inline_response_200_13.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **group_v2_get_banned_members_of_group**
> ::models::InlineResponse20022 group_v2_get_banned_members_of_group(ctx, currentpage, group_id)


Get the list of banned members in a given group. Only accessible to group Admins and above. Not applicable to all groups. Check group features.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **currentpage** | **i32**| Page number (starting with 1). Each page has a fixed size of 50 entries. | 
  **group_id** | **i64**| Group ID whose banned members you are fetching | 

### Return type

[**::models::InlineResponse20022**](inline_response_200_22.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **group_v2_get_group**
> ::models::InlineResponse20017 group_v2_get_group(group_id)


Get information about a specific group of the given ID.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **group_id** | **i64**| Requested group&#39;s id. | 

### Return type

[**::models::InlineResponse20017**](inline_response_200_17.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **group_v2_get_group_by_name**
> ::models::InlineResponse20017 group_v2_get_group_by_name(group_name, group_type)


Get information about a specific group with the given name and type.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **group_name** | **String**| Exact name of the group to find. | 
  **group_type** | **i32**| Type of group to find. | 

### Return type

[**::models::InlineResponse20017**](inline_response_200_17.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **group_v2_get_group_optional_conversations**
> ::models::InlineResponse20018 group_v2_get_group_optional_conversations(group_id)


Gets a list of available optional conversation channels and their settings.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **group_id** | **i64**| Requested group&#39;s id. | 

### Return type

[**::models::InlineResponse20018**](inline_response_200_18.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **group_v2_get_groups_for_member**
> ::models::InlineResponse20026 group_v2_get_groups_for_member(filter, group_type, membership_id, membership_type)


Get information about the groups that a given member has joined.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **filter** | **i32**| Filter apply to list of joined groups. | 
  **group_type** | **i32**| Type of group the supplied member founded. | 
  **membership_id** | **i64**| Membership ID to for which to find founded groups. | 
  **membership_type** | **i32**| Membership type of the supplied membership ID. | 

### Return type

[**::models::InlineResponse20026**](inline_response_200_26.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **group_v2_get_invited_individuals**
> ::models::InlineResponse20024 group_v2_get_invited_individuals(ctx, currentpage, group_id)


Get the list of users who have been invited into the group.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **currentpage** | **i32**| Page number (starting with 1). Each page has a fixed size of 50 items per page. | 
  **group_id** | **i64**| ID of the group. | 

### Return type

[**::models::InlineResponse20024**](inline_response_200_24.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **group_v2_get_members_of_group**
> ::models::InlineResponse20020 group_v2_get_members_of_group(currentpage, group_id, optional)


Get the list of members in a given group.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **currentpage** | **i32**| Page number (starting with 1). Each page has a fixed size of 50 items per page. | 
  **group_id** | **i64**| The ID of the group. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **currentpage** | **i32**| Page number (starting with 1). Each page has a fixed size of 50 items per page. | 
 **group_id** | **i64**| The ID of the group. | 
 **member_type** | **i32**| Filter out other member types. Use None for all members. | 
 **name_search** | **String**| The name fragment upon which a search should be executed for members with matching display or unique names. | 

### Return type

[**::models::InlineResponse20020**](inline_response_200_20.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **group_v2_get_pending_memberships**
> ::models::InlineResponse20024 group_v2_get_pending_memberships(ctx, currentpage, group_id)


Get the list of users who are awaiting a decision on their application to join a given group. Modified to include application info.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **currentpage** | **i32**| Page number (starting with 1). Each page has a fixed size of 50 items per page. | 
  **group_id** | **i64**| ID of the group. | 

### Return type

[**::models::InlineResponse20024**](inline_response_200_24.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **group_v2_get_potential_groups_for_member**
> ::models::InlineResponse20027 group_v2_get_potential_groups_for_member(filter, group_type, membership_id, membership_type)


Get information about the groups that a given member has applied to or been invited to.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **filter** | **i32**| Filter apply to list of potential joined groups. | 
  **group_type** | **i32**| Type of group the supplied member applied. | 
  **membership_id** | **i64**| Membership ID to for which to find applied groups. | 
  **membership_type** | **i32**| Membership type of the supplied membership ID. | 

### Return type

[**::models::InlineResponse20027**](inline_response_200_27.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **group_v2_get_recommended_groups**
> ::models::InlineResponse20016 group_v2_get_recommended_groups(ctx, )


Gets groups recommended for you based on the groups to whom those you follow belong.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::InlineResponse20016**](inline_response_200_16.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **group_v2_get_user_clan_invite_setting**
> ::models::InlineResponse20014 group_v2_get_user_clan_invite_setting(ctx, m_type)


Gets the state of the user's clan invite preferences for a particular membership type - true if they wish to be invited to clans, false otherwise.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **m_type** | **i32**| The Destiny membership type of the account we wish to access settings. | 

### Return type

[**::models::InlineResponse20014**](inline_response_200_14.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **group_v2_group_search**
> ::models::InlineResponse20016 group_v2_group_search()


Search for Groups.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::InlineResponse20016**](inline_response_200_16.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **group_v2_individual_group_invite**
> ::models::InlineResponse20023 group_v2_individual_group_invite(ctx, group_id, membership_id, membership_type)


Invite a user to join this group.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **group_id** | **i64**| ID of the group you would like to join. | 
  **membership_id** | **i64**| Membership id of the account being invited. | 
  **membership_type** | **i32**| MembershipType of the account being invited. | 

### Return type

[**::models::InlineResponse20023**](inline_response_200_23.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **group_v2_individual_group_invite_cancel**
> ::models::InlineResponse20023 group_v2_individual_group_invite_cancel(ctx, group_id, membership_id, membership_type)


Cancels a pending invitation to join a group.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **group_id** | **i64**| ID of the group you would like to join. | 
  **membership_id** | **i64**| Membership id of the account being cancelled. | 
  **membership_type** | **i32**| MembershipType of the account being cancelled. | 

### Return type

[**::models::InlineResponse20023**](inline_response_200_23.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **group_v2_kick_member**
> ::models::InlineResponse20021 group_v2_kick_member(ctx, group_id, membership_id, membership_type)


Kick a member from the given group, forcing them to reapply if they wish to re-join the group. You must have suitable permissions in the group to perform this operation.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **group_id** | **i64**| Group ID to kick the user from. | 
  **membership_id** | **i64**| Membership ID to kick. | 
  **membership_type** | **i32**| Membership type of the provided membership ID. | 

### Return type

[**::models::InlineResponse20021**](inline_response_200_21.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **group_v2_request_group_membership**
> ::models::InlineResponse20023 group_v2_request_group_membership(ctx, group_id, membership_type)


Request permission to join the given group.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **group_id** | **i64**| ID of the group you would like to join. | 
  **membership_type** | **i32**| MembershipType of the account to use when joining. | 

### Return type

[**::models::InlineResponse20023**](inline_response_200_23.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **group_v2_rescind_group_membership**
> ::models::InlineResponse20021 group_v2_rescind_group_membership(ctx, group_id, membership_type)


Rescind your application to join the given group or leave the group if you are already a member..

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **group_id** | **i64**| ID of the group. | 
  **membership_type** | **i32**| MembershipType of the account to leave. | 

### Return type

[**::models::InlineResponse20021**](inline_response_200_21.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **group_v2_set_user_clan_invite_setting**
> ::models::InlineResponse20015 group_v2_set_user_clan_invite_setting(ctx, allow_invites, m_type)


Sets the state of the user's clan invite preferences - true if they wish to be invited to clans, false otherwise.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **allow_invites** | **bool**| True to allow invites of this user to clans, false otherwise. | 
  **m_type** | **i32**| The Destiny membership type of linked account we are manipulating. | 

### Return type

[**::models::InlineResponse20015**](inline_response_200_15.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **group_v2_unban_member**
> ::models::InlineResponse20015 group_v2_unban_member(ctx, group_id, membership_id, membership_type)


Unbans the requested member, allowing them to re-apply for membership.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **group_id** | **i64**|  | 
  **membership_id** | **i64**| Membership ID of the member to unban from the group | 
  **membership_type** | **i32**| Membership type of the provided membership ID. | 

### Return type

[**::models::InlineResponse20015**](inline_response_200_15.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

