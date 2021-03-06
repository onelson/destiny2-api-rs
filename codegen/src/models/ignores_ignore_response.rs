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
pub struct IgnoresIgnoreResponse {
  #[serde(rename = "isIgnored")]
  is_ignored: Option<bool>,
  #[serde(rename = "ignoreFlags")]
  ignore_flags: Option<::models::IgnoresIgnoreStatus>
}

impl IgnoresIgnoreResponse {
  pub fn new() -> IgnoresIgnoreResponse {
    IgnoresIgnoreResponse {
      is_ignored: None,
      ignore_flags: None
    }
  }

  pub fn set_is_ignored(&mut self, is_ignored: bool) {
    self.is_ignored = Some(is_ignored);
  }

  pub fn with_is_ignored(mut self, is_ignored: bool) -> IgnoresIgnoreResponse {
    self.is_ignored = Some(is_ignored);
    self
  }

  pub fn is_ignored(&self) -> Option<&bool> {
    self.is_ignored.as_ref()
  }

  pub fn reset_is_ignored(&mut self) {
    self.is_ignored = None;
  }

  pub fn set_ignore_flags(&mut self, ignore_flags: ::models::IgnoresIgnoreStatus) {
    self.ignore_flags = Some(ignore_flags);
  }

  pub fn with_ignore_flags(mut self, ignore_flags: ::models::IgnoresIgnoreStatus) -> IgnoresIgnoreResponse {
    self.ignore_flags = Some(ignore_flags);
    self
  }

  pub fn ignore_flags(&self) -> Option<&::models::IgnoresIgnoreStatus> {
    self.ignore_flags.as_ref()
  }

  pub fn reset_ignore_flags(&mut self) {
    self.ignore_flags = None;
  }

}



