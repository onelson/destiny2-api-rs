/* 
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * OpenAPI spec version: 2.0.0
 * Contact: support@bungie.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DestinyDefinitionsDestinyStatGroupDefinition : When an inventory item (DestinyInventoryItemDefinition) has Stats (such as Attack Power), the item will refer to a Stat Group. This definition enumerates the properties used to transform the item's \"Investment\" stats into \"Display\" stats.  See DestinyStatDefinition's documentation for information about the transformation of Stats, and the meaning of an Investment vs. a Display stat.  If you don't want to do these calculations on your own, fear not: pulling live data from the BNet endpoints will return display stat values pre-computed and ready for you to use. I highly recommend this approach, saves a lot of time and also accounts for certain stat modifiers that can't easily be accounted for without live data (such as stat modifiers on Talent Grids and Socket Plugs)

#[derive(Debug, Serialize, Deserialize)]
pub struct DestinyDefinitionsDestinyStatGroupDefinition {
  /// The maximum possible value that any stat in this group can be transformed into.  This is used by stats that *don't* have scaledStats entries below, but that still need to be displayed as a progress bar, in which case this is used as the upper bound for said progress bar. (the lower bound is always 0)
  #[serde(rename = "maximumValue")]
  maximum_value: Option<i32>,
  /// This apparently indicates the position of the stats in the UI? I've returned it in case anyone can use it, but it's not of any use to us on BNet. Something's being lost in translation with this value.
  #[serde(rename = "uiPosition")]
  ui_position: Option<i32>,
  /// Any stat that requires scaling to be transformed from an \"Investment\" stat to a \"Display\" stat will have an entry in this list. For more information on what those types of stats mean and the transformation process, see DestinyStatDefinition.  In retrospect, I wouldn't mind if this was a dictionary keyed by the stat hash instead. But I'm going to leave it be because [[After Apple Picking]].
  #[serde(rename = "scaledStats")]
  scaled_stats: Option<Vec<::models::DestinyDefinitionsDestinyStatDisplayDefinition>>,
  /// The game has the ability to override, based on the stat group, what the localized text is that is displayed for Stats being shown on the item.  Mercifully, no Stat Groups use this feature currently. If they start using them, we'll all need to start using them (and those of you who are more prudent than I am can go ahead and start pre-checking for this.)
  #[serde(rename = "overrides")]
  overrides: Option<::std::collections::HashMap<String, ::models::DestinyDefinitionsDestinyStatOverrideDefinition>>,
  /// The unique identifier for this entity. Guaranteed to be unique for the type of entity, but not globally.  When entities refer to each other in Destiny content, it is this hash that they are referring to.
  #[serde(rename = "hash")]
  hash: Option<i32>,
  /// The index of the entity as it was found in the investment tables.
  #[serde(rename = "index")]
  index: Option<i32>,
  /// If this is true, then there is an entity with this identifier/type combination, but BNet is not yet allowed to show it. Sorry!
  #[serde(rename = "redacted")]
  redacted: Option<bool>
}

impl DestinyDefinitionsDestinyStatGroupDefinition {
  /// When an inventory item (DestinyInventoryItemDefinition) has Stats (such as Attack Power), the item will refer to a Stat Group. This definition enumerates the properties used to transform the item's \"Investment\" stats into \"Display\" stats.  See DestinyStatDefinition's documentation for information about the transformation of Stats, and the meaning of an Investment vs. a Display stat.  If you don't want to do these calculations on your own, fear not: pulling live data from the BNet endpoints will return display stat values pre-computed and ready for you to use. I highly recommend this approach, saves a lot of time and also accounts for certain stat modifiers that can't easily be accounted for without live data (such as stat modifiers on Talent Grids and Socket Plugs)
  pub fn new() -> DestinyDefinitionsDestinyStatGroupDefinition {
    DestinyDefinitionsDestinyStatGroupDefinition {
      maximum_value: None,
      ui_position: None,
      scaled_stats: None,
      overrides: None,
      hash: None,
      index: None,
      redacted: None
    }
  }

  pub fn set_maximum_value(&mut self, maximum_value: i32) {
    self.maximum_value = Some(maximum_value);
  }

  pub fn with_maximum_value(mut self, maximum_value: i32) -> DestinyDefinitionsDestinyStatGroupDefinition {
    self.maximum_value = Some(maximum_value);
    self
  }

  pub fn maximum_value(&self) -> Option<&i32> {
    self.maximum_value.as_ref()
  }

  pub fn reset_maximum_value(&mut self) {
    self.maximum_value = None;
  }

  pub fn set_ui_position(&mut self, ui_position: i32) {
    self.ui_position = Some(ui_position);
  }

  pub fn with_ui_position(mut self, ui_position: i32) -> DestinyDefinitionsDestinyStatGroupDefinition {
    self.ui_position = Some(ui_position);
    self
  }

  pub fn ui_position(&self) -> Option<&i32> {
    self.ui_position.as_ref()
  }

  pub fn reset_ui_position(&mut self) {
    self.ui_position = None;
  }

  pub fn set_scaled_stats(&mut self, scaled_stats: Vec<::models::DestinyDefinitionsDestinyStatDisplayDefinition>) {
    self.scaled_stats = Some(scaled_stats);
  }

  pub fn with_scaled_stats(mut self, scaled_stats: Vec<::models::DestinyDefinitionsDestinyStatDisplayDefinition>) -> DestinyDefinitionsDestinyStatGroupDefinition {
    self.scaled_stats = Some(scaled_stats);
    self
  }

  pub fn scaled_stats(&self) -> Option<&Vec<::models::DestinyDefinitionsDestinyStatDisplayDefinition>> {
    self.scaled_stats.as_ref()
  }

  pub fn reset_scaled_stats(&mut self) {
    self.scaled_stats = None;
  }

  pub fn set_overrides(&mut self, overrides: ::std::collections::HashMap<String, ::models::DestinyDefinitionsDestinyStatOverrideDefinition>) {
    self.overrides = Some(overrides);
  }

  pub fn with_overrides(mut self, overrides: ::std::collections::HashMap<String, ::models::DestinyDefinitionsDestinyStatOverrideDefinition>) -> DestinyDefinitionsDestinyStatGroupDefinition {
    self.overrides = Some(overrides);
    self
  }

  pub fn overrides(&self) -> Option<&::std::collections::HashMap<String, ::models::DestinyDefinitionsDestinyStatOverrideDefinition>> {
    self.overrides.as_ref()
  }

  pub fn reset_overrides(&mut self) {
    self.overrides = None;
  }

  pub fn set_hash(&mut self, hash: i32) {
    self.hash = Some(hash);
  }

  pub fn with_hash(mut self, hash: i32) -> DestinyDefinitionsDestinyStatGroupDefinition {
    self.hash = Some(hash);
    self
  }

  pub fn hash(&self) -> Option<&i32> {
    self.hash.as_ref()
  }

  pub fn reset_hash(&mut self) {
    self.hash = None;
  }

  pub fn set_index(&mut self, index: i32) {
    self.index = Some(index);
  }

  pub fn with_index(mut self, index: i32) -> DestinyDefinitionsDestinyStatGroupDefinition {
    self.index = Some(index);
    self
  }

  pub fn index(&self) -> Option<&i32> {
    self.index.as_ref()
  }

  pub fn reset_index(&mut self) {
    self.index = None;
  }

  pub fn set_redacted(&mut self, redacted: bool) {
    self.redacted = Some(redacted);
  }

  pub fn with_redacted(mut self, redacted: bool) -> DestinyDefinitionsDestinyStatGroupDefinition {
    self.redacted = Some(redacted);
    self
  }

  pub fn redacted(&self) -> Option<&bool> {
    self.redacted.as_ref()
  }

  pub fn reset_redacted(&mut self) {
    self.redacted = None;
  }

}



