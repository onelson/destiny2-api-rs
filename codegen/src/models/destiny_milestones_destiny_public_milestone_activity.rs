/* 
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * OpenAPI spec version: 2.0.0
 * Contact: support@bungie.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DestinyMilestonesDestinyPublicMilestoneActivity : A milestone may have one or more conceptual Activities associated with it, and each of those conceptual activities could have a variety of variants, modes, tiers, what-have-you. Our attempts to determine what qualifies as a conceptual activity are, unfortunately, janky. So if you see missing modes or modes that don't seem appropriate to you, let us know and I'll buy you a beer if we ever meet up in person.

#[derive(Debug, Serialize, Deserialize)]
pub struct DestinyMilestonesDestinyPublicMilestoneActivity {
  /// The hash identifier of the activity that's been chosen to be considered the canonical \"conceptual\" activity definition. This may have many variants, defined herein.
  #[serde(rename = "activityHash")]
  activity_hash: Option<i32>,
  /// The activity may have 0-to-many modifiers: if it does, this will contain the hashes to the DestinyActivityModifierDefinition that defines the modifier being applied.
  #[serde(rename = "modifierHashes")]
  modifier_hashes: Option<Vec<i32>>,
  /// Every relevant variation of this conceptual activity, including the conceptual activity itself, have variants defined here.
  #[serde(rename = "variants")]
  variants: Option<Vec<::models::DestinyMilestonesDestinyPublicMilestoneActivityVariant>>
}

impl DestinyMilestonesDestinyPublicMilestoneActivity {
  /// A milestone may have one or more conceptual Activities associated with it, and each of those conceptual activities could have a variety of variants, modes, tiers, what-have-you. Our attempts to determine what qualifies as a conceptual activity are, unfortunately, janky. So if you see missing modes or modes that don't seem appropriate to you, let us know and I'll buy you a beer if we ever meet up in person.
  pub fn new() -> DestinyMilestonesDestinyPublicMilestoneActivity {
    DestinyMilestonesDestinyPublicMilestoneActivity {
      activity_hash: None,
      modifier_hashes: None,
      variants: None
    }
  }

  pub fn set_activity_hash(&mut self, activity_hash: i32) {
    self.activity_hash = Some(activity_hash);
  }

  pub fn with_activity_hash(mut self, activity_hash: i32) -> DestinyMilestonesDestinyPublicMilestoneActivity {
    self.activity_hash = Some(activity_hash);
    self
  }

  pub fn activity_hash(&self) -> Option<&i32> {
    self.activity_hash.as_ref()
  }

  pub fn reset_activity_hash(&mut self) {
    self.activity_hash = None;
  }

  pub fn set_modifier_hashes(&mut self, modifier_hashes: Vec<i32>) {
    self.modifier_hashes = Some(modifier_hashes);
  }

  pub fn with_modifier_hashes(mut self, modifier_hashes: Vec<i32>) -> DestinyMilestonesDestinyPublicMilestoneActivity {
    self.modifier_hashes = Some(modifier_hashes);
    self
  }

  pub fn modifier_hashes(&self) -> Option<&Vec<i32>> {
    self.modifier_hashes.as_ref()
  }

  pub fn reset_modifier_hashes(&mut self) {
    self.modifier_hashes = None;
  }

  pub fn set_variants(&mut self, variants: Vec<::models::DestinyMilestonesDestinyPublicMilestoneActivityVariant>) {
    self.variants = Some(variants);
  }

  pub fn with_variants(mut self, variants: Vec<::models::DestinyMilestonesDestinyPublicMilestoneActivityVariant>) -> DestinyMilestonesDestinyPublicMilestoneActivity {
    self.variants = Some(variants);
    self
  }

  pub fn variants(&self) -> Option<&Vec<::models::DestinyMilestonesDestinyPublicMilestoneActivityVariant>> {
    self.variants.as_ref()
  }

  pub fn reset_variants(&mut self) {
    self.variants = None;
  }

}


