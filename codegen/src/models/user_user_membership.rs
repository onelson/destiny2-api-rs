/* 
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * OpenAPI spec version: 2.0.0
 * Contact: support@bungie.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// UserUserMembership : Very basic info about a user as returned by the Account server.

#[derive(Debug, Serialize, Deserialize)]
pub struct UserUserMembership {
  /// Type of the membership.
  #[serde(rename = "membershipType")]
  membership_type: Option<Object>,
  /// Membership ID as they user is known in the Accounts service
  #[serde(rename = "membershipId")]
  membership_id: Option<i64>,
  /// Display Name the player has chosen for themselves. The display name is optional when the data type is used as input to a platform API.
  #[serde(rename = "displayName")]
  display_name: Option<String>
}

impl UserUserMembership {
  /// Very basic info about a user as returned by the Account server.
  pub fn new() -> UserUserMembership {
    UserUserMembership {
      membership_type: None,
      membership_id: None,
      display_name: None
    }
  }

  pub fn set_membership_type(&mut self, membership_type: Object) {
    self.membership_type = Some(membership_type);
  }

  pub fn with_membership_type(mut self, membership_type: Object) -> UserUserMembership {
    self.membership_type = Some(membership_type);
    self
  }

  pub fn membership_type(&self) -> Option<&Object> {
    self.membership_type.as_ref()
  }

  pub fn reset_membership_type(&mut self) {
    self.membership_type = None;
  }

  pub fn set_membership_id(&mut self, membership_id: i64) {
    self.membership_id = Some(membership_id);
  }

  pub fn with_membership_id(mut self, membership_id: i64) -> UserUserMembership {
    self.membership_id = Some(membership_id);
    self
  }

  pub fn membership_id(&self) -> Option<&i64> {
    self.membership_id.as_ref()
  }

  pub fn reset_membership_id(&mut self) {
    self.membership_id = None;
  }

  pub fn set_display_name(&mut self, display_name: String) {
    self.display_name = Some(display_name);
  }

  pub fn with_display_name(mut self, display_name: String) -> UserUserMembership {
    self.display_name = Some(display_name);
    self
  }

  pub fn display_name(&self) -> Option<&String> {
    self.display_name.as_ref()
  }

  pub fn reset_display_name(&mut self) {
    self.display_name = None;
  }

}



