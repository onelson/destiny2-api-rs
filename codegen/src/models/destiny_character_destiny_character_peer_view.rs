/* 
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * OpenAPI spec version: 2.0.0
 * Contact: support@bungie.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DestinyCharacterDestinyCharacterPeerView : A minimal view of a character's equipped items, for the purpose of rendering a summary screen or showing the character in 3D.

#[derive(Debug, Serialize, Deserialize)]
pub struct DestinyCharacterDestinyCharacterPeerView {
  #[serde(rename = "equipment")]
  equipment: Option<Vec<::models::DestinyCharacterDestinyItemPeerView>>
}

impl DestinyCharacterDestinyCharacterPeerView {
  /// A minimal view of a character's equipped items, for the purpose of rendering a summary screen or showing the character in 3D.
  pub fn new() -> DestinyCharacterDestinyCharacterPeerView {
    DestinyCharacterDestinyCharacterPeerView {
      equipment: None
    }
  }

  pub fn set_equipment(&mut self, equipment: Vec<::models::DestinyCharacterDestinyItemPeerView>) {
    self.equipment = Some(equipment);
  }

  pub fn with_equipment(mut self, equipment: Vec<::models::DestinyCharacterDestinyItemPeerView>) -> DestinyCharacterDestinyCharacterPeerView {
    self.equipment = Some(equipment);
    self
  }

  pub fn equipment(&self) -> Option<&Vec<::models::DestinyCharacterDestinyItemPeerView>> {
    self.equipment.as_ref()
  }

  pub fn reset_equipment(&mut self) {
    self.equipment = None;
  }

}



