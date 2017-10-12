/* 
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * OpenAPI spec version: 2.0.0
 * Contact: support@bungie.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DestinyQuestsDestinyQuestStatus : Data regarding the progress of a Quest for a specific character. Quests are composed of multiple steps, each with potentially multiple objectives: this QuestStatus will return Objective data for the *currently active* step in this quest.

#[derive(Debug, Serialize, Deserialize)]
pub struct DestinyQuestsDestinyQuestStatus {
  /// The hash identifier for the Quest Item. (Note: Quests are defined as Items, and thus you would use this to look up the quest's DestinyInventoryItemDefinition). For information on all steps in the quest, you can then examine its DestinyInventoryItemDefinition.setData property for Quest Steps (which are *also* items). You can use the Item Definition to display human readable data about the overall quest.
  #[serde(rename = "questHash")]
  quest_hash: Option<i32>,
  /// The hash identifier of the current Quest Step, which is also a DestinyInventoryItemDefinition. You can use this to get human readable data about the current step and what to do in that step.
  #[serde(rename = "stepHash")]
  step_hash: Option<i32>,
  /// A step can have multiple objectives. This will give you the progress for each objective in the current step, in the order in which they are rendered in-game.
  #[serde(rename = "stepObjectives")]
  step_objectives: Option<Vec<::models::DestinyQuestsDestinyObjectiveProgress>>,
  /// Whether or not the quest is tracked
  #[serde(rename = "tracked")]
  tracked: Option<bool>,
  /// The current Quest Step will be an instanced item in the player's inventory. If you care about that, this is the instance ID of that item.
  #[serde(rename = "itemInstanceId")]
  item_instance_id: Option<i64>,
  /// Whether or not the whole quest has been completed, regardless of whether or not you have redeemed the rewards for the quest.
  #[serde(rename = "completed")]
  completed: Option<bool>,
  /// Whether or not you have redeemed rewards for this quest.
  #[serde(rename = "redeemed")]
  redeemed: Option<bool>,
  /// Whether or not you have started this quest.
  #[serde(rename = "started")]
  started: Option<bool>,
  /// If the quest has a related Vendor that you should talk to in order to initiate the quest/earn rewards/continue the quest, this will be the hash identifier of that Vendor. Look it up its DestinyVendorDefinition.
  #[serde(rename = "vendorHash")]
  vendor_hash: Option<i32>
}

impl DestinyQuestsDestinyQuestStatus {
  /// Data regarding the progress of a Quest for a specific character. Quests are composed of multiple steps, each with potentially multiple objectives: this QuestStatus will return Objective data for the *currently active* step in this quest.
  pub fn new() -> DestinyQuestsDestinyQuestStatus {
    DestinyQuestsDestinyQuestStatus {
      quest_hash: None,
      step_hash: None,
      step_objectives: None,
      tracked: None,
      item_instance_id: None,
      completed: None,
      redeemed: None,
      started: None,
      vendor_hash: None
    }
  }

  pub fn set_quest_hash(&mut self, quest_hash: i32) {
    self.quest_hash = Some(quest_hash);
  }

  pub fn with_quest_hash(mut self, quest_hash: i32) -> DestinyQuestsDestinyQuestStatus {
    self.quest_hash = Some(quest_hash);
    self
  }

  pub fn quest_hash(&self) -> Option<&i32> {
    self.quest_hash.as_ref()
  }

  pub fn reset_quest_hash(&mut self) {
    self.quest_hash = None;
  }

  pub fn set_step_hash(&mut self, step_hash: i32) {
    self.step_hash = Some(step_hash);
  }

  pub fn with_step_hash(mut self, step_hash: i32) -> DestinyQuestsDestinyQuestStatus {
    self.step_hash = Some(step_hash);
    self
  }

  pub fn step_hash(&self) -> Option<&i32> {
    self.step_hash.as_ref()
  }

  pub fn reset_step_hash(&mut self) {
    self.step_hash = None;
  }

  pub fn set_step_objectives(&mut self, step_objectives: Vec<::models::DestinyQuestsDestinyObjectiveProgress>) {
    self.step_objectives = Some(step_objectives);
  }

  pub fn with_step_objectives(mut self, step_objectives: Vec<::models::DestinyQuestsDestinyObjectiveProgress>) -> DestinyQuestsDestinyQuestStatus {
    self.step_objectives = Some(step_objectives);
    self
  }

  pub fn step_objectives(&self) -> Option<&Vec<::models::DestinyQuestsDestinyObjectiveProgress>> {
    self.step_objectives.as_ref()
  }

  pub fn reset_step_objectives(&mut self) {
    self.step_objectives = None;
  }

  pub fn set_tracked(&mut self, tracked: bool) {
    self.tracked = Some(tracked);
  }

  pub fn with_tracked(mut self, tracked: bool) -> DestinyQuestsDestinyQuestStatus {
    self.tracked = Some(tracked);
    self
  }

  pub fn tracked(&self) -> Option<&bool> {
    self.tracked.as_ref()
  }

  pub fn reset_tracked(&mut self) {
    self.tracked = None;
  }

  pub fn set_item_instance_id(&mut self, item_instance_id: i64) {
    self.item_instance_id = Some(item_instance_id);
  }

  pub fn with_item_instance_id(mut self, item_instance_id: i64) -> DestinyQuestsDestinyQuestStatus {
    self.item_instance_id = Some(item_instance_id);
    self
  }

  pub fn item_instance_id(&self) -> Option<&i64> {
    self.item_instance_id.as_ref()
  }

  pub fn reset_item_instance_id(&mut self) {
    self.item_instance_id = None;
  }

  pub fn set_completed(&mut self, completed: bool) {
    self.completed = Some(completed);
  }

  pub fn with_completed(mut self, completed: bool) -> DestinyQuestsDestinyQuestStatus {
    self.completed = Some(completed);
    self
  }

  pub fn completed(&self) -> Option<&bool> {
    self.completed.as_ref()
  }

  pub fn reset_completed(&mut self) {
    self.completed = None;
  }

  pub fn set_redeemed(&mut self, redeemed: bool) {
    self.redeemed = Some(redeemed);
  }

  pub fn with_redeemed(mut self, redeemed: bool) -> DestinyQuestsDestinyQuestStatus {
    self.redeemed = Some(redeemed);
    self
  }

  pub fn redeemed(&self) -> Option<&bool> {
    self.redeemed.as_ref()
  }

  pub fn reset_redeemed(&mut self) {
    self.redeemed = None;
  }

  pub fn set_started(&mut self, started: bool) {
    self.started = Some(started);
  }

  pub fn with_started(mut self, started: bool) -> DestinyQuestsDestinyQuestStatus {
    self.started = Some(started);
    self
  }

  pub fn started(&self) -> Option<&bool> {
    self.started.as_ref()
  }

  pub fn reset_started(&mut self) {
    self.started = None;
  }

  pub fn set_vendor_hash(&mut self, vendor_hash: i32) {
    self.vendor_hash = Some(vendor_hash);
  }

  pub fn with_vendor_hash(mut self, vendor_hash: i32) -> DestinyQuestsDestinyQuestStatus {
    self.vendor_hash = Some(vendor_hash);
    self
  }

  pub fn vendor_hash(&self) -> Option<&i32> {
    self.vendor_hash.as_ref()
  }

  pub fn reset_vendor_hash(&mut self) {
    self.vendor_hash = None;
  }

}



