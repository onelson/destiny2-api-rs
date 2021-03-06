/* 
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * OpenAPI spec version: 2.0.0
 * Contact: support@bungie.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DestinyEntitiesVendorsDestinyVendorSaleItemComponent : Request this component if you want the details about an item being sold in relation to the character making the request: whether the character can buy it, whether they can afford it, and other data related to purchasing the item.  Note that if you want instance, stats, etc... data for the item, you'll have to request additional components such as ItemInstances, ItemPerks etc... and acquire them from the DestinyVendorResponse's \"items\" property.

#[derive(Debug, Serialize, Deserialize)]
pub struct DestinyEntitiesVendorsDestinyVendorSaleItemComponent {
  /// The index into the DestinyVendorDefinition.itemList property. Note that this means Vendor data *is* Content Version dependent: make sure you have the latest content before you use Vendor data, or these indexes may mismatch.   Most systems avoid this problem, but Vendors is one area where we are unable to reasonably avoid content dependency at the moment.
  #[serde(rename = "vendorItemIndex")]
  vendor_item_index: Option<i32>,
  /// The hash of the item being sold, as a quick shortcut for looking up the DestinyInventoryItemDefinition of the sale item.
  #[serde(rename = "itemHash")]
  item_hash: Option<i32>,
  /// A flag indicating whether the requesting character can buy the item, and if not the reasons why the character can't buy it.
  #[serde(rename = "saleStatus")]
  sale_status: Option<Object>,
  /// A summary of the current costs of the item.
  #[serde(rename = "costs")]
  costs: Option<Vec<::models::DestinyDestinyItemQuantity>>,
  /// If you can't buy the item due to a complex character state, these will be hashes for DestinyUnlockDefinitions that you can check to see messages regarding the failure (if the unlocks have human readable information: it is not guaranteed that Unlocks will have human readable strings, and your application will have to handle that)  Prefer using failureIndexes instead. These are provided for informational purposes, but have largely been supplanted by failureIndexes.
  #[serde(rename = "requiredUnlocks")]
  required_unlocks: Option<Vec<i32>>,
  /// If any complex unlock states are checked in determining purchasability, these will be returned here along with the status of the unlock check.  Prefer using failureIndexes instead. These are provided for informational purposes, but have largely been supplanted by failureIndexes.
  #[serde(rename = "unlockStatuses")]
  unlock_statuses: Option<Vec<::models::DestinyDestinyUnlockStatus>>,
  /// Indexes in to the \"failureStrings\" lookup table in DestinyVendorDefinition for the given Vendor. Gives some more reliable failure information for why you can't purchase an item.  It is preferred to use these over requiredUnlocks and unlockStatuses: the latter are provided mostly in case someone can do something interesting with it that I didn't anticipate.
  #[serde(rename = "failureIndexes")]
  failure_indexes: Option<Vec<i32>>
}

impl DestinyEntitiesVendorsDestinyVendorSaleItemComponent {
  /// Request this component if you want the details about an item being sold in relation to the character making the request: whether the character can buy it, whether they can afford it, and other data related to purchasing the item.  Note that if you want instance, stats, etc... data for the item, you'll have to request additional components such as ItemInstances, ItemPerks etc... and acquire them from the DestinyVendorResponse's \"items\" property.
  pub fn new() -> DestinyEntitiesVendorsDestinyVendorSaleItemComponent {
    DestinyEntitiesVendorsDestinyVendorSaleItemComponent {
      vendor_item_index: None,
      item_hash: None,
      sale_status: None,
      costs: None,
      required_unlocks: None,
      unlock_statuses: None,
      failure_indexes: None
    }
  }

  pub fn set_vendor_item_index(&mut self, vendor_item_index: i32) {
    self.vendor_item_index = Some(vendor_item_index);
  }

  pub fn with_vendor_item_index(mut self, vendor_item_index: i32) -> DestinyEntitiesVendorsDestinyVendorSaleItemComponent {
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

  pub fn with_item_hash(mut self, item_hash: i32) -> DestinyEntitiesVendorsDestinyVendorSaleItemComponent {
    self.item_hash = Some(item_hash);
    self
  }

  pub fn item_hash(&self) -> Option<&i32> {
    self.item_hash.as_ref()
  }

  pub fn reset_item_hash(&mut self) {
    self.item_hash = None;
  }

  pub fn set_sale_status(&mut self, sale_status: Object) {
    self.sale_status = Some(sale_status);
  }

  pub fn with_sale_status(mut self, sale_status: Object) -> DestinyEntitiesVendorsDestinyVendorSaleItemComponent {
    self.sale_status = Some(sale_status);
    self
  }

  pub fn sale_status(&self) -> Option<&Object> {
    self.sale_status.as_ref()
  }

  pub fn reset_sale_status(&mut self) {
    self.sale_status = None;
  }

  pub fn set_costs(&mut self, costs: Vec<::models::DestinyDestinyItemQuantity>) {
    self.costs = Some(costs);
  }

  pub fn with_costs(mut self, costs: Vec<::models::DestinyDestinyItemQuantity>) -> DestinyEntitiesVendorsDestinyVendorSaleItemComponent {
    self.costs = Some(costs);
    self
  }

  pub fn costs(&self) -> Option<&Vec<::models::DestinyDestinyItemQuantity>> {
    self.costs.as_ref()
  }

  pub fn reset_costs(&mut self) {
    self.costs = None;
  }

  pub fn set_required_unlocks(&mut self, required_unlocks: Vec<i32>) {
    self.required_unlocks = Some(required_unlocks);
  }

  pub fn with_required_unlocks(mut self, required_unlocks: Vec<i32>) -> DestinyEntitiesVendorsDestinyVendorSaleItemComponent {
    self.required_unlocks = Some(required_unlocks);
    self
  }

  pub fn required_unlocks(&self) -> Option<&Vec<i32>> {
    self.required_unlocks.as_ref()
  }

  pub fn reset_required_unlocks(&mut self) {
    self.required_unlocks = None;
  }

  pub fn set_unlock_statuses(&mut self, unlock_statuses: Vec<::models::DestinyDestinyUnlockStatus>) {
    self.unlock_statuses = Some(unlock_statuses);
  }

  pub fn with_unlock_statuses(mut self, unlock_statuses: Vec<::models::DestinyDestinyUnlockStatus>) -> DestinyEntitiesVendorsDestinyVendorSaleItemComponent {
    self.unlock_statuses = Some(unlock_statuses);
    self
  }

  pub fn unlock_statuses(&self) -> Option<&Vec<::models::DestinyDestinyUnlockStatus>> {
    self.unlock_statuses.as_ref()
  }

  pub fn reset_unlock_statuses(&mut self) {
    self.unlock_statuses = None;
  }

  pub fn set_failure_indexes(&mut self, failure_indexes: Vec<i32>) {
    self.failure_indexes = Some(failure_indexes);
  }

  pub fn with_failure_indexes(mut self, failure_indexes: Vec<i32>) -> DestinyEntitiesVendorsDestinyVendorSaleItemComponent {
    self.failure_indexes = Some(failure_indexes);
    self
  }

  pub fn failure_indexes(&self) -> Option<&Vec<i32>> {
    self.failure_indexes.as_ref()
  }

  pub fn reset_failure_indexes(&mut self) {
    self.failure_indexes = None;
  }

}



