/* 
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * OpenAPI spec version: 2.0.0
 * Contact: support@bungie.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DestinyDefinitionsDestinyNodeSocketReplaceResponse : This is a bit of an odd duck. Apparently, if talent nodes steps have this data, the game will go through on step activation and alter the first Socket it finds on the item that has a type matching the given socket type, inserting the indicated plug item.

#[derive(Debug, Serialize, Deserialize)]
pub struct DestinyDefinitionsDestinyNodeSocketReplaceResponse {
  /// The hash identifier of the socket type to find amidst the item's sockets (the item to which this talent grid is attached). See DestinyInventoryItemDefinition.sockets.socketEntries to find the socket type of sockets on the item in question.
  #[serde(rename = "socketTypeHash")]
  socket_type_hash: Option<i32>,
  /// The hash identifier of the plug item that will be inserted into the socket found.
  #[serde(rename = "plugItemHash")]
  plug_item_hash: Option<i32>
}

impl DestinyDefinitionsDestinyNodeSocketReplaceResponse {
  /// This is a bit of an odd duck. Apparently, if talent nodes steps have this data, the game will go through on step activation and alter the first Socket it finds on the item that has a type matching the given socket type, inserting the indicated plug item.
  pub fn new() -> DestinyDefinitionsDestinyNodeSocketReplaceResponse {
    DestinyDefinitionsDestinyNodeSocketReplaceResponse {
      socket_type_hash: None,
      plug_item_hash: None
    }
  }

  pub fn set_socket_type_hash(&mut self, socket_type_hash: i32) {
    self.socket_type_hash = Some(socket_type_hash);
  }

  pub fn with_socket_type_hash(mut self, socket_type_hash: i32) -> DestinyDefinitionsDestinyNodeSocketReplaceResponse {
    self.socket_type_hash = Some(socket_type_hash);
    self
  }

  pub fn socket_type_hash(&self) -> Option<&i32> {
    self.socket_type_hash.as_ref()
  }

  pub fn reset_socket_type_hash(&mut self) {
    self.socket_type_hash = None;
  }

  pub fn set_plug_item_hash(&mut self, plug_item_hash: i32) {
    self.plug_item_hash = Some(plug_item_hash);
  }

  pub fn with_plug_item_hash(mut self, plug_item_hash: i32) -> DestinyDefinitionsDestinyNodeSocketReplaceResponse {
    self.plug_item_hash = Some(plug_item_hash);
    self
  }

  pub fn plug_item_hash(&self) -> Option<&i32> {
    self.plug_item_hash.as_ref()
  }

  pub fn reset_plug_item_hash(&mut self) {
    self.plug_item_hash = None;
  }

}



