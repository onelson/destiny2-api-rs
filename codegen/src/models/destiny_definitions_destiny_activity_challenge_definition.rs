/* 
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * OpenAPI spec version: 2.0.0
 * Contact: support@bungie.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DestinyDefinitionsDestinyActivityChallengeDefinition : Represents a reference to a Challenge, which for now is just an Objective.

#[derive(Debug, Serialize, Deserialize)]
pub struct DestinyDefinitionsDestinyActivityChallengeDefinition {
  /// The hash for the Objective that matches this challenge. Use it to look up the DestinyObjectiveDefinition.
  #[serde(rename = "objectiveHash")]
  objective_hash: Option<i32>
}

impl DestinyDefinitionsDestinyActivityChallengeDefinition {
  /// Represents a reference to a Challenge, which for now is just an Objective.
  pub fn new() -> DestinyDefinitionsDestinyActivityChallengeDefinition {
    DestinyDefinitionsDestinyActivityChallengeDefinition {
      objective_hash: None
    }
  }

  pub fn set_objective_hash(&mut self, objective_hash: i32) {
    self.objective_hash = Some(objective_hash);
  }

  pub fn with_objective_hash(mut self, objective_hash: i32) -> DestinyDefinitionsDestinyActivityChallengeDefinition {
    self.objective_hash = Some(objective_hash);
    self
  }

  pub fn objective_hash(&self) -> Option<&i32> {
    self.objective_hash.as_ref()
  }

  pub fn reset_objective_hash(&mut self) {
    self.objective_hash = None;
  }

}



