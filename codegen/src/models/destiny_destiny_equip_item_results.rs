/* 
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * OpenAPI spec version: 2.0.0
 * Contact: support@bungie.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DestinyDestinyEquipItemResults : The results of a bulk Equipping operation performed through the Destiny API.

#[derive(Debug, Serialize, Deserialize)]
pub struct DestinyDestinyEquipItemResults {
  #[serde(rename = "equipResults")]
  equip_results: Option<Vec<::models::DestinyDestinyEquipItemResult>>
}

impl DestinyDestinyEquipItemResults {
  /// The results of a bulk Equipping operation performed through the Destiny API.
  pub fn new() -> DestinyDestinyEquipItemResults {
    DestinyDestinyEquipItemResults {
      equip_results: None
    }
  }

  pub fn set_equip_results(&mut self, equip_results: Vec<::models::DestinyDestinyEquipItemResult>) {
    self.equip_results = Some(equip_results);
  }

  pub fn with_equip_results(mut self, equip_results: Vec<::models::DestinyDestinyEquipItemResult>) -> DestinyDestinyEquipItemResults {
    self.equip_results = Some(equip_results);
    self
  }

  pub fn equip_results(&self) -> Option<&Vec<::models::DestinyDestinyEquipItemResult>> {
    self.equip_results.as_ref()
  }

  pub fn reset_equip_results(&mut self) {
    self.equip_results = None;
  }

}



