/* 
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * OpenAPI spec version: 2.0.0
 * Contact: support@bungie.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DestinyMilestonesDestinyMilestoneRewardEntry : The character-specific data for a milestone's reward entry. See DestinyMilestoneDefinition for more information about Reward Entries.

#[derive(Debug, Serialize, Deserialize)]
pub struct DestinyMilestonesDestinyMilestoneRewardEntry {
  /// The identifier for the reward entry in question. It is important to look up the related DestinyMilestoneRewardEntryDefinition to get the static details about the reward, which you can do by looking up the milestone's DestinyMilestoneDefinition and examining the DestinyMilestoneDefinition.rewards[rewardCategoryHash].rewardEntries[rewardEntryHash] data.
  #[serde(rename = "rewardEntryHash")]
  reward_entry_hash: Option<i32>,
  /// If TRUE, the player has earned this reward.
  #[serde(rename = "earned")]
  earned: Option<bool>,
  /// If TRUE, the player has redeemed/picked up/obtained this reward. Feel free to alias this to \"gotTheShinyBauble\" in your own codebase.
  #[serde(rename = "redeemed")]
  redeemed: Option<bool>
}

impl DestinyMilestonesDestinyMilestoneRewardEntry {
  /// The character-specific data for a milestone's reward entry. See DestinyMilestoneDefinition for more information about Reward Entries.
  pub fn new() -> DestinyMilestonesDestinyMilestoneRewardEntry {
    DestinyMilestonesDestinyMilestoneRewardEntry {
      reward_entry_hash: None,
      earned: None,
      redeemed: None
    }
  }

  pub fn set_reward_entry_hash(&mut self, reward_entry_hash: i32) {
    self.reward_entry_hash = Some(reward_entry_hash);
  }

  pub fn with_reward_entry_hash(mut self, reward_entry_hash: i32) -> DestinyMilestonesDestinyMilestoneRewardEntry {
    self.reward_entry_hash = Some(reward_entry_hash);
    self
  }

  pub fn reward_entry_hash(&self) -> Option<&i32> {
    self.reward_entry_hash.as_ref()
  }

  pub fn reset_reward_entry_hash(&mut self) {
    self.reward_entry_hash = None;
  }

  pub fn set_earned(&mut self, earned: bool) {
    self.earned = Some(earned);
  }

  pub fn with_earned(mut self, earned: bool) -> DestinyMilestonesDestinyMilestoneRewardEntry {
    self.earned = Some(earned);
    self
  }

  pub fn earned(&self) -> Option<&bool> {
    self.earned.as_ref()
  }

  pub fn reset_earned(&mut self) {
    self.earned = None;
  }

  pub fn set_redeemed(&mut self, redeemed: bool) {
    self.redeemed = Some(redeemed);
  }

  pub fn with_redeemed(mut self, redeemed: bool) -> DestinyMilestonesDestinyMilestoneRewardEntry {
    self.redeemed = Some(redeemed);
    self
  }

  pub fn redeemed(&self) -> Option<&bool> {
    self.redeemed.as_ref()
  }

  pub fn reset_redeemed(&mut self) {
    self.redeemed = None;
  }

}



