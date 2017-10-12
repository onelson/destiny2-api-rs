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
pub struct UserModelsUserAlias {
  #[serde(rename = "cachePrefix")]
  cache_prefix: Option<String>
}

impl UserModelsUserAlias {
  pub fn new() -> UserModelsUserAlias {
    UserModelsUserAlias {
      cache_prefix: None
    }
  }

  pub fn set_cache_prefix(&mut self, cache_prefix: String) {
    self.cache_prefix = Some(cache_prefix);
  }

  pub fn with_cache_prefix(mut self, cache_prefix: String) -> UserModelsUserAlias {
    self.cache_prefix = Some(cache_prefix);
    self
  }

  pub fn cache_prefix(&self) -> Option<&String> {
    self.cache_prefix.as_ref()
  }

  pub fn reset_cache_prefix(&mut self) {
    self.cache_prefix = None;
  }

}


