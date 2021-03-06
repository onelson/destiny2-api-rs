/* 
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * OpenAPI spec version: 2.0.0
 * Contact: support@bungie.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DestinyDefinitionsDestinyActivityGuidedBlockDefinition : Guided Game information for this activity.

#[derive(Debug, Serialize, Deserialize)]
pub struct DestinyDefinitionsDestinyActivityGuidedBlockDefinition {
  /// The maximum amount of people that can be in the waiting lobby.
  #[serde(rename = "guidedMaxLobbySize")]
  guided_max_lobby_size: Option<i32>,
  /// The minimum amount of people that can be in the waiting lobby.
  #[serde(rename = "guidedMinLobbySize")]
  guided_min_lobby_size: Option<i32>,
  /// If -1, the guided group cannot be disbanded. Otherwise, take the total # of players in the activity and subtract this number: that is the total # of votes needed for the guided group to disband.
  #[serde(rename = "guidedDisbandCount")]
  guided_disband_count: Option<i32>
}

impl DestinyDefinitionsDestinyActivityGuidedBlockDefinition {
  /// Guided Game information for this activity.
  pub fn new() -> DestinyDefinitionsDestinyActivityGuidedBlockDefinition {
    DestinyDefinitionsDestinyActivityGuidedBlockDefinition {
      guided_max_lobby_size: None,
      guided_min_lobby_size: None,
      guided_disband_count: None
    }
  }

  pub fn set_guided_max_lobby_size(&mut self, guided_max_lobby_size: i32) {
    self.guided_max_lobby_size = Some(guided_max_lobby_size);
  }

  pub fn with_guided_max_lobby_size(mut self, guided_max_lobby_size: i32) -> DestinyDefinitionsDestinyActivityGuidedBlockDefinition {
    self.guided_max_lobby_size = Some(guided_max_lobby_size);
    self
  }

  pub fn guided_max_lobby_size(&self) -> Option<&i32> {
    self.guided_max_lobby_size.as_ref()
  }

  pub fn reset_guided_max_lobby_size(&mut self) {
    self.guided_max_lobby_size = None;
  }

  pub fn set_guided_min_lobby_size(&mut self, guided_min_lobby_size: i32) {
    self.guided_min_lobby_size = Some(guided_min_lobby_size);
  }

  pub fn with_guided_min_lobby_size(mut self, guided_min_lobby_size: i32) -> DestinyDefinitionsDestinyActivityGuidedBlockDefinition {
    self.guided_min_lobby_size = Some(guided_min_lobby_size);
    self
  }

  pub fn guided_min_lobby_size(&self) -> Option<&i32> {
    self.guided_min_lobby_size.as_ref()
  }

  pub fn reset_guided_min_lobby_size(&mut self) {
    self.guided_min_lobby_size = None;
  }

  pub fn set_guided_disband_count(&mut self, guided_disband_count: i32) {
    self.guided_disband_count = Some(guided_disband_count);
  }

  pub fn with_guided_disband_count(mut self, guided_disband_count: i32) -> DestinyDefinitionsDestinyActivityGuidedBlockDefinition {
    self.guided_disband_count = Some(guided_disband_count);
    self
  }

  pub fn guided_disband_count(&self) -> Option<&i32> {
    self.guided_disband_count.as_ref()
  }

  pub fn reset_guided_disband_count(&mut self) {
    self.guided_disband_count = None;
  }

}



