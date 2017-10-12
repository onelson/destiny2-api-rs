/* 
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * OpenAPI spec version: 2.0.0
 * Contact: support@bungie.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DestinyEntitiesInventoryDestinyInventoryComponent : A list of minimal information for items in an inventory: be it a character's inventory, or a Profile's inventory. (Note that the Vault is a collection of inventory buckets in the Profile's inventory)  Inventory Items returned here are in a flat list, but importantly they have a bucketHash property that indicates the specific inventory bucket that is holding them. These buckets constitute things like the separate sections of the Vault, the user's inventory slots, etc. See DestinyInventoryBucketDefinition for more info.

#[derive(Debug, Serialize, Deserialize)]
pub struct DestinyEntitiesInventoryDestinyInventoryComponent {
  /// The items in this inventory. If you care to bucket them, use the item's bucketHash property to group them.
  #[serde(rename = "items")]
  items: Option<Vec<::models::DestinyEntitiesItemsDestinyItemComponent>>
}

impl DestinyEntitiesInventoryDestinyInventoryComponent {
  /// A list of minimal information for items in an inventory: be it a character's inventory, or a Profile's inventory. (Note that the Vault is a collection of inventory buckets in the Profile's inventory)  Inventory Items returned here are in a flat list, but importantly they have a bucketHash property that indicates the specific inventory bucket that is holding them. These buckets constitute things like the separate sections of the Vault, the user's inventory slots, etc. See DestinyInventoryBucketDefinition for more info.
  pub fn new() -> DestinyEntitiesInventoryDestinyInventoryComponent {
    DestinyEntitiesInventoryDestinyInventoryComponent {
      items: None
    }
  }

  pub fn set_items(&mut self, items: Vec<::models::DestinyEntitiesItemsDestinyItemComponent>) {
    self.items = Some(items);
  }

  pub fn with_items(mut self, items: Vec<::models::DestinyEntitiesItemsDestinyItemComponent>) -> DestinyEntitiesInventoryDestinyInventoryComponent {
    self.items = Some(items);
    self
  }

  pub fn items(&self) -> Option<&Vec<::models::DestinyEntitiesItemsDestinyItemComponent>> {
    self.items.as_ref()
  }

  pub fn reset_items(&mut self) {
    self.items = None;
  }

}


