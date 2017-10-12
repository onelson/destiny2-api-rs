/* 
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * OpenAPI spec version: 2.0.0
 * Contact: support@bungie.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DestinyDefinitionsItemsDestinyItemPlugDefinition : If an item is a Plug, its DestinyInventoryItemDefinition.plug property will be populated with an instance of one of these bad boys.  This gives information about when it can be inserted, what the plug's category is (and thus whether it is compatible with a socket... see DestinySocketTypeDefinition for information about Plug Categories and socket compatibility), whether it is enabled and other Plug info.

#[derive(Debug, Serialize, Deserialize)]
pub struct DestinyDefinitionsItemsDestinyItemPlugDefinition {
  /// The rules around when this plug can be inserted into a socket, aside from the socket's individual restrictions.  The live data DestinyItemPlugComponent.insertFailIndexes will be an index into this array, so you can pull out the failure strings appropriate for the user.
  #[serde(rename = "insertionRules")]
  insertion_rules: Option<Vec<::models::DestinyDefinitionsItemsDestinyPlugRuleDefinition>>,
  /// The string identifier for the plug's category. Use the socket's DestinySocketTypeDefinition.plugWhitelist to determine whether this plug can be inserted into the socket.
  #[serde(rename = "plugCategoryIdentifier")]
  plug_category_identifier: Option<String>,
  /// The hash for the plugCategoryIdentifier. You can use this instead if you wish: I put both in the definition for debugging purposes.
  #[serde(rename = "plugCategoryHash")]
  plug_category_hash: Option<i32>,
  /// If you successfully socket the item, this will determine whether or not you get \"refunded\" on the plug.
  #[serde(rename = "onActionRecreateSelf")]
  on_action_recreate_self: Option<bool>,
  /// If inserting this plug requires materials, this is the hash identifier for looking up the DestinyMaterialRequirementSetDefinition for those requirements.
  #[serde(rename = "insertionMaterialRequirementHash")]
  insertion_material_requirement_hash: Option<i32>,
  /// In the game, if you're inspecting a plug item directly, this will be the item shown with the plug attached. Look up the DestinyInventoryItemDefinition for this hash for the item.
  #[serde(rename = "previewItemOverrideHash")]
  preview_item_override_hash: Option<i32>,
  /// It's not enough for the plug to be inserted. It has to be enabled as well. For it to be enabled, it may require materials. This is the hash identifier for the DestinyMaterialRequirementSetDefinition for those requirements, if there is one.
  #[serde(rename = "enabledMaterialRequirementHash")]
  enabled_material_requirement_hash: Option<i32>,
  /// The rules around whether the plug, once inserted, is enabled and providing its benefits.  The live data DestinyItemPlugComponent.enableFailIndexes will be an index into this array, so you can pull out the failure strings appropriate for the user.
  #[serde(rename = "enabledRules")]
  enabled_rules: Option<Vec<::models::DestinyDefinitionsItemsDestinyPlugRuleDefinition>>
}

impl DestinyDefinitionsItemsDestinyItemPlugDefinition {
  /// If an item is a Plug, its DestinyInventoryItemDefinition.plug property will be populated with an instance of one of these bad boys.  This gives information about when it can be inserted, what the plug's category is (and thus whether it is compatible with a socket... see DestinySocketTypeDefinition for information about Plug Categories and socket compatibility), whether it is enabled and other Plug info.
  pub fn new() -> DestinyDefinitionsItemsDestinyItemPlugDefinition {
    DestinyDefinitionsItemsDestinyItemPlugDefinition {
      insertion_rules: None,
      plug_category_identifier: None,
      plug_category_hash: None,
      on_action_recreate_self: None,
      insertion_material_requirement_hash: None,
      preview_item_override_hash: None,
      enabled_material_requirement_hash: None,
      enabled_rules: None
    }
  }

  pub fn set_insertion_rules(&mut self, insertion_rules: Vec<::models::DestinyDefinitionsItemsDestinyPlugRuleDefinition>) {
    self.insertion_rules = Some(insertion_rules);
  }

  pub fn with_insertion_rules(mut self, insertion_rules: Vec<::models::DestinyDefinitionsItemsDestinyPlugRuleDefinition>) -> DestinyDefinitionsItemsDestinyItemPlugDefinition {
    self.insertion_rules = Some(insertion_rules);
    self
  }

  pub fn insertion_rules(&self) -> Option<&Vec<::models::DestinyDefinitionsItemsDestinyPlugRuleDefinition>> {
    self.insertion_rules.as_ref()
  }

  pub fn reset_insertion_rules(&mut self) {
    self.insertion_rules = None;
  }

  pub fn set_plug_category_identifier(&mut self, plug_category_identifier: String) {
    self.plug_category_identifier = Some(plug_category_identifier);
  }

  pub fn with_plug_category_identifier(mut self, plug_category_identifier: String) -> DestinyDefinitionsItemsDestinyItemPlugDefinition {
    self.plug_category_identifier = Some(plug_category_identifier);
    self
  }

  pub fn plug_category_identifier(&self) -> Option<&String> {
    self.plug_category_identifier.as_ref()
  }

  pub fn reset_plug_category_identifier(&mut self) {
    self.plug_category_identifier = None;
  }

  pub fn set_plug_category_hash(&mut self, plug_category_hash: i32) {
    self.plug_category_hash = Some(plug_category_hash);
  }

  pub fn with_plug_category_hash(mut self, plug_category_hash: i32) -> DestinyDefinitionsItemsDestinyItemPlugDefinition {
    self.plug_category_hash = Some(plug_category_hash);
    self
  }

  pub fn plug_category_hash(&self) -> Option<&i32> {
    self.plug_category_hash.as_ref()
  }

  pub fn reset_plug_category_hash(&mut self) {
    self.plug_category_hash = None;
  }

  pub fn set_on_action_recreate_self(&mut self, on_action_recreate_self: bool) {
    self.on_action_recreate_self = Some(on_action_recreate_self);
  }

  pub fn with_on_action_recreate_self(mut self, on_action_recreate_self: bool) -> DestinyDefinitionsItemsDestinyItemPlugDefinition {
    self.on_action_recreate_self = Some(on_action_recreate_self);
    self
  }

  pub fn on_action_recreate_self(&self) -> Option<&bool> {
    self.on_action_recreate_self.as_ref()
  }

  pub fn reset_on_action_recreate_self(&mut self) {
    self.on_action_recreate_self = None;
  }

  pub fn set_insertion_material_requirement_hash(&mut self, insertion_material_requirement_hash: i32) {
    self.insertion_material_requirement_hash = Some(insertion_material_requirement_hash);
  }

  pub fn with_insertion_material_requirement_hash(mut self, insertion_material_requirement_hash: i32) -> DestinyDefinitionsItemsDestinyItemPlugDefinition {
    self.insertion_material_requirement_hash = Some(insertion_material_requirement_hash);
    self
  }

  pub fn insertion_material_requirement_hash(&self) -> Option<&i32> {
    self.insertion_material_requirement_hash.as_ref()
  }

  pub fn reset_insertion_material_requirement_hash(&mut self) {
    self.insertion_material_requirement_hash = None;
  }

  pub fn set_preview_item_override_hash(&mut self, preview_item_override_hash: i32) {
    self.preview_item_override_hash = Some(preview_item_override_hash);
  }

  pub fn with_preview_item_override_hash(mut self, preview_item_override_hash: i32) -> DestinyDefinitionsItemsDestinyItemPlugDefinition {
    self.preview_item_override_hash = Some(preview_item_override_hash);
    self
  }

  pub fn preview_item_override_hash(&self) -> Option<&i32> {
    self.preview_item_override_hash.as_ref()
  }

  pub fn reset_preview_item_override_hash(&mut self) {
    self.preview_item_override_hash = None;
  }

  pub fn set_enabled_material_requirement_hash(&mut self, enabled_material_requirement_hash: i32) {
    self.enabled_material_requirement_hash = Some(enabled_material_requirement_hash);
  }

  pub fn with_enabled_material_requirement_hash(mut self, enabled_material_requirement_hash: i32) -> DestinyDefinitionsItemsDestinyItemPlugDefinition {
    self.enabled_material_requirement_hash = Some(enabled_material_requirement_hash);
    self
  }

  pub fn enabled_material_requirement_hash(&self) -> Option<&i32> {
    self.enabled_material_requirement_hash.as_ref()
  }

  pub fn reset_enabled_material_requirement_hash(&mut self) {
    self.enabled_material_requirement_hash = None;
  }

  pub fn set_enabled_rules(&mut self, enabled_rules: Vec<::models::DestinyDefinitionsItemsDestinyPlugRuleDefinition>) {
    self.enabled_rules = Some(enabled_rules);
  }

  pub fn with_enabled_rules(mut self, enabled_rules: Vec<::models::DestinyDefinitionsItemsDestinyPlugRuleDefinition>) -> DestinyDefinitionsItemsDestinyItemPlugDefinition {
    self.enabled_rules = Some(enabled_rules);
    self
  }

  pub fn enabled_rules(&self) -> Option<&Vec<::models::DestinyDefinitionsItemsDestinyPlugRuleDefinition>> {
    self.enabled_rules.as_ref()
  }

  pub fn reset_enabled_rules(&mut self) {
    self.enabled_rules = None;
  }

}


