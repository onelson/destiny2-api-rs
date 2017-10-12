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
pub struct QueriesPagedQuery {
  #[serde(rename = "itemsPerPage")]
  items_per_page: Option<i32>,
  #[serde(rename = "currentPage")]
  current_page: Option<i32>,
  #[serde(rename = "requestContinuationToken")]
  request_continuation_token: Option<String>
}

impl QueriesPagedQuery {
  pub fn new() -> QueriesPagedQuery {
    QueriesPagedQuery {
      items_per_page: None,
      current_page: None,
      request_continuation_token: None
    }
  }

  pub fn set_items_per_page(&mut self, items_per_page: i32) {
    self.items_per_page = Some(items_per_page);
  }

  pub fn with_items_per_page(mut self, items_per_page: i32) -> QueriesPagedQuery {
    self.items_per_page = Some(items_per_page);
    self
  }

  pub fn items_per_page(&self) -> Option<&i32> {
    self.items_per_page.as_ref()
  }

  pub fn reset_items_per_page(&mut self) {
    self.items_per_page = None;
  }

  pub fn set_current_page(&mut self, current_page: i32) {
    self.current_page = Some(current_page);
  }

  pub fn with_current_page(mut self, current_page: i32) -> QueriesPagedQuery {
    self.current_page = Some(current_page);
    self
  }

  pub fn current_page(&self) -> Option<&i32> {
    self.current_page.as_ref()
  }

  pub fn reset_current_page(&mut self) {
    self.current_page = None;
  }

  pub fn set_request_continuation_token(&mut self, request_continuation_token: String) {
    self.request_continuation_token = Some(request_continuation_token);
  }

  pub fn with_request_continuation_token(mut self, request_continuation_token: String) -> QueriesPagedQuery {
    self.request_continuation_token = Some(request_continuation_token);
    self
  }

  pub fn request_continuation_token(&self) -> Option<&String> {
    self.request_continuation_token.as_ref()
  }

  pub fn reset_request_continuation_token(&mut self) {
    self.request_continuation_token = None;
  }

}



