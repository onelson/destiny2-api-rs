/* 
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * OpenAPI spec version: 2.0.0
 * Contact: support@bungie.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DestinyDefinitionsDestinyActivityUnlockStringDefinition : Represents a status string that could be conditionally displayed about an activity. Note that externally, you can only see the strings themselves. Internally we combine this information with server state to determine which strings should be shown.

#[derive(Debug, Serialize, Deserialize)]
pub struct DestinyDefinitionsDestinyActivityUnlockStringDefinition {
  /// The string to be displayed if the conditions are met.
  #[serde(rename = "displayString")]
  display_string: Option<String>
}

impl DestinyDefinitionsDestinyActivityUnlockStringDefinition {
  /// Represents a status string that could be conditionally displayed about an activity. Note that externally, you can only see the strings themselves. Internally we combine this information with server state to determine which strings should be shown.
  pub fn new() -> DestinyDefinitionsDestinyActivityUnlockStringDefinition {
    DestinyDefinitionsDestinyActivityUnlockStringDefinition {
      display_string: None
    }
  }

  pub fn set_display_string(&mut self, display_string: String) {
    self.display_string = Some(display_string);
  }

  pub fn with_display_string(mut self, display_string: String) -> DestinyDefinitionsDestinyActivityUnlockStringDefinition {
    self.display_string = Some(display_string);
    self
  }

  pub fn display_string(&self) -> Option<&String> {
    self.display_string.as_ref()
  }

  pub fn reset_display_string(&mut self) {
    self.display_string = None;
  }

}


