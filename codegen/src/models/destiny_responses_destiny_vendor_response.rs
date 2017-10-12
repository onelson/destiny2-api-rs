/* 
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * OpenAPI spec version: 2.0.0
 * Contact: support@bungie.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DestinyResponsesDestinyVendorResponse : A response containing all of the components for a vendor.

#[derive(Debug, Serialize, Deserialize)]
pub struct DestinyResponsesDestinyVendorResponse {
  /// The base properties of the vendor.  COMPONENT TYPE: Vendors
  #[serde(rename = "vendor")]
  vendor: Option<Object>,
  /// Categories that the vendor has available, and references to the sales therein.  COMPONENT TYPE: VendorCategories
  #[serde(rename = "categories")]
  categories: Option<Object>,
  /// Sales, keyed by the vendorItemIndex of the item being sold.  COMPONENT TYPE: VendorSales
  #[serde(rename = "sales")]
  sales: Option<Object>,
  /// Item components, keyed by the vendorItemIndex of the active sale items.  COMPONENT TYPE: [See inside the DestinyItemComponentSet contract for component types.]
  #[serde(rename = "items")]
  items: Option<Object>
}

impl DestinyResponsesDestinyVendorResponse {
  /// A response containing all of the components for a vendor.
  pub fn new() -> DestinyResponsesDestinyVendorResponse {
    DestinyResponsesDestinyVendorResponse {
      vendor: None,
      categories: None,
      sales: None,
      items: None
    }
  }

  pub fn set_vendor(&mut self, vendor: Object) {
    self.vendor = Some(vendor);
  }

  pub fn with_vendor(mut self, vendor: Object) -> DestinyResponsesDestinyVendorResponse {
    self.vendor = Some(vendor);
    self
  }

  pub fn vendor(&self) -> Option<&Object> {
    self.vendor.as_ref()
  }

  pub fn reset_vendor(&mut self) {
    self.vendor = None;
  }

  pub fn set_categories(&mut self, categories: Object) {
    self.categories = Some(categories);
  }

  pub fn with_categories(mut self, categories: Object) -> DestinyResponsesDestinyVendorResponse {
    self.categories = Some(categories);
    self
  }

  pub fn categories(&self) -> Option<&Object> {
    self.categories.as_ref()
  }

  pub fn reset_categories(&mut self) {
    self.categories = None;
  }

  pub fn set_sales(&mut self, sales: Object) {
    self.sales = Some(sales);
  }

  pub fn with_sales(mut self, sales: Object) -> DestinyResponsesDestinyVendorResponse {
    self.sales = Some(sales);
    self
  }

  pub fn sales(&self) -> Option<&Object> {
    self.sales.as_ref()
  }

  pub fn reset_sales(&mut self) {
    self.sales = None;
  }

  pub fn set_items(&mut self, items: Object) {
    self.items = Some(items);
  }

  pub fn with_items(mut self, items: Object) -> DestinyResponsesDestinyVendorResponse {
    self.items = Some(items);
    self
  }

  pub fn items(&self) -> Option<&Object> {
    self.items.as_ref()
  }

  pub fn reset_items(&mut self) {
    self.items = None;
  }

}


