/* 
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * OpenAPI spec version: 2.0.0
 * Contact: support@bungie.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DestinyDefinitionsSocketsDestinySocketTypeDefinition : All Sockets have a \"Type\": a set of common properties that determine when the socket allows Plugs to be inserted, what Categories of Plugs can be inserted, and whether the socket is even visible at all given the current game/character/account state.  See DestinyInventoryItemDefinition for more information about Socketed items and Plugs.

#[derive(Debug, Serialize, Deserialize)]
pub struct DestinyDefinitionsSocketsDestinySocketTypeDefinition {
  /// There are fields for this display data, but they appear to be unpopulated as of now. I am not sure where in the UI these would show if they even were populated, but I will continue to return this data in case it becomes useful.
  #[serde(rename = "displayProperties")]
  display_properties: Option<Object>,
  /// Defines what happens when a plug is inserted into sockets of this type.
  #[serde(rename = "insertAction")]
  insert_action: Option<Object>,
  /// A list of Plug \"Categories\" that are allowed to be plugged into sockets of this type.  These should be compared against a given plug item's DestinyInventoryItemDefinition.plug.plugCategoryHash, which indicates the plug item's category.  If the plug's category matches any whitelisted plug, or if the whitelist is empty, it is allowed to be inserted.
  #[serde(rename = "plugWhitelist")]
  plug_whitelist: Option<Vec<::models::DestinyDefinitionsSocketsDestinyPlugWhitelistEntryDefinition>>,
  #[serde(rename = "socketCategoryHash")]
  socket_category_hash: Option<i32>,
  #[serde(rename = "visibility")]
  visibility: Option<::models::DestinyDestinySocketVisibility>,
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

impl DestinyDefinitionsSocketsDestinySocketTypeDefinition {
  /// All Sockets have a \"Type\": a set of common properties that determine when the socket allows Plugs to be inserted, what Categories of Plugs can be inserted, and whether the socket is even visible at all given the current game/character/account state.  See DestinyInventoryItemDefinition for more information about Socketed items and Plugs.
  pub fn new() -> DestinyDefinitionsSocketsDestinySocketTypeDefinition {
    DestinyDefinitionsSocketsDestinySocketTypeDefinition {
      display_properties: None,
      insert_action: None,
      plug_whitelist: None,
      socket_category_hash: None,
      visibility: None,
      hash: None,
      index: None,
      redacted: None
    }
  }

  pub fn set_display_properties(&mut self, display_properties: Object) {
    self.display_properties = Some(display_properties);
  }

  pub fn with_display_properties(mut self, display_properties: Object) -> DestinyDefinitionsSocketsDestinySocketTypeDefinition {
    self.display_properties = Some(display_properties);
    self
  }

  pub fn display_properties(&self) -> Option<&Object> {
    self.display_properties.as_ref()
  }

  pub fn reset_display_properties(&mut self) {
    self.display_properties = None;
  }

  pub fn set_insert_action(&mut self, insert_action: Object) {
    self.insert_action = Some(insert_action);
  }

  pub fn with_insert_action(mut self, insert_action: Object) -> DestinyDefinitionsSocketsDestinySocketTypeDefinition {
    self.insert_action = Some(insert_action);
    self
  }

  pub fn insert_action(&self) -> Option<&Object> {
    self.insert_action.as_ref()
  }

  pub fn reset_insert_action(&mut self) {
    self.insert_action = None;
  }

  pub fn set_plug_whitelist(&mut self, plug_whitelist: Vec<::models::DestinyDefinitionsSocketsDestinyPlugWhitelistEntryDefinition>) {
    self.plug_whitelist = Some(plug_whitelist);
  }

  pub fn with_plug_whitelist(mut self, plug_whitelist: Vec<::models::DestinyDefinitionsSocketsDestinyPlugWhitelistEntryDefinition>) -> DestinyDefinitionsSocketsDestinySocketTypeDefinition {
    self.plug_whitelist = Some(plug_whitelist);
    self
  }

  pub fn plug_whitelist(&self) -> Option<&Vec<::models::DestinyDefinitionsSocketsDestinyPlugWhitelistEntryDefinition>> {
    self.plug_whitelist.as_ref()
  }

  pub fn reset_plug_whitelist(&mut self) {
    self.plug_whitelist = None;
  }

  pub fn set_socket_category_hash(&mut self, socket_category_hash: i32) {
    self.socket_category_hash = Some(socket_category_hash);
  }

  pub fn with_socket_category_hash(mut self, socket_category_hash: i32) -> DestinyDefinitionsSocketsDestinySocketTypeDefinition {
    self.socket_category_hash = Some(socket_category_hash);
    self
  }

  pub fn socket_category_hash(&self) -> Option<&i32> {
    self.socket_category_hash.as_ref()
  }

  pub fn reset_socket_category_hash(&mut self) {
    self.socket_category_hash = None;
  }

  pub fn set_visibility(&mut self, visibility: ::models::DestinyDestinySocketVisibility) {
    self.visibility = Some(visibility);
  }

  pub fn with_visibility(mut self, visibility: ::models::DestinyDestinySocketVisibility) -> DestinyDefinitionsSocketsDestinySocketTypeDefinition {
    self.visibility = Some(visibility);
    self
  }

  pub fn visibility(&self) -> Option<&::models::DestinyDestinySocketVisibility> {
    self.visibility.as_ref()
  }

  pub fn reset_visibility(&mut self) {
    self.visibility = None;
  }

  pub fn set_hash(&mut self, hash: i32) {
    self.hash = Some(hash);
  }

  pub fn with_hash(mut self, hash: i32) -> DestinyDefinitionsSocketsDestinySocketTypeDefinition {
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

  pub fn with_index(mut self, index: i32) -> DestinyDefinitionsSocketsDestinySocketTypeDefinition {
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

  pub fn with_redacted(mut self, redacted: bool) -> DestinyDefinitionsSocketsDestinySocketTypeDefinition {
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



