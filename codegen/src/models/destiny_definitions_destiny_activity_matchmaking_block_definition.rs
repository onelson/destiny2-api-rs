/* 
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * OpenAPI spec version: 2.0.0
 * Contact: support@bungie.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DestinyDefinitionsDestinyActivityMatchmakingBlockDefinition : Information about matchmaking and party size for the activity.

#[derive(Debug, Serialize, Deserialize)]
pub struct DestinyDefinitionsDestinyActivityMatchmakingBlockDefinition {
  /// If TRUE, the activity is matchmade. Otherwise, it requires explicit forming of a party.
  #[serde(rename = "isMatchmade")]
  is_matchmade: Option<bool>,
  /// The minimum # of people in the fireteam for the activity to launch.
  #[serde(rename = "minParty")]
  min_party: Option<i32>,
  /// The maximum # of people allowed in a Fireteam.
  #[serde(rename = "maxParty")]
  max_party: Option<i32>,
  /// The maximum # of people allowed across all teams in the activity.
  #[serde(rename = "maxPlayers")]
  max_players: Option<i32>,
  /// If true, you have to Solemnly Swear to be up to Nothing But Good(tm) to play.
  #[serde(rename = "requiresGuardianOath")]
  requires_guardian_oath: Option<bool>
}

impl DestinyDefinitionsDestinyActivityMatchmakingBlockDefinition {
  /// Information about matchmaking and party size for the activity.
  pub fn new() -> DestinyDefinitionsDestinyActivityMatchmakingBlockDefinition {
    DestinyDefinitionsDestinyActivityMatchmakingBlockDefinition {
      is_matchmade: None,
      min_party: None,
      max_party: None,
      max_players: None,
      requires_guardian_oath: None
    }
  }

  pub fn set_is_matchmade(&mut self, is_matchmade: bool) {
    self.is_matchmade = Some(is_matchmade);
  }

  pub fn with_is_matchmade(mut self, is_matchmade: bool) -> DestinyDefinitionsDestinyActivityMatchmakingBlockDefinition {
    self.is_matchmade = Some(is_matchmade);
    self
  }

  pub fn is_matchmade(&self) -> Option<&bool> {
    self.is_matchmade.as_ref()
  }

  pub fn reset_is_matchmade(&mut self) {
    self.is_matchmade = None;
  }

  pub fn set_min_party(&mut self, min_party: i32) {
    self.min_party = Some(min_party);
  }

  pub fn with_min_party(mut self, min_party: i32) -> DestinyDefinitionsDestinyActivityMatchmakingBlockDefinition {
    self.min_party = Some(min_party);
    self
  }

  pub fn min_party(&self) -> Option<&i32> {
    self.min_party.as_ref()
  }

  pub fn reset_min_party(&mut self) {
    self.min_party = None;
  }

  pub fn set_max_party(&mut self, max_party: i32) {
    self.max_party = Some(max_party);
  }

  pub fn with_max_party(mut self, max_party: i32) -> DestinyDefinitionsDestinyActivityMatchmakingBlockDefinition {
    self.max_party = Some(max_party);
    self
  }

  pub fn max_party(&self) -> Option<&i32> {
    self.max_party.as_ref()
  }

  pub fn reset_max_party(&mut self) {
    self.max_party = None;
  }

  pub fn set_max_players(&mut self, max_players: i32) {
    self.max_players = Some(max_players);
  }

  pub fn with_max_players(mut self, max_players: i32) -> DestinyDefinitionsDestinyActivityMatchmakingBlockDefinition {
    self.max_players = Some(max_players);
    self
  }

  pub fn max_players(&self) -> Option<&i32> {
    self.max_players.as_ref()
  }

  pub fn reset_max_players(&mut self) {
    self.max_players = None;
  }

  pub fn set_requires_guardian_oath(&mut self, requires_guardian_oath: bool) {
    self.requires_guardian_oath = Some(requires_guardian_oath);
  }

  pub fn with_requires_guardian_oath(mut self, requires_guardian_oath: bool) -> DestinyDefinitionsDestinyActivityMatchmakingBlockDefinition {
    self.requires_guardian_oath = Some(requires_guardian_oath);
    self
  }

  pub fn requires_guardian_oath(&self) -> Option<&bool> {
    self.requires_guardian_oath.as_ref()
  }

  pub fn reset_requires_guardian_oath(&mut self) {
    self.requires_guardian_oath = None;
  }

}



