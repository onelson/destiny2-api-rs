/* 
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * OpenAPI spec version: 2.0.0
 * Contact: support@bungie.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DestinyDefinitionsDestinyItemSocketEntryDefinition : The definition information for a specific socket on an item. This will determine how the socket behaves in-game.

#[derive(Debug, Serialize, Deserialize)]
pub struct DestinyDefinitionsDestinyItemSocketEntryDefinition {
  /// All sockets have a type, and this is the hash identifier for this particular type. Use it to look up the DestinySocketTypeDefinition: read there for more information on how socket types affect the behavior of the socket.
  #[serde(rename = "socketTypeHash")]
  socket_type_hash: Option<i32>,
  /// If a valid hash, this is the hash identifier for the DestinyInventoryItemDefinition representing the Plug that will be initially inserted into the item on item creation. Otherwise, this Socket will either start without a plug inserted, or will have one randomly inserted.
  #[serde(rename = "singleInitialItemHash")]
  single_initial_item_hash: Option<i32>,
  /// This is a list of pre-determined plugs that can *always* be plugged into this socket, without the character having the plug in their inventory.  If this list is populated, you will not be allowed to plug an arbitrary item in the socket: you will only be able to choose from one of these reusable plugs.
  #[serde(rename = "reusablePlugItems")]
  reusable_plug_items: Option<Vec<::models::DestinyDefinitionsDestinyItemSocketEntryPlugItemDefinition>>
}

impl DestinyDefinitionsDestinyItemSocketEntryDefinition {
  /// The definition information for a specific socket on an item. This will determine how the socket behaves in-game.
  pub fn new() -> DestinyDefinitionsDestinyItemSocketEntryDefinition {
    DestinyDefinitionsDestinyItemSocketEntryDefinition {
      socket_type_hash: None,
      single_initial_item_hash: None,
      reusable_plug_items: None
    }
  }

  pub fn set_socket_type_hash(&mut self, socket_type_hash: i32) {
    self.socket_type_hash = Some(socket_type_hash);
  }

  pub fn with_socket_type_hash(mut self, socket_type_hash: i32) -> DestinyDefinitionsDestinyItemSocketEntryDefinition {
    self.socket_type_hash = Some(socket_type_hash);
    self
  }

  pub fn socket_type_hash(&self) -> Option<&i32> {
    self.socket_type_hash.as_ref()
  }

  pub fn reset_socket_type_hash(&mut self) {
    self.socket_type_hash = None;
  }

  pub fn set_single_initial_item_hash(&mut self, single_initial_item_hash: i32) {
    self.single_initial_item_hash = Some(single_initial_item_hash);
  }

  pub fn with_single_initial_item_hash(mut self, single_initial_item_hash: i32) -> DestinyDefinitionsDestinyItemSocketEntryDefinition {
    self.single_initial_item_hash = Some(single_initial_item_hash);
    self
  }

  pub fn single_initial_item_hash(&self) -> Option<&i32> {
    self.single_initial_item_hash.as_ref()
  }

  pub fn reset_single_initial_item_hash(&mut self) {
    self.single_initial_item_hash = None;
  }

  pub fn set_reusable_plug_items(&mut self, reusable_plug_items: Vec<::models::DestinyDefinitionsDestinyItemSocketEntryPlugItemDefinition>) {
    self.reusable_plug_items = Some(reusable_plug_items);
  }

  pub fn with_reusable_plug_items(mut self, reusable_plug_items: Vec<::models::DestinyDefinitionsDestinyItemSocketEntryPlugItemDefinition>) -> DestinyDefinitionsDestinyItemSocketEntryDefinition {
    self.reusable_plug_items = Some(reusable_plug_items);
    self
  }

  pub fn reusable_plug_items(&self) -> Option<&Vec<::models::DestinyDefinitionsDestinyItemSocketEntryPlugItemDefinition>> {
    self.reusable_plug_items.as_ref()
  }

  pub fn reset_reusable_plug_items(&mut self) {
    self.reusable_plug_items = None;
  }

}



