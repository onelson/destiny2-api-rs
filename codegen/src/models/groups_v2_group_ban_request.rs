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
pub struct GroupsV2GroupBanRequest {
  #[serde(rename = "comment")]
  comment: Option<String>,
  #[serde(rename = "length")]
  length: Option<::models::IgnoresIgnoreLength>
}

impl GroupsV2GroupBanRequest {
  pub fn new() -> GroupsV2GroupBanRequest {
    GroupsV2GroupBanRequest {
      comment: None,
      length: None
    }
  }

  pub fn set_comment(&mut self, comment: String) {
    self.comment = Some(comment);
  }

  pub fn with_comment(mut self, comment: String) -> GroupsV2GroupBanRequest {
    self.comment = Some(comment);
    self
  }

  pub fn comment(&self) -> Option<&String> {
    self.comment.as_ref()
  }

  pub fn reset_comment(&mut self) {
    self.comment = None;
  }

  pub fn set_length(&mut self, length: ::models::IgnoresIgnoreLength) {
    self.length = Some(length);
  }

  pub fn with_length(mut self, length: ::models::IgnoresIgnoreLength) -> GroupsV2GroupBanRequest {
    self.length = Some(length);
    self
  }

  pub fn length(&self) -> Option<&::models::IgnoresIgnoreLength> {
    self.length.as_ref()
  }

  pub fn reset_length(&mut self) {
    self.length = None;
  }

}



