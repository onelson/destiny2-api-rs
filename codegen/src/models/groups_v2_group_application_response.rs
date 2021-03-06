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
pub struct GroupsV2GroupApplicationResponse {
  #[serde(rename = "resolution")]
  resolution: Option<::models::GroupsV2GroupApplicationResolveState>
}

impl GroupsV2GroupApplicationResponse {
  pub fn new() -> GroupsV2GroupApplicationResponse {
    GroupsV2GroupApplicationResponse {
      resolution: None
    }
  }

  pub fn set_resolution(&mut self, resolution: ::models::GroupsV2GroupApplicationResolveState) {
    self.resolution = Some(resolution);
  }

  pub fn with_resolution(mut self, resolution: ::models::GroupsV2GroupApplicationResolveState) -> GroupsV2GroupApplicationResponse {
    self.resolution = Some(resolution);
    self
  }

  pub fn resolution(&self) -> Option<&::models::GroupsV2GroupApplicationResolveState> {
    self.resolution.as_ref()
  }

  pub fn reset_resolution(&mut self) {
    self.resolution = None;
  }

}



