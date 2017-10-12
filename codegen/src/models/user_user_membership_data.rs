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
pub struct UserUserMembershipData {
  /// this allows you to see destiny memberships that are visible and linked to this account (regardless of whether or not they have characters on the world server)
  #[serde(rename = "destinyMemberships")]
  destiny_memberships: Option<Vec<::models::UserUserInfoCard>>,
  #[serde(rename = "bungieNetUser")]
  bungie_net_user: Option<::models::UserGeneralUser>
}

impl UserUserMembershipData {
  pub fn new() -> UserUserMembershipData {
    UserUserMembershipData {
      destiny_memberships: None,
      bungie_net_user: None
    }
  }

  pub fn set_destiny_memberships(&mut self, destiny_memberships: Vec<::models::UserUserInfoCard>) {
    self.destiny_memberships = Some(destiny_memberships);
  }

  pub fn with_destiny_memberships(mut self, destiny_memberships: Vec<::models::UserUserInfoCard>) -> UserUserMembershipData {
    self.destiny_memberships = Some(destiny_memberships);
    self
  }

  pub fn destiny_memberships(&self) -> Option<&Vec<::models::UserUserInfoCard>> {
    self.destiny_memberships.as_ref()
  }

  pub fn reset_destiny_memberships(&mut self) {
    self.destiny_memberships = None;
  }

  pub fn set_bungie_net_user(&mut self, bungie_net_user: ::models::UserGeneralUser) {
    self.bungie_net_user = Some(bungie_net_user);
  }

  pub fn with_bungie_net_user(mut self, bungie_net_user: ::models::UserGeneralUser) -> UserUserMembershipData {
    self.bungie_net_user = Some(bungie_net_user);
    self
  }

  pub fn bungie_net_user(&self) -> Option<&::models::UserGeneralUser> {
    self.bungie_net_user.as_ref()
  }

  pub fn reset_bungie_net_user(&mut self) {
    self.bungie_net_user = None;
  }

}



