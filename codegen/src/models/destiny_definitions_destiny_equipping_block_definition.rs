/* 
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * OpenAPI spec version: 2.0.0
 * Contact: support@bungie.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DestinyDefinitionsDestinyEquippingBlockDefinition : Items that can be equipped define this block. It contains information we need to understand how and when the item can be equipped.

#[derive(Debug, Serialize, Deserialize)]
pub struct DestinyDefinitionsDestinyEquippingBlockDefinition {
  /// If the item is part of a gearset, this is a reference to that gearset item.
  #[serde(rename = "gearsetItemHash")]
  gearset_item_hash: Option<i32>,
  /// If defined, this is the label used to check if the item has other items of matching types already equipped.   For instance, when you aren't allowed to equip more than one Exotic Weapon, that's because all exotic weapons have identical uniqueLabels and the game checks the to-be-equipped item's uniqueLabel vs. all other already equipped items (other than the item in the slot that's about to be occupied).
  #[serde(rename = "uniqueLabel")]
  unique_label: Option<String>,
  /// The hash of that unique label. Does not point to a specific definition.
  #[serde(rename = "uniqueLabelHash")]
  unique_label_hash: Option<i32>,
  /// An equipped item *must* be equipped in an Equipment Slot. This is the hash identifier of the DestinyEquipmentSlotDefinition into which it must be equipped.
  #[serde(rename = "equipmentSlotTypeHash")]
  equipment_slot_type_hash: Option<i32>,
  /// These are custom attributes on the equippability of the item.  For now, this can only be \"equip on acquire\", which would mean that the item will be automatically equipped as soon as you pick it up.
  #[serde(rename = "attributes")]
  attributes: Option<Object>,
  /// These are strings that represent the possible Game/Account/Character state failure conditions that can occur when trying to equip the item. They match up one-to-one with requiredUnlockExpressions.
  #[serde(rename = "displayStrings")]
  display_strings: Option<Vec<String>>
}

impl DestinyDefinitionsDestinyEquippingBlockDefinition {
  /// Items that can be equipped define this block. It contains information we need to understand how and when the item can be equipped.
  pub fn new() -> DestinyDefinitionsDestinyEquippingBlockDefinition {
    DestinyDefinitionsDestinyEquippingBlockDefinition {
      gearset_item_hash: None,
      unique_label: None,
      unique_label_hash: None,
      equipment_slot_type_hash: None,
      attributes: None,
      display_strings: None
    }
  }

  pub fn set_gearset_item_hash(&mut self, gearset_item_hash: i32) {
    self.gearset_item_hash = Some(gearset_item_hash);
  }

  pub fn with_gearset_item_hash(mut self, gearset_item_hash: i32) -> DestinyDefinitionsDestinyEquippingBlockDefinition {
    self.gearset_item_hash = Some(gearset_item_hash);
    self
  }

  pub fn gearset_item_hash(&self) -> Option<&i32> {
    self.gearset_item_hash.as_ref()
  }

  pub fn reset_gearset_item_hash(&mut self) {
    self.gearset_item_hash = None;
  }

  pub fn set_unique_label(&mut self, unique_label: String) {
    self.unique_label = Some(unique_label);
  }

  pub fn with_unique_label(mut self, unique_label: String) -> DestinyDefinitionsDestinyEquippingBlockDefinition {
    self.unique_label = Some(unique_label);
    self
  }

  pub fn unique_label(&self) -> Option<&String> {
    self.unique_label.as_ref()
  }

  pub fn reset_unique_label(&mut self) {
    self.unique_label = None;
  }

  pub fn set_unique_label_hash(&mut self, unique_label_hash: i32) {
    self.unique_label_hash = Some(unique_label_hash);
  }

  pub fn with_unique_label_hash(mut self, unique_label_hash: i32) -> DestinyDefinitionsDestinyEquippingBlockDefinition {
    self.unique_label_hash = Some(unique_label_hash);
    self
  }

  pub fn unique_label_hash(&self) -> Option<&i32> {
    self.unique_label_hash.as_ref()
  }

  pub fn reset_unique_label_hash(&mut self) {
    self.unique_label_hash = None;
  }

  pub fn set_equipment_slot_type_hash(&mut self, equipment_slot_type_hash: i32) {
    self.equipment_slot_type_hash = Some(equipment_slot_type_hash);
  }

  pub fn with_equipment_slot_type_hash(mut self, equipment_slot_type_hash: i32) -> DestinyDefinitionsDestinyEquippingBlockDefinition {
    self.equipment_slot_type_hash = Some(equipment_slot_type_hash);
    self
  }

  pub fn equipment_slot_type_hash(&self) -> Option<&i32> {
    self.equipment_slot_type_hash.as_ref()
  }

  pub fn reset_equipment_slot_type_hash(&mut self) {
    self.equipment_slot_type_hash = None;
  }

  pub fn set_attributes(&mut self, attributes: Object) {
    self.attributes = Some(attributes);
  }

  pub fn with_attributes(mut self, attributes: Object) -> DestinyDefinitionsDestinyEquippingBlockDefinition {
    self.attributes = Some(attributes);
    self
  }

  pub fn attributes(&self) -> Option<&Object> {
    self.attributes.as_ref()
  }

  pub fn reset_attributes(&mut self) {
    self.attributes = None;
  }

  pub fn set_display_strings(&mut self, display_strings: Vec<String>) {
    self.display_strings = Some(display_strings);
  }

  pub fn with_display_strings(mut self, display_strings: Vec<String>) -> DestinyDefinitionsDestinyEquippingBlockDefinition {
    self.display_strings = Some(display_strings);
    self
  }

  pub fn display_strings(&self) -> Option<&Vec<String>> {
    self.display_strings.as_ref()
  }

  pub fn reset_display_strings(&mut self) {
    self.display_strings = None;
  }

}



