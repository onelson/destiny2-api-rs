/* 
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * OpenAPI spec version: 2.0.0
 * Contact: support@bungie.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DestinyEntitiesCharactersDestinyCharacterProgressionComponent : This component returns anything that could be considered \"Progression\" on a user: data where the user is gaining levels, reputation, completions, rewards, etc...

#[derive(Debug, Serialize, Deserialize)]
pub struct DestinyEntitiesCharactersDestinyCharacterProgressionComponent {
  /// A Dictionary of all known progressions for the Character, keyed by the Progression's hash.  Not all progressions have user-facing data, but those who do will have that data contained in the DestinyProgressionDefinition.
  #[serde(rename = "progressions")]
  progressions: Option<::std::collections::HashMap<String, ::models::DestinyDestinyProgression>>,
  /// A dictionary of all known Factions, keyed by the Faction's hash. It contains data about this character's status with the faction.
  #[serde(rename = "factions")]
  factions: Option<::std::collections::HashMap<String, ::models::DestinyProgressionDestinyFactionProgression>>,
  /// Milestones are related to the simple progressions shown in the game, but return additional and hopefully helpful information for users about the specifics of the Milestone's status.
  #[serde(rename = "milestones")]
  milestones: Option<::std::collections::HashMap<String, ::models::DestinyMilestonesDestinyMilestone>>,
  /// If the user has any active quests, the quests' statuses will be returned here.  Note that quests have been largely supplanted by Milestones, but that doesn't mean that they won't make a comeback independent of milestones at some point.
  #[serde(rename = "quests")]
  quests: Option<Vec<::models::DestinyQuestsDestinyQuestStatus>>,
  /// Sometimes, you have items in your inventory that don't have instances, but still have Objective information. This provides you that objective information for uninstanced items.   This dictionary is keyed by the item's hash: which you can use to look up the name and description for the overall task(s) implied by the objective. The value is the list of objectives for this item, and their statuses.
  #[serde(rename = "uninstancedItemObjectives")]
  uninstanced_item_objectives: Option<::std::collections::HashMap<String, Vec<::models::DestinyQuestsDestinyObjectiveProgress>>>
}

impl DestinyEntitiesCharactersDestinyCharacterProgressionComponent {
  /// This component returns anything that could be considered \"Progression\" on a user: data where the user is gaining levels, reputation, completions, rewards, etc...
  pub fn new() -> DestinyEntitiesCharactersDestinyCharacterProgressionComponent {
    DestinyEntitiesCharactersDestinyCharacterProgressionComponent {
      progressions: None,
      factions: None,
      milestones: None,
      quests: None,
      uninstanced_item_objectives: None
    }
  }

  pub fn set_progressions(&mut self, progressions: ::std::collections::HashMap<String, ::models::DestinyDestinyProgression>) {
    self.progressions = Some(progressions);
  }

  pub fn with_progressions(mut self, progressions: ::std::collections::HashMap<String, ::models::DestinyDestinyProgression>) -> DestinyEntitiesCharactersDestinyCharacterProgressionComponent {
    self.progressions = Some(progressions);
    self
  }

  pub fn progressions(&self) -> Option<&::std::collections::HashMap<String, ::models::DestinyDestinyProgression>> {
    self.progressions.as_ref()
  }

  pub fn reset_progressions(&mut self) {
    self.progressions = None;
  }

  pub fn set_factions(&mut self, factions: ::std::collections::HashMap<String, ::models::DestinyProgressionDestinyFactionProgression>) {
    self.factions = Some(factions);
  }

  pub fn with_factions(mut self, factions: ::std::collections::HashMap<String, ::models::DestinyProgressionDestinyFactionProgression>) -> DestinyEntitiesCharactersDestinyCharacterProgressionComponent {
    self.factions = Some(factions);
    self
  }

  pub fn factions(&self) -> Option<&::std::collections::HashMap<String, ::models::DestinyProgressionDestinyFactionProgression>> {
    self.factions.as_ref()
  }

  pub fn reset_factions(&mut self) {
    self.factions = None;
  }

  pub fn set_milestones(&mut self, milestones: ::std::collections::HashMap<String, ::models::DestinyMilestonesDestinyMilestone>) {
    self.milestones = Some(milestones);
  }

  pub fn with_milestones(mut self, milestones: ::std::collections::HashMap<String, ::models::DestinyMilestonesDestinyMilestone>) -> DestinyEntitiesCharactersDestinyCharacterProgressionComponent {
    self.milestones = Some(milestones);
    self
  }

  pub fn milestones(&self) -> Option<&::std::collections::HashMap<String, ::models::DestinyMilestonesDestinyMilestone>> {
    self.milestones.as_ref()
  }

  pub fn reset_milestones(&mut self) {
    self.milestones = None;
  }

  pub fn set_quests(&mut self, quests: Vec<::models::DestinyQuestsDestinyQuestStatus>) {
    self.quests = Some(quests);
  }

  pub fn with_quests(mut self, quests: Vec<::models::DestinyQuestsDestinyQuestStatus>) -> DestinyEntitiesCharactersDestinyCharacterProgressionComponent {
    self.quests = Some(quests);
    self
  }

  pub fn quests(&self) -> Option<&Vec<::models::DestinyQuestsDestinyQuestStatus>> {
    self.quests.as_ref()
  }

  pub fn reset_quests(&mut self) {
    self.quests = None;
  }

  pub fn set_uninstanced_item_objectives(&mut self, uninstanced_item_objectives: ::std::collections::HashMap<String, Vec<::models::DestinyQuestsDestinyObjectiveProgress>>) {
    self.uninstanced_item_objectives = Some(uninstanced_item_objectives);
  }

  pub fn with_uninstanced_item_objectives(mut self, uninstanced_item_objectives: ::std::collections::HashMap<String, Vec<::models::DestinyQuestsDestinyObjectiveProgress>>) -> DestinyEntitiesCharactersDestinyCharacterProgressionComponent {
    self.uninstanced_item_objectives = Some(uninstanced_item_objectives);
    self
  }

  pub fn uninstanced_item_objectives(&self) -> Option<&::std::collections::HashMap<String, Vec<::models::DestinyQuestsDestinyObjectiveProgress>>> {
    self.uninstanced_item_objectives.as_ref()
  }

  pub fn reset_uninstanced_item_objectives(&mut self) {
    self.uninstanced_item_objectives = None;
  }

}



