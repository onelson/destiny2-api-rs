/* 
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * OpenAPI spec version: 2.0.0
 * Contact: support@bungie.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DestinyDefinitionsDestinyActivityGraphListEntryDefinition : Destinations and Activities may have default Activity Graphs that should be shown when you bring up the Director and are playing in either.  This contract defines the graph referred to and the gating for when it is relevant.

#[derive(Debug, Serialize, Deserialize)]
pub struct DestinyDefinitionsDestinyActivityGraphListEntryDefinition {
  /// The hash identifier of the DestinyActivityGraphDefinition that should be shown when opening the director.
  #[serde(rename = "activityGraphHash")]
  activity_graph_hash: Option<i32>
}

impl DestinyDefinitionsDestinyActivityGraphListEntryDefinition {
  /// Destinations and Activities may have default Activity Graphs that should be shown when you bring up the Director and are playing in either.  This contract defines the graph referred to and the gating for when it is relevant.
  pub fn new() -> DestinyDefinitionsDestinyActivityGraphListEntryDefinition {
    DestinyDefinitionsDestinyActivityGraphListEntryDefinition {
      activity_graph_hash: None
    }
  }

  pub fn set_activity_graph_hash(&mut self, activity_graph_hash: i32) {
    self.activity_graph_hash = Some(activity_graph_hash);
  }

  pub fn with_activity_graph_hash(mut self, activity_graph_hash: i32) -> DestinyDefinitionsDestinyActivityGraphListEntryDefinition {
    self.activity_graph_hash = Some(activity_graph_hash);
    self
  }

  pub fn activity_graph_hash(&self) -> Option<&i32> {
    self.activity_graph_hash.as_ref()
  }

  pub fn reset_activity_graph_hash(&mut self) {
    self.activity_graph_hash = None;
  }

}



