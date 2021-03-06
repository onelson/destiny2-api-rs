/* 
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * OpenAPI spec version: 2.0.0
 * Contact: support@bungie.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */


#[derive(Debug, Serialize, Deserialize)]
pub struct DestinyDefinitionsItemsDestinyItemTierTypeInfusionBlock {
  /// The default portion of quality that will transfer from the infuser to the infusee item. (InfuserQuality - InfuseeQuality) * baseQualityTransferRatio = base quality transferred.
  #[serde(rename = "baseQualityTransferRatio")]
  base_quality_transfer_ratio: Option<f32>,
  /// As long as InfuserQuality > InfuseeQuality, the amount of quality bestowed is guaranteed to be at least this value, even if the transferRatio would dictate that it should be less. The total amount of quality that ends up in the Infusee cannot exceed the Infuser's quality however (for instance, if you infuse a 300 item with a 301 item and the minimum quality increment is 10, the infused item will not end up with 310 quality)
  #[serde(rename = "minimumQualityIncrement")]
  minimum_quality_increment: Option<i32>
}

impl DestinyDefinitionsItemsDestinyItemTierTypeInfusionBlock {
  pub fn new() -> DestinyDefinitionsItemsDestinyItemTierTypeInfusionBlock {
    DestinyDefinitionsItemsDestinyItemTierTypeInfusionBlock {
      base_quality_transfer_ratio: None,
      minimum_quality_increment: None
    }
  }

  pub fn set_base_quality_transfer_ratio(&mut self, base_quality_transfer_ratio: f32) {
    self.base_quality_transfer_ratio = Some(base_quality_transfer_ratio);
  }

  pub fn with_base_quality_transfer_ratio(mut self, base_quality_transfer_ratio: f32) -> DestinyDefinitionsItemsDestinyItemTierTypeInfusionBlock {
    self.base_quality_transfer_ratio = Some(base_quality_transfer_ratio);
    self
  }

  pub fn base_quality_transfer_ratio(&self) -> Option<&f32> {
    self.base_quality_transfer_ratio.as_ref()
  }

  pub fn reset_base_quality_transfer_ratio(&mut self) {
    self.base_quality_transfer_ratio = None;
  }

  pub fn set_minimum_quality_increment(&mut self, minimum_quality_increment: i32) {
    self.minimum_quality_increment = Some(minimum_quality_increment);
  }

  pub fn with_minimum_quality_increment(mut self, minimum_quality_increment: i32) -> DestinyDefinitionsItemsDestinyItemTierTypeInfusionBlock {
    self.minimum_quality_increment = Some(minimum_quality_increment);
    self
  }

  pub fn minimum_quality_increment(&self) -> Option<&i32> {
    self.minimum_quality_increment.as_ref()
  }

  pub fn reset_minimum_quality_increment(&mut self) {
    self.minimum_quality_increment = None;
  }

}



