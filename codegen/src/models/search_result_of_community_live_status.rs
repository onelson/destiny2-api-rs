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
pub struct SearchResultOfCommunityLiveStatus {
  #[serde(rename = "results")]
  results: Option<Vec<::models::CommunityCommunityLiveStatus>>,
  #[serde(rename = "totalResults")]
  total_results: Option<i32>,
  #[serde(rename = "hasMore")]
  has_more: Option<bool>,
  #[serde(rename = "query")]
  query: Option<::models::QueriesPagedQuery>,
  #[serde(rename = "replacementContinuationToken")]
  replacement_continuation_token: Option<String>,
  /// If useTotalResults is true, then totalResults represents an accurate count.  If False, it does not, and may be estimated/only the size of the current page.  Either way, you should probably always only trust hasMore.  This is a long-held historical throwback to when we used to do paging with known total results. Those queries toasted our database, and we were left to hastily alter our endpoints and create backward- compatible shims, of which useTotalResults is one.
  #[serde(rename = "useTotalResults")]
  use_total_results: Option<bool>
}

impl SearchResultOfCommunityLiveStatus {
  pub fn new() -> SearchResultOfCommunityLiveStatus {
    SearchResultOfCommunityLiveStatus {
      results: None,
      total_results: None,
      has_more: None,
      query: None,
      replacement_continuation_token: None,
      use_total_results: None
    }
  }

  pub fn set_results(&mut self, results: Vec<::models::CommunityCommunityLiveStatus>) {
    self.results = Some(results);
  }

  pub fn with_results(mut self, results: Vec<::models::CommunityCommunityLiveStatus>) -> SearchResultOfCommunityLiveStatus {
    self.results = Some(results);
    self
  }

  pub fn results(&self) -> Option<&Vec<::models::CommunityCommunityLiveStatus>> {
    self.results.as_ref()
  }

  pub fn reset_results(&mut self) {
    self.results = None;
  }

  pub fn set_total_results(&mut self, total_results: i32) {
    self.total_results = Some(total_results);
  }

  pub fn with_total_results(mut self, total_results: i32) -> SearchResultOfCommunityLiveStatus {
    self.total_results = Some(total_results);
    self
  }

  pub fn total_results(&self) -> Option<&i32> {
    self.total_results.as_ref()
  }

  pub fn reset_total_results(&mut self) {
    self.total_results = None;
  }

  pub fn set_has_more(&mut self, has_more: bool) {
    self.has_more = Some(has_more);
  }

  pub fn with_has_more(mut self, has_more: bool) -> SearchResultOfCommunityLiveStatus {
    self.has_more = Some(has_more);
    self
  }

  pub fn has_more(&self) -> Option<&bool> {
    self.has_more.as_ref()
  }

  pub fn reset_has_more(&mut self) {
    self.has_more = None;
  }

  pub fn set_query(&mut self, query: ::models::QueriesPagedQuery) {
    self.query = Some(query);
  }

  pub fn with_query(mut self, query: ::models::QueriesPagedQuery) -> SearchResultOfCommunityLiveStatus {
    self.query = Some(query);
    self
  }

  pub fn query(&self) -> Option<&::models::QueriesPagedQuery> {
    self.query.as_ref()
  }

  pub fn reset_query(&mut self) {
    self.query = None;
  }

  pub fn set_replacement_continuation_token(&mut self, replacement_continuation_token: String) {
    self.replacement_continuation_token = Some(replacement_continuation_token);
  }

  pub fn with_replacement_continuation_token(mut self, replacement_continuation_token: String) -> SearchResultOfCommunityLiveStatus {
    self.replacement_continuation_token = Some(replacement_continuation_token);
    self
  }

  pub fn replacement_continuation_token(&self) -> Option<&String> {
    self.replacement_continuation_token.as_ref()
  }

  pub fn reset_replacement_continuation_token(&mut self) {
    self.replacement_continuation_token = None;
  }

  pub fn set_use_total_results(&mut self, use_total_results: bool) {
    self.use_total_results = Some(use_total_results);
  }

  pub fn with_use_total_results(mut self, use_total_results: bool) -> SearchResultOfCommunityLiveStatus {
    self.use_total_results = Some(use_total_results);
    self
  }

  pub fn use_total_results(&self) -> Option<&bool> {
    self.use_total_results.as_ref()
  }

  pub fn reset_use_total_results(&mut self) {
    self.use_total_results = None;
  }

}



