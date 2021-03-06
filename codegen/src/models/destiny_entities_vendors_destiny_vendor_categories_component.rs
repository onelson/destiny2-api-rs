/* 
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * OpenAPI spec version: 2.0.0
 * Contact: support@bungie.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DestinyEntitiesVendorsDestinyVendorCategoriesComponent : A vendor can have many categories of items that they sell. This component will return the category information for available items, as well as the index into those items in the user's sale item list.  Note that, since both the category and items are indexes, this data is Content Version dependent. Be sure to check that your content is up to date before using this data. This is an unfortunate, but permanent, limitation of Vendor data.

#[derive(Debug, Serialize, Deserialize)]
pub struct DestinyEntitiesVendorsDestinyVendorCategoriesComponent {
  /// The list of categories for items that the vendor sells, in rendering order.
  #[serde(rename = "categories")]
  categories: Option<Vec<::models::DestinyEntitiesVendorsDestinyVendorCategory>>
}

impl DestinyEntitiesVendorsDestinyVendorCategoriesComponent {
  /// A vendor can have many categories of items that they sell. This component will return the category information for available items, as well as the index into those items in the user's sale item list.  Note that, since both the category and items are indexes, this data is Content Version dependent. Be sure to check that your content is up to date before using this data. This is an unfortunate, but permanent, limitation of Vendor data.
  pub fn new() -> DestinyEntitiesVendorsDestinyVendorCategoriesComponent {
    DestinyEntitiesVendorsDestinyVendorCategoriesComponent {
      categories: None
    }
  }

  pub fn set_categories(&mut self, categories: Vec<::models::DestinyEntitiesVendorsDestinyVendorCategory>) {
    self.categories = Some(categories);
  }

  pub fn with_categories(mut self, categories: Vec<::models::DestinyEntitiesVendorsDestinyVendorCategory>) -> DestinyEntitiesVendorsDestinyVendorCategoriesComponent {
    self.categories = Some(categories);
    self
  }

  pub fn categories(&self) -> Option<&Vec<::models::DestinyEntitiesVendorsDestinyVendorCategory>> {
    self.categories.as_ref()
  }

  pub fn reset_categories(&mut self) {
    self.categories = None;
  }

}



