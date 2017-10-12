/* 
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * OpenAPI spec version: 2.0.0
 * Contact: support@bungie.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */


#[derive(Debug, Serialize, Deserialize)]
pub struct DestinyComponentsKiosksDestinyKioskItem {
  /// The index of the item in the related DestinyVendorDefintion's itemList property, representing the sale.
  #[serde(rename = "index")]
  index: Option<i32>,
  /// If true, the user can not only see the item, but they can acquire it. It is possible that a user can see a kiosk item and not be able to acquire it.
  #[serde(rename = "canAcquire")]
  can_acquire: Option<bool>,
  /// Indexes into failureStrings for the Vendor, indicating the reasons why it failed if any.
  #[serde(rename = "failureIndexes")]
  failure_indexes: Option<Vec<i32>>
}

impl DestinyComponentsKiosksDestinyKioskItem {
  pub fn new() -> DestinyComponentsKiosksDestinyKioskItem {
    DestinyComponentsKiosksDestinyKioskItem {
      index: None,
      can_acquire: None,
      failure_indexes: None
    }
  }

  pub fn set_index(&mut self, index: i32) {
    self.index = Some(index);
  }

  pub fn with_index(mut self, index: i32) -> DestinyComponentsKiosksDestinyKioskItem {
    self.index = Some(index);
    self
  }

  pub fn index(&self) -> Option<&i32> {
    self.index.as_ref()
  }

  pub fn reset_index(&mut self) {
    self.index = None;
  }

  pub fn set_can_acquire(&mut self, can_acquire: bool) {
    self.can_acquire = Some(can_acquire);
  }

  pub fn with_can_acquire(mut self, can_acquire: bool) -> DestinyComponentsKiosksDestinyKioskItem {
    self.can_acquire = Some(can_acquire);
    self
  }

  pub fn can_acquire(&self) -> Option<&bool> {
    self.can_acquire.as_ref()
  }

  pub fn reset_can_acquire(&mut self) {
    self.can_acquire = None;
  }

  pub fn set_failure_indexes(&mut self, failure_indexes: Vec<i32>) {
    self.failure_indexes = Some(failure_indexes);
  }

  pub fn with_failure_indexes(mut self, failure_indexes: Vec<i32>) -> DestinyComponentsKiosksDestinyKioskItem {
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



