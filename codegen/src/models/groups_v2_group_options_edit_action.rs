/* 
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * OpenAPI spec version: 2.0.0
 * Contact: support@bungie.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */


#[derive(Debug, Serialize, Deserialize)]
pub struct GroupsV2GroupOptionsEditAction {
  /// Minimum Member Level allowed to invite new members to group  Always Allowed: Founder, Acting Founder  True means admins have this power, false means they don't  Default is false for clans, true for groups.
  #[serde(rename = "InvitePermissionOverride")]
  invite_permission_override: Option<bool>,
  /// Minimum Member Level allowed to update group culture  Always Allowed: Founder, Acting Founder  True means admins have this power, false means they don't  Default is false for clans, true for groups.
  #[serde(rename = "UpdateCulturePermissionOverride")]
  update_culture_permission_override: Option<bool>,
  /// Minimum Member Level allowed to host guided games  Always Allowed: Founder, Acting Founder, Admin  Allowed Overrides: None, Member, Beginner  Default is Member for clans, None for groups, although this means nothing for groups.
  #[serde(rename = "HostGuidedGamePermissionOverride")]
  host_guided_game_permission_override: Option<i32>,
  /// Minimum Member Level allowed to update banner  Always Allowed: Founder, Acting Founder  True means admins have this power, false means they don't  Default is false for clans, true for groups.
  #[serde(rename = "UpdateBannerPermissionOverride")]
  update_banner_permission_override: Option<bool>,
  /// Level to join a member at when accepting an invite, application, or joining an open clan  Default is Beginner.
  #[serde(rename = "JoinLevel")]
  join_level: Option<i32>
}

impl GroupsV2GroupOptionsEditAction {
  pub fn new() -> GroupsV2GroupOptionsEditAction {
    GroupsV2GroupOptionsEditAction {
      invite_permission_override: None,
      update_culture_permission_override: None,
      host_guided_game_permission_override: None,
      update_banner_permission_override: None,
      join_level: None
    }
  }

  pub fn set_invite_permission_override(&mut self, invite_permission_override: bool) {
    self.invite_permission_override = Some(invite_permission_override);
  }

  pub fn with_invite_permission_override(mut self, invite_permission_override: bool) -> GroupsV2GroupOptionsEditAction {
    self.invite_permission_override = Some(invite_permission_override);
    self
  }

  pub fn invite_permission_override(&self) -> Option<&bool> {
    self.invite_permission_override.as_ref()
  }

  pub fn reset_invite_permission_override(&mut self) {
    self.invite_permission_override = None;
  }

  pub fn set_update_culture_permission_override(&mut self, update_culture_permission_override: bool) {
    self.update_culture_permission_override = Some(update_culture_permission_override);
  }

  pub fn with_update_culture_permission_override(mut self, update_culture_permission_override: bool) -> GroupsV2GroupOptionsEditAction {
    self.update_culture_permission_override = Some(update_culture_permission_override);
    self
  }

  pub fn update_culture_permission_override(&self) -> Option<&bool> {
    self.update_culture_permission_override.as_ref()
  }

  pub fn reset_update_culture_permission_override(&mut self) {
    self.update_culture_permission_override = None;
  }

  pub fn set_host_guided_game_permission_override(&mut self, host_guided_game_permission_override: i32) {
    self.host_guided_game_permission_override = Some(host_guided_game_permission_override);
  }

  pub fn with_host_guided_game_permission_override(mut self, host_guided_game_permission_override: i32) -> GroupsV2GroupOptionsEditAction {
    self.host_guided_game_permission_override = Some(host_guided_game_permission_override);
    self
  }

  pub fn host_guided_game_permission_override(&self) -> Option<&i32> {
    self.host_guided_game_permission_override.as_ref()
  }

  pub fn reset_host_guided_game_permission_override(&mut self) {
    self.host_guided_game_permission_override = None;
  }

  pub fn set_update_banner_permission_override(&mut self, update_banner_permission_override: bool) {
    self.update_banner_permission_override = Some(update_banner_permission_override);
  }

  pub fn with_update_banner_permission_override(mut self, update_banner_permission_override: bool) -> GroupsV2GroupOptionsEditAction {
    self.update_banner_permission_override = Some(update_banner_permission_override);
    self
  }

  pub fn update_banner_permission_override(&self) -> Option<&bool> {
    self.update_banner_permission_override.as_ref()
  }

  pub fn reset_update_banner_permission_override(&mut self) {
    self.update_banner_permission_override = None;
  }

  pub fn set_join_level(&mut self, join_level: i32) {
    self.join_level = Some(join_level);
  }

  pub fn with_join_level(mut self, join_level: i32) -> GroupsV2GroupOptionsEditAction {
    self.join_level = Some(join_level);
    self
  }

  pub fn join_level(&self) -> Option<&i32> {
    self.join_level.as_ref()
  }

  pub fn reset_join_level(&mut self) {
    self.join_level = None;
  }

}



