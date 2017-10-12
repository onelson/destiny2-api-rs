/* 
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * OpenAPI spec version: 2.0.0
 * Contact: support@bungie.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DestinyDefinitionsDestinyStatDefinition : This represents a stat that's applied to a character or an item (such as a weapon, piece of armor, or a vehicle).  An example of a stat might be Attack Power on a weapon.  Stats go through a complex set of transformations before they end up being shown to the user as a number or a progress bar, and those transformations are fundamentally intertwined with the concept of a \"Stat Group\" (DestinyStatGroupDefinition). Items have both Stats and a reference to a Stat Group, and it is the Stat Group that takes the raw stat information and gives it both rendering metadata (such as whether to show it as a number or a progress bar) and the final transformation data (interpolation tables to turn the raw investment stat into a display stat). Please see DestinyStatGroupDefinition for more information on that transformational process.  Stats are segregated from Stat Groups because different items and types of items can refer to the same stat, but have different \"scales\" for the stat while still having the same underlying value. For example, both a Shotgun and an Auto Rifle may have a \"raw\" impact stat of 50, but the Auto Rifle's Stat Group will scale that 50 down so that, when it is displayed, it is a smaller value relative to the shotgun. (this is a totally made up example, don't assume shotguns have naturally higher impact than auto rifles because of this)  A final caveat is that some stats, even after this \"final\" transformation, go through yet another set of transformations directly in the game as a result of dynamic, stateful scripts that get run. BNet has no access to these scripts, nor any way to know which scripts get executed. As a result, the stats for an item that you see in-game - particularly for stats that are often impacted by Perks, like Magazine Size - can change dramatically from what we return on Bungie.Net. This is a known issue with no fix coming down the pipeline. Take these stats with a grain of salt.  Stats actually go through four transformations, for those interested:  1) \"Sandbox\" stat, the \"most raw\" form. These are pretty much useless without transformations applied, and thus are not currently returned in the API. If you really want these, we can provide them. Maybe someone could do something cool with it?  2) \"Investment\" stat (the stat's value after DestinyStatDefinition's interpolation tables and aggregation logic is applied to the \"Sandbox\" stat value)  3) \"Display\" stat (the stat's base UI-visible value after DestinyStatGroupDefinition's interpolation tables are applied to the Investment Stat value. For most stats, this is what is displayed.)  4) Underlying in-game stat (the stat's actual value according to the game, after the game runs dynamic scripts based on the game and character's state. This is the final transformation that BNet does not have access to. For most stats, this is not actually displayed to the user, with the exception of Magazine Size which is then piped back to the UI for display in-game, but not to BNet.)

#[derive(Debug, Serialize, Deserialize)]
pub struct DestinyDefinitionsDestinyStatDefinition {
  #[serde(rename = "displayProperties")]
  display_properties: Option<::models::DestinyDefinitionsCommonDestinyDisplayPropertiesDefinition>,
  /// Stats can exist on a character or an item, and they may potentially be aggregated in different ways. The DestinyStatAggregationType enum value indicates the way that this stat is being aggregated.
  #[serde(rename = "aggregationType")]
  aggregation_type: Option<Object>,
  /// True if the stat is computed rather than being delivered as a raw value on items.  For instance, the Light stat in Destiny 1 was a computed stat.
  #[serde(rename = "hasComputedBlock")]
  has_computed_block: Option<bool>,
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

impl DestinyDefinitionsDestinyStatDefinition {
  /// This represents a stat that's applied to a character or an item (such as a weapon, piece of armor, or a vehicle).  An example of a stat might be Attack Power on a weapon.  Stats go through a complex set of transformations before they end up being shown to the user as a number or a progress bar, and those transformations are fundamentally intertwined with the concept of a \"Stat Group\" (DestinyStatGroupDefinition). Items have both Stats and a reference to a Stat Group, and it is the Stat Group that takes the raw stat information and gives it both rendering metadata (such as whether to show it as a number or a progress bar) and the final transformation data (interpolation tables to turn the raw investment stat into a display stat). Please see DestinyStatGroupDefinition for more information on that transformational process.  Stats are segregated from Stat Groups because different items and types of items can refer to the same stat, but have different \"scales\" for the stat while still having the same underlying value. For example, both a Shotgun and an Auto Rifle may have a \"raw\" impact stat of 50, but the Auto Rifle's Stat Group will scale that 50 down so that, when it is displayed, it is a smaller value relative to the shotgun. (this is a totally made up example, don't assume shotguns have naturally higher impact than auto rifles because of this)  A final caveat is that some stats, even after this \"final\" transformation, go through yet another set of transformations directly in the game as a result of dynamic, stateful scripts that get run. BNet has no access to these scripts, nor any way to know which scripts get executed. As a result, the stats for an item that you see in-game - particularly for stats that are often impacted by Perks, like Magazine Size - can change dramatically from what we return on Bungie.Net. This is a known issue with no fix coming down the pipeline. Take these stats with a grain of salt.  Stats actually go through four transformations, for those interested:  1) \"Sandbox\" stat, the \"most raw\" form. These are pretty much useless without transformations applied, and thus are not currently returned in the API. If you really want these, we can provide them. Maybe someone could do something cool with it?  2) \"Investment\" stat (the stat's value after DestinyStatDefinition's interpolation tables and aggregation logic is applied to the \"Sandbox\" stat value)  3) \"Display\" stat (the stat's base UI-visible value after DestinyStatGroupDefinition's interpolation tables are applied to the Investment Stat value. For most stats, this is what is displayed.)  4) Underlying in-game stat (the stat's actual value according to the game, after the game runs dynamic scripts based on the game and character's state. This is the final transformation that BNet does not have access to. For most stats, this is not actually displayed to the user, with the exception of Magazine Size which is then piped back to the UI for display in-game, but not to BNet.)
  pub fn new() -> DestinyDefinitionsDestinyStatDefinition {
    DestinyDefinitionsDestinyStatDefinition {
      display_properties: None,
      aggregation_type: None,
      has_computed_block: None,
      hash: None,
      index: None,
      redacted: None
    }
  }

  pub fn set_display_properties(&mut self, display_properties: ::models::DestinyDefinitionsCommonDestinyDisplayPropertiesDefinition) {
    self.display_properties = Some(display_properties);
  }

  pub fn with_display_properties(mut self, display_properties: ::models::DestinyDefinitionsCommonDestinyDisplayPropertiesDefinition) -> DestinyDefinitionsDestinyStatDefinition {
    self.display_properties = Some(display_properties);
    self
  }

  pub fn display_properties(&self) -> Option<&::models::DestinyDefinitionsCommonDestinyDisplayPropertiesDefinition> {
    self.display_properties.as_ref()
  }

  pub fn reset_display_properties(&mut self) {
    self.display_properties = None;
  }

  pub fn set_aggregation_type(&mut self, aggregation_type: Object) {
    self.aggregation_type = Some(aggregation_type);
  }

  pub fn with_aggregation_type(mut self, aggregation_type: Object) -> DestinyDefinitionsDestinyStatDefinition {
    self.aggregation_type = Some(aggregation_type);
    self
  }

  pub fn aggregation_type(&self) -> Option<&Object> {
    self.aggregation_type.as_ref()
  }

  pub fn reset_aggregation_type(&mut self) {
    self.aggregation_type = None;
  }

  pub fn set_has_computed_block(&mut self, has_computed_block: bool) {
    self.has_computed_block = Some(has_computed_block);
  }

  pub fn with_has_computed_block(mut self, has_computed_block: bool) -> DestinyDefinitionsDestinyStatDefinition {
    self.has_computed_block = Some(has_computed_block);
    self
  }

  pub fn has_computed_block(&self) -> Option<&bool> {
    self.has_computed_block.as_ref()
  }

  pub fn reset_has_computed_block(&mut self) {
    self.has_computed_block = None;
  }

  pub fn set_hash(&mut self, hash: i32) {
    self.hash = Some(hash);
  }

  pub fn with_hash(mut self, hash: i32) -> DestinyDefinitionsDestinyStatDefinition {
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

  pub fn with_index(mut self, index: i32) -> DestinyDefinitionsDestinyStatDefinition {
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

  pub fn with_redacted(mut self, redacted: bool) -> DestinyDefinitionsDestinyStatDefinition {
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



