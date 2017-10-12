/* 
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * OpenAPI spec version: 2.0.0
 * Contact: support@bungie.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DestinyDefinitionsDestinyActivityRewardDefinition : Activities can refer to one or more sets of tooltip-friendly reward data. These are the definitions for those tooltip friendly rewards.

#[derive(Debug, Serialize, Deserialize)]
pub struct DestinyDefinitionsDestinyActivityRewardDefinition {
  /// The header for the reward set, if any.
  #[serde(rename = "rewardText")]
  reward_text: Option<String>,
  /// The \"Items provided\" in the reward. This is almost always a pointer to a DestinyInventoryItemDefintion for an item that you can't actually earn in-game, but that has name/description/icon information for the vague concept of the rewards you will receive. This is because the actual reward generation is non-deterministic and extremely complicated, so the best the game can do is tell you what you'll get in vague terms. And so too shall we.  Interesting trivia: you actually *do* earn these items when you complete the activity. They go into a single-slot bucket on your profile, which is how you see the pop-ups of these rewards when you complete an activity that match these \"dummy\" items. You can even see them if you look at the last one you earned in your profile-level inventory through the BNet API! Who said reading documentation is a waste of time?
  #[serde(rename = "rewardItems")]
  reward_items: Option<Vec<::models::DestinyDestinyItemQuantity>>
}

impl DestinyDefinitionsDestinyActivityRewardDefinition {
  /// Activities can refer to one or more sets of tooltip-friendly reward data. These are the definitions for those tooltip friendly rewards.
  pub fn new() -> DestinyDefinitionsDestinyActivityRewardDefinition {
    DestinyDefinitionsDestinyActivityRewardDefinition {
      reward_text: None,
      reward_items: None
    }
  }

  pub fn set_reward_text(&mut self, reward_text: String) {
    self.reward_text = Some(reward_text);
  }

  pub fn with_reward_text(mut self, reward_text: String) -> DestinyDefinitionsDestinyActivityRewardDefinition {
    self.reward_text = Some(reward_text);
    self
  }

  pub fn reward_text(&self) -> Option<&String> {
    self.reward_text.as_ref()
  }

  pub fn reset_reward_text(&mut self) {
    self.reward_text = None;
  }

  pub fn set_reward_items(&mut self, reward_items: Vec<::models::DestinyDestinyItemQuantity>) {
    self.reward_items = Some(reward_items);
  }

  pub fn with_reward_items(mut self, reward_items: Vec<::models::DestinyDestinyItemQuantity>) -> DestinyDefinitionsDestinyActivityRewardDefinition {
    self.reward_items = Some(reward_items);
    self
  }

  pub fn reward_items(&self) -> Option<&Vec<::models::DestinyDestinyItemQuantity>> {
    self.reward_items.as_ref()
  }

  pub fn reset_reward_items(&mut self) {
    self.reward_items = None;
  }

}



