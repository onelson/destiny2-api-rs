/* 
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * OpenAPI spec version: 2.0.0
 * Contact: support@bungie.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DestinyMilestonesDestinyMilestoneActivityCompletionStatus : Represents this player's personal completion status for the Activity under a Milestone, if the activity has trackable completion and progress information. (most activities won't, or the concept won't apply. For instance, it makes sense to talk about a tier of a raid as being Completed or having progress, but it doesn't make sense to talk about a Crucible Playlist in those terms.

#[derive(Debug, Serialize, Deserialize)]
pub struct DestinyMilestonesDestinyMilestoneActivityCompletionStatus {
  /// If the activity has been \"completed\", that information will be returned here.
  #[serde(rename = "completed")]
  completed: Option<bool>,
  /// If the Activity has discrete \"phases\" that we can track, that info will be here. Otherwise, this value will be NULL. Note that this is a list and not a dictionary: the order implies the ascending order of phases or progression in this activity.
  #[serde(rename = "phases")]
  phases: Option<Vec<::models::DestinyMilestonesDestinyMilestoneActivityPhase>>
}

impl DestinyMilestonesDestinyMilestoneActivityCompletionStatus {
  /// Represents this player's personal completion status for the Activity under a Milestone, if the activity has trackable completion and progress information. (most activities won't, or the concept won't apply. For instance, it makes sense to talk about a tier of a raid as being Completed or having progress, but it doesn't make sense to talk about a Crucible Playlist in those terms.
  pub fn new() -> DestinyMilestonesDestinyMilestoneActivityCompletionStatus {
    DestinyMilestonesDestinyMilestoneActivityCompletionStatus {
      completed: None,
      phases: None
    }
  }

  pub fn set_completed(&mut self, completed: bool) {
    self.completed = Some(completed);
  }

  pub fn with_completed(mut self, completed: bool) -> DestinyMilestonesDestinyMilestoneActivityCompletionStatus {
    self.completed = Some(completed);
    self
  }

  pub fn completed(&self) -> Option<&bool> {
    self.completed.as_ref()
  }

  pub fn reset_completed(&mut self) {
    self.completed = None;
  }

  pub fn set_phases(&mut self, phases: Vec<::models::DestinyMilestonesDestinyMilestoneActivityPhase>) {
    self.phases = Some(phases);
  }

  pub fn with_phases(mut self, phases: Vec<::models::DestinyMilestonesDestinyMilestoneActivityPhase>) -> DestinyMilestonesDestinyMilestoneActivityCompletionStatus {
    self.phases = Some(phases);
    self
  }

  pub fn phases(&self) -> Option<&Vec<::models::DestinyMilestonesDestinyMilestoneActivityPhase>> {
    self.phases.as_ref()
  }

  pub fn reset_phases(&mut self) {
    self.phases = None;
  }

}



