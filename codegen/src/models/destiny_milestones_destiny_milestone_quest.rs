/* 
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * OpenAPI spec version: 2.0.0
 * Contact: support@bungie.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DestinyMilestonesDestinyMilestoneQuest : If a Milestone has one or more Quests, this will contain the live information for the character's status with one of those quests.

#[derive(Debug, Serialize, Deserialize)]
pub struct DestinyMilestonesDestinyMilestoneQuest {
  /// Quests are defined as Items in content. As such, this is the hash identifier of the DestinyInventoryItemDefinition that represents this quest. It will have pointers to all of the steps in the quest, and display information for the quest (title, description, icon etc) Individual steps will be referred to in the Quest item's DestinyInventoryItemDefinition.setData property, and themselves are Items with their own renderable data.
  #[serde(rename = "questItemHash")]
  quest_item_hash: Option<i32>,
  /// The current status of the quest for the character making the request.
  #[serde(rename = "status")]
  status: Option<Object>,
  /// *IF* the Milestone has an active Activity that can give you greater details about what you need to do, it will be returned here. Remember to associate this with the DestinyMilestoneDefinition's activities to get details about the activity, including what specific quest it is related to if you have multiple quests to choose from.
  #[serde(rename = "activity")]
  activity: Option<Object>,
  /// The activities referred to by this quest can have many associated challenges. They are all contained here, with activityHashes so that you can associate them with the specific activity variants in which they can be found. In retrospect, I probably should have put these under the specific Activity Variants, but it's too late to change it now. Theoretically, a quest without Activities can still have Challenges, which is why this is on a higher level than activity/variants, but it probably should have been in both places. That may come as a later revision.
  #[serde(rename = "challenges")]
  challenges: Option<Vec<::models::DestinyChallengesDestinyChallengeStatus>>
}

impl DestinyMilestonesDestinyMilestoneQuest {
  /// If a Milestone has one or more Quests, this will contain the live information for the character's status with one of those quests.
  pub fn new() -> DestinyMilestonesDestinyMilestoneQuest {
    DestinyMilestonesDestinyMilestoneQuest {
      quest_item_hash: None,
      status: None,
      activity: None,
      challenges: None
    }
  }

  pub fn set_quest_item_hash(&mut self, quest_item_hash: i32) {
    self.quest_item_hash = Some(quest_item_hash);
  }

  pub fn with_quest_item_hash(mut self, quest_item_hash: i32) -> DestinyMilestonesDestinyMilestoneQuest {
    self.quest_item_hash = Some(quest_item_hash);
    self
  }

  pub fn quest_item_hash(&self) -> Option<&i32> {
    self.quest_item_hash.as_ref()
  }

  pub fn reset_quest_item_hash(&mut self) {
    self.quest_item_hash = None;
  }

  pub fn set_status(&mut self, status: Object) {
    self.status = Some(status);
  }

  pub fn with_status(mut self, status: Object) -> DestinyMilestonesDestinyMilestoneQuest {
    self.status = Some(status);
    self
  }

  pub fn status(&self) -> Option<&Object> {
    self.status.as_ref()
  }

  pub fn reset_status(&mut self) {
    self.status = None;
  }

  pub fn set_activity(&mut self, activity: Object) {
    self.activity = Some(activity);
  }

  pub fn with_activity(mut self, activity: Object) -> DestinyMilestonesDestinyMilestoneQuest {
    self.activity = Some(activity);
    self
  }

  pub fn activity(&self) -> Option<&Object> {
    self.activity.as_ref()
  }

  pub fn reset_activity(&mut self) {
    self.activity = None;
  }

  pub fn set_challenges(&mut self, challenges: Vec<::models::DestinyChallengesDestinyChallengeStatus>) {
    self.challenges = Some(challenges);
  }

  pub fn with_challenges(mut self, challenges: Vec<::models::DestinyChallengesDestinyChallengeStatus>) -> DestinyMilestonesDestinyMilestoneQuest {
    self.challenges = Some(challenges);
    self
  }

  pub fn challenges(&self) -> Option<&Vec<::models::DestinyChallengesDestinyChallengeStatus>> {
    self.challenges.as_ref()
  }

  pub fn reset_challenges(&mut self) {
    self.challenges = None;
  }

}


