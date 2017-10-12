/* 
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * OpenAPI spec version: 2.0.0
 * Contact: support@bungie.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DestinyDefinitionsDestinyVendorItemDefinition : This represents an item being sold by the vendor.

#[derive(Debug, Serialize, Deserialize)]
pub struct DestinyDefinitionsDestinyVendorItemDefinition {
  /// The index into the DestinyVendorDefinition.saleList. This is what we use to refer to items being sold throughout live and definition data.
  #[serde(rename = "vendorItemIndex")]
  vendor_item_index: Option<i32>,
  /// The hash identifier of the item being sold (DestinyInventoryItemDefinition).  Note that a vendor can sell the same item in multiple ways, so don't assume that itemHash is a unique identifier for this entity.
  #[serde(rename = "itemHash")]
  item_hash: Option<i32>,
  /// The amount you will recieve of the item described in itemHash if you make the purchase.
  #[serde(rename = "quantity")]
  quantity: Option<i32>,
  /// An list of indexes into the DestinyVendorDefinition.failureStrings array, indicating the possible failure strings that can be relevant for this item.
  #[serde(rename = "failureIndexes")]
  failure_indexes: Option<Vec<i32>>,
  /// This is a pre-compiled aggregation of item value and priceOverrideList, so that we have one place to check for what the purchaser must pay for the item. Use this instead of trying to piece together the price separately.
  #[serde(rename = "currencies")]
  currencies: Option<Vec<::models::DestinyDestinyItemQuantity>>,
  /// If this item can be refunded, this is the policy for what will be refundd, how, and in what time period.
  #[serde(rename = "refundPolicy")]
  refund_policy: Option<Object>,
  /// The amount of time before refundability of the newly purchased item will expire.
  #[serde(rename = "refundTimeLimit")]
  refund_time_limit: Option<i32>,
  /// The Default level at which the item will spawn. Almost always driven by an adjusto these days. Ideally should be singular. It's a long story how this ended up as a list, but there is always either going to be 0:1 of these entities.
  #[serde(rename = "creationLevels")]
  creation_levels: Option<Vec<::models::DestinyDefinitionsDestinyItemCreationEntryLevelDefinition>>,
  /// This is an index specifically into the display category, as opposed to the server-side Categories (which do not need to match or pair with each other in any way: server side categories are really just structures for common validation. Display Category will let us more easily categorize items visually)
  #[serde(rename = "displayCategoryIndex")]
  display_category_index: Option<i32>,
  /// The index into the DestinyVendorDefinition.categories array, so you can find the category associated with this item.
  #[serde(rename = "categoryIndex")]
  category_index: Option<i32>,
  /// Same as above, but for the original category indexes.
  #[serde(rename = "originalCategoryIndex")]
  original_category_index: Option<i32>,
  /// The minimum character level at which this item is available for sale.
  #[serde(rename = "minimumLevel")]
  minimum_level: Option<i32>,
  /// The maximum character level at which this item is available for sale.
  #[serde(rename = "maximumLevel")]
  maximum_level: Option<i32>,
  /// The action to be performed when purchasing the item, if it's not just \"buy\".
  #[serde(rename = "action")]
  action: Option<Object>,
  /// The string identifier for the category selling this item.
  #[serde(rename = "displayCategory")]
  display_category: Option<String>,
  /// The inventory bucket into which this item will be placed upon purchase.
  #[serde(rename = "inventoryBucketHash")]
  inventory_bucket_hash: Option<i32>,
  /// The most restrictive scope that determines whether the item is available in the Vendor's inventory. See DestinyGatingScope's documentation for more information.  This can be determined by Unlock gating, or by whether or not the item has purchase level requirements (minimumLevel and maximumLevel properties).
  #[serde(rename = "visibilityScope")]
  visibility_scope: Option<Object>,
  /// Similar to visibilityScope, it represents the most restrictive scope that determines whether the item can be purchased. It will at least be as restrictive as visibilityScope, but could be more restrictive if the item has additional purchase requirements beyond whether it is merely visible or not.  See DestinyGatingScope's documentation for more information.
  #[serde(rename = "purchasableScope")]
  purchasable_scope: Option<Object>
}

impl DestinyDefinitionsDestinyVendorItemDefinition {
  /// This represents an item being sold by the vendor.
  pub fn new() -> DestinyDefinitionsDestinyVendorItemDefinition {
    DestinyDefinitionsDestinyVendorItemDefinition {
      vendor_item_index: None,
      item_hash: None,
      quantity: None,
      failure_indexes: None,
      currencies: None,
      refund_policy: None,
      refund_time_limit: None,
      creation_levels: None,
      display_category_index: None,
      category_index: None,
      original_category_index: None,
      minimum_level: None,
      maximum_level: None,
      action: None,
      display_category: None,
      inventory_bucket_hash: None,
      visibility_scope: None,
      purchasable_scope: None
    }
  }

  pub fn set_vendor_item_index(&mut self, vendor_item_index: i32) {
    self.vendor_item_index = Some(vendor_item_index);
  }

  pub fn with_vendor_item_index(mut self, vendor_item_index: i32) -> DestinyDefinitionsDestinyVendorItemDefinition {
    self.vendor_item_index = Some(vendor_item_index);
    self
  }

  pub fn vendor_item_index(&self) -> Option<&i32> {
    self.vendor_item_index.as_ref()
  }

  pub fn reset_vendor_item_index(&mut self) {
    self.vendor_item_index = None;
  }

  pub fn set_item_hash(&mut self, item_hash: i32) {
    self.item_hash = Some(item_hash);
  }

  pub fn with_item_hash(mut self, item_hash: i32) -> DestinyDefinitionsDestinyVendorItemDefinition {
    self.item_hash = Some(item_hash);
    self
  }

  pub fn item_hash(&self) -> Option<&i32> {
    self.item_hash.as_ref()
  }

  pub fn reset_item_hash(&mut self) {
    self.item_hash = None;
  }

  pub fn set_quantity(&mut self, quantity: i32) {
    self.quantity = Some(quantity);
  }

  pub fn with_quantity(mut self, quantity: i32) -> DestinyDefinitionsDestinyVendorItemDefinition {
    self.quantity = Some(quantity);
    self
  }

  pub fn quantity(&self) -> Option<&i32> {
    self.quantity.as_ref()
  }

  pub fn reset_quantity(&mut self) {
    self.quantity = None;
  }

  pub fn set_failure_indexes(&mut self, failure_indexes: Vec<i32>) {
    self.failure_indexes = Some(failure_indexes);
  }

  pub fn with_failure_indexes(mut self, failure_indexes: Vec<i32>) -> DestinyDefinitionsDestinyVendorItemDefinition {
    self.failure_indexes = Some(failure_indexes);
    self
  }

  pub fn failure_indexes(&self) -> Option<&Vec<i32>> {
    self.failure_indexes.as_ref()
  }

  pub fn reset_failure_indexes(&mut self) {
    self.failure_indexes = None;
  }

  pub fn set_currencies(&mut self, currencies: Vec<::models::DestinyDestinyItemQuantity>) {
    self.currencies = Some(currencies);
  }

  pub fn with_currencies(mut self, currencies: Vec<::models::DestinyDestinyItemQuantity>) -> DestinyDefinitionsDestinyVendorItemDefinition {
    self.currencies = Some(currencies);
    self
  }

  pub fn currencies(&self) -> Option<&Vec<::models::DestinyDestinyItemQuantity>> {
    self.currencies.as_ref()
  }

  pub fn reset_currencies(&mut self) {
    self.currencies = None;
  }

  pub fn set_refund_policy(&mut self, refund_policy: Object) {
    self.refund_policy = Some(refund_policy);
  }

  pub fn with_refund_policy(mut self, refund_policy: Object) -> DestinyDefinitionsDestinyVendorItemDefinition {
    self.refund_policy = Some(refund_policy);
    self
  }

  pub fn refund_policy(&self) -> Option<&Object> {
    self.refund_policy.as_ref()
  }

  pub fn reset_refund_policy(&mut self) {
    self.refund_policy = None;
  }

  pub fn set_refund_time_limit(&mut self, refund_time_limit: i32) {
    self.refund_time_limit = Some(refund_time_limit);
  }

  pub fn with_refund_time_limit(mut self, refund_time_limit: i32) -> DestinyDefinitionsDestinyVendorItemDefinition {
    self.refund_time_limit = Some(refund_time_limit);
    self
  }

  pub fn refund_time_limit(&self) -> Option<&i32> {
    self.refund_time_limit.as_ref()
  }

  pub fn reset_refund_time_limit(&mut self) {
    self.refund_time_limit = None;
  }

  pub fn set_creation_levels(&mut self, creation_levels: Vec<::models::DestinyDefinitionsDestinyItemCreationEntryLevelDefinition>) {
    self.creation_levels = Some(creation_levels);
  }

  pub fn with_creation_levels(mut self, creation_levels: Vec<::models::DestinyDefinitionsDestinyItemCreationEntryLevelDefinition>) -> DestinyDefinitionsDestinyVendorItemDefinition {
    self.creation_levels = Some(creation_levels);
    self
  }

  pub fn creation_levels(&self) -> Option<&Vec<::models::DestinyDefinitionsDestinyItemCreationEntryLevelDefinition>> {
    self.creation_levels.as_ref()
  }

  pub fn reset_creation_levels(&mut self) {
    self.creation_levels = None;
  }

  pub fn set_display_category_index(&mut self, display_category_index: i32) {
    self.display_category_index = Some(display_category_index);
  }

  pub fn with_display_category_index(mut self, display_category_index: i32) -> DestinyDefinitionsDestinyVendorItemDefinition {
    self.display_category_index = Some(display_category_index);
    self
  }

  pub fn display_category_index(&self) -> Option<&i32> {
    self.display_category_index.as_ref()
  }

  pub fn reset_display_category_index(&mut self) {
    self.display_category_index = None;
  }

  pub fn set_category_index(&mut self, category_index: i32) {
    self.category_index = Some(category_index);
  }

  pub fn with_category_index(mut self, category_index: i32) -> DestinyDefinitionsDestinyVendorItemDefinition {
    self.category_index = Some(category_index);
    self
  }

  pub fn category_index(&self) -> Option<&i32> {
    self.category_index.as_ref()
  }

  pub fn reset_category_index(&mut self) {
    self.category_index = None;
  }

  pub fn set_original_category_index(&mut self, original_category_index: i32) {
    self.original_category_index = Some(original_category_index);
  }

  pub fn with_original_category_index(mut self, original_category_index: i32) -> DestinyDefinitionsDestinyVendorItemDefinition {
    self.original_category_index = Some(original_category_index);
    self
  }

  pub fn original_category_index(&self) -> Option<&i32> {
    self.original_category_index.as_ref()
  }

  pub fn reset_original_category_index(&mut self) {
    self.original_category_index = None;
  }

  pub fn set_minimum_level(&mut self, minimum_level: i32) {
    self.minimum_level = Some(minimum_level);
  }

  pub fn with_minimum_level(mut self, minimum_level: i32) -> DestinyDefinitionsDestinyVendorItemDefinition {
    self.minimum_level = Some(minimum_level);
    self
  }

  pub fn minimum_level(&self) -> Option<&i32> {
    self.minimum_level.as_ref()
  }

  pub fn reset_minimum_level(&mut self) {
    self.minimum_level = None;
  }

  pub fn set_maximum_level(&mut self, maximum_level: i32) {
    self.maximum_level = Some(maximum_level);
  }

  pub fn with_maximum_level(mut self, maximum_level: i32) -> DestinyDefinitionsDestinyVendorItemDefinition {
    self.maximum_level = Some(maximum_level);
    self
  }

  pub fn maximum_level(&self) -> Option<&i32> {
    self.maximum_level.as_ref()
  }

  pub fn reset_maximum_level(&mut self) {
    self.maximum_level = None;
  }

  pub fn set_action(&mut self, action: Object) {
    self.action = Some(action);
  }

  pub fn with_action(mut self, action: Object) -> DestinyDefinitionsDestinyVendorItemDefinition {
    self.action = Some(action);
    self
  }

  pub fn action(&self) -> Option<&Object> {
    self.action.as_ref()
  }

  pub fn reset_action(&mut self) {
    self.action = None;
  }

  pub fn set_display_category(&mut self, display_category: String) {
    self.display_category = Some(display_category);
  }

  pub fn with_display_category(mut self, display_category: String) -> DestinyDefinitionsDestinyVendorItemDefinition {
    self.display_category = Some(display_category);
    self
  }

  pub fn display_category(&self) -> Option<&String> {
    self.display_category.as_ref()
  }

  pub fn reset_display_category(&mut self) {
    self.display_category = None;
  }

  pub fn set_inventory_bucket_hash(&mut self, inventory_bucket_hash: i32) {
    self.inventory_bucket_hash = Some(inventory_bucket_hash);
  }

  pub fn with_inventory_bucket_hash(mut self, inventory_bucket_hash: i32) -> DestinyDefinitionsDestinyVendorItemDefinition {
    self.inventory_bucket_hash = Some(inventory_bucket_hash);
    self
  }

  pub fn inventory_bucket_hash(&self) -> Option<&i32> {
    self.inventory_bucket_hash.as_ref()
  }

  pub fn reset_inventory_bucket_hash(&mut self) {
    self.inventory_bucket_hash = None;
  }

  pub fn set_visibility_scope(&mut self, visibility_scope: Object) {
    self.visibility_scope = Some(visibility_scope);
  }

  pub fn with_visibility_scope(mut self, visibility_scope: Object) -> DestinyDefinitionsDestinyVendorItemDefinition {
    self.visibility_scope = Some(visibility_scope);
    self
  }

  pub fn visibility_scope(&self) -> Option<&Object> {
    self.visibility_scope.as_ref()
  }

  pub fn reset_visibility_scope(&mut self) {
    self.visibility_scope = None;
  }

  pub fn set_purchasable_scope(&mut self, purchasable_scope: Object) {
    self.purchasable_scope = Some(purchasable_scope);
  }

  pub fn with_purchasable_scope(mut self, purchasable_scope: Object) -> DestinyDefinitionsDestinyVendorItemDefinition {
    self.purchasable_scope = Some(purchasable_scope);
    self
  }

  pub fn purchasable_scope(&self) -> Option<&Object> {
    self.purchasable_scope.as_ref()
  }

  pub fn reset_purchasable_scope(&mut self) {
    self.purchasable_scope = None;
  }

}


