/* 
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * OpenAPI spec version: 2.0.0
 * Contact: support@bungie.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DestinyDefinitionsSocketsDestinyPlugWhitelistEntryDefinition : Defines a plug \"Category\" that is allowed to be plugged into a socket of this type.  This should be compared against a given plug item's DestinyInventoryItemDefinition.plug.plugCategoryHash, which indicates the plug item's category.

#[derive(Debug, Serialize, Deserialize)]
pub struct DestinyDefinitionsSocketsDestinyPlugWhitelistEntryDefinition {
  /// The hash identifier of the Plug Category to compare against the plug item's plug.plugCategoryHash.  Note that this does NOT relate to any Definition in itself, it is only used for comparison purposes.
  #[serde(rename = "categoryHash")]
  category_hash: Option<i32>,
  /// The string identifier for the category, which is here mostly for debug purposes.
  #[serde(rename = "categoryIdentifier")]
  category_identifier: Option<String>
}

impl DestinyDefinitionsSocketsDestinyPlugWhitelistEntryDefinition {
  /// Defines a plug \"Category\" that is allowed to be plugged into a socket of this type.  This should be compared against a given plug item's DestinyInventoryItemDefinition.plug.plugCategoryHash, which indicates the plug item's category.
  pub fn new() -> DestinyDefinitionsSocketsDestinyPlugWhitelistEntryDefinition {
    DestinyDefinitionsSocketsDestinyPlugWhitelistEntryDefinition {
      category_hash: None,
      category_identifier: None
    }
  }

  pub fn set_category_hash(&mut self, category_hash: i32) {
    self.category_hash = Some(category_hash);
  }

  pub fn with_category_hash(mut self, category_hash: i32) -> DestinyDefinitionsSocketsDestinyPlugWhitelistEntryDefinition {
    self.category_hash = Some(category_hash);
    self
  }

  pub fn category_hash(&self) -> Option<&i32> {
    self.category_hash.as_ref()
  }

  pub fn reset_category_hash(&mut self) {
    self.category_hash = None;
  }

  pub fn set_category_identifier(&mut self, category_identifier: String) {
    self.category_identifier = Some(category_identifier);
  }

  pub fn with_category_identifier(mut self, category_identifier: String) -> DestinyDefinitionsSocketsDestinyPlugWhitelistEntryDefinition {
    self.category_identifier = Some(category_identifier);
    self
  }

  pub fn category_identifier(&self) -> Option<&String> {
    self.category_identifier.as_ref()
  }

  pub fn reset_category_identifier(&mut self) {
    self.category_identifier = None;
  }

}



