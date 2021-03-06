/* 
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * OpenAPI spec version: 2.0.0
 * Contact: support@bungie.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DestinyDefinitionsDestinyObjectiveStatEntryDefinition : Defines the conditions under which stat modifications will be applied to a Character while participating in an objective.

#[derive(Debug, Serialize, Deserialize)]
pub struct DestinyDefinitionsDestinyObjectiveStatEntryDefinition {
  /// The stat being modified, and the value used.
  #[serde(rename = "stat")]
  stat: Option<Object>,
  /// Whether it will be applied as long as the objective is active, when it's completed, or until it's completed.
  #[serde(rename = "style")]
  style: Option<Object>
}

impl DestinyDefinitionsDestinyObjectiveStatEntryDefinition {
  /// Defines the conditions under which stat modifications will be applied to a Character while participating in an objective.
  pub fn new() -> DestinyDefinitionsDestinyObjectiveStatEntryDefinition {
    DestinyDefinitionsDestinyObjectiveStatEntryDefinition {
      stat: None,
      style: None
    }
  }

  pub fn set_stat(&mut self, stat: Object) {
    self.stat = Some(stat);
  }

  pub fn with_stat(mut self, stat: Object) -> DestinyDefinitionsDestinyObjectiveStatEntryDefinition {
    self.stat = Some(stat);
    self
  }

  pub fn stat(&self) -> Option<&Object> {
    self.stat.as_ref()
  }

  pub fn reset_stat(&mut self) {
    self.stat = None;
  }

  pub fn set_style(&mut self, style: Object) {
    self.style = Some(style);
  }

  pub fn with_style(mut self, style: Object) -> DestinyDefinitionsDestinyObjectiveStatEntryDefinition {
    self.style = Some(style);
    self
  }

  pub fn style(&self) -> Option<&Object> {
    self.style.as_ref()
  }

  pub fn reset_style(&mut self) {
    self.style = None;
  }

}



