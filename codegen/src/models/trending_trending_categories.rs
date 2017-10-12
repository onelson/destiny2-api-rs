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
pub struct TrendingTrendingCategories {
  #[serde(rename = "categories")]
  categories: Option<Vec<::models::TrendingTrendingCategory>>
}

impl TrendingTrendingCategories {
  pub fn new() -> TrendingTrendingCategories {
    TrendingTrendingCategories {
      categories: None
    }
  }

  pub fn set_categories(&mut self, categories: Vec<::models::TrendingTrendingCategory>) {
    self.categories = Some(categories);
  }

  pub fn with_categories(mut self, categories: Vec<::models::TrendingTrendingCategory>) -> TrendingTrendingCategories {
    self.categories = Some(categories);
    self
  }

  pub fn categories(&self) -> Option<&Vec<::models::TrendingTrendingCategory>> {
    self.categories.as_ref()
  }

  pub fn reset_categories(&mut self) {
    self.categories = None;
  }

}


