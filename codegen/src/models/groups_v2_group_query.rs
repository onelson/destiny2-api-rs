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
pub struct GroupsV2GroupQuery {
  #[serde(rename = "name")]
  name: Option<String>,
  #[serde(rename = "groupType")]
  group_type: Option<::models::GroupsV2GroupType>,
  #[serde(rename = "creationDate")]
  creation_date: Option<::models::GroupsV2GroupDateRange>,
  #[serde(rename = "sortBy")]
  sort_by: Option<::models::GroupsV2GroupSortBy>,
  #[serde(rename = "groupMemberCountFilter")]
  group_member_count_filter: Option<i32>,
  #[serde(rename = "localeFilter")]
  locale_filter: Option<String>,
  #[serde(rename = "tagText")]
  tag_text: Option<String>,
  #[serde(rename = "itemsPerPage")]
  items_per_page: Option<i32>,
  #[serde(rename = "currentPage")]
  current_page: Option<i32>,
  #[serde(rename = "requestContinuationToken")]
  request_continuation_token: Option<String>
}

impl GroupsV2GroupQuery {
  pub fn new() -> GroupsV2GroupQuery {
    GroupsV2GroupQuery {
      name: None,
      group_type: None,
      creation_date: None,
      sort_by: None,
      group_member_count_filter: None,
      locale_filter: None,
      tag_text: None,
      items_per_page: None,
      current_page: None,
      request_continuation_token: None
    }
  }

  pub fn set_name(&mut self, name: String) {
    self.name = Some(name);
  }

  pub fn with_name(mut self, name: String) -> GroupsV2GroupQuery {
    self.name = Some(name);
    self
  }

  pub fn name(&self) -> Option<&String> {
    self.name.as_ref()
  }

  pub fn reset_name(&mut self) {
    self.name = None;
  }

  pub fn set_group_type(&mut self, group_type: ::models::GroupsV2GroupType) {
    self.group_type = Some(group_type);
  }

  pub fn with_group_type(mut self, group_type: ::models::GroupsV2GroupType) -> GroupsV2GroupQuery {
    self.group_type = Some(group_type);
    self
  }

  pub fn group_type(&self) -> Option<&::models::GroupsV2GroupType> {
    self.group_type.as_ref()
  }

  pub fn reset_group_type(&mut self) {
    self.group_type = None;
  }

  pub fn set_creation_date(&mut self, creation_date: ::models::GroupsV2GroupDateRange) {
    self.creation_date = Some(creation_date);
  }

  pub fn with_creation_date(mut self, creation_date: ::models::GroupsV2GroupDateRange) -> GroupsV2GroupQuery {
    self.creation_date = Some(creation_date);
    self
  }

  pub fn creation_date(&self) -> Option<&::models::GroupsV2GroupDateRange> {
    self.creation_date.as_ref()
  }

  pub fn reset_creation_date(&mut self) {
    self.creation_date = None;
  }

  pub fn set_sort_by(&mut self, sort_by: ::models::GroupsV2GroupSortBy) {
    self.sort_by = Some(sort_by);
  }

  pub fn with_sort_by(mut self, sort_by: ::models::GroupsV2GroupSortBy) -> GroupsV2GroupQuery {
    self.sort_by = Some(sort_by);
    self
  }

  pub fn sort_by(&self) -> Option<&::models::GroupsV2GroupSortBy> {
    self.sort_by.as_ref()
  }

  pub fn reset_sort_by(&mut self) {
    self.sort_by = None;
  }

  pub fn set_group_member_count_filter(&mut self, group_member_count_filter: i32) {
    self.group_member_count_filter = Some(group_member_count_filter);
  }

  pub fn with_group_member_count_filter(mut self, group_member_count_filter: i32) -> GroupsV2GroupQuery {
    self.group_member_count_filter = Some(group_member_count_filter);
    self
  }

  pub fn group_member_count_filter(&self) -> Option<&i32> {
    self.group_member_count_filter.as_ref()
  }

  pub fn reset_group_member_count_filter(&mut self) {
    self.group_member_count_filter = None;
  }

  pub fn set_locale_filter(&mut self, locale_filter: String) {
    self.locale_filter = Some(locale_filter);
  }

  pub fn with_locale_filter(mut self, locale_filter: String) -> GroupsV2GroupQuery {
    self.locale_filter = Some(locale_filter);
    self
  }

  pub fn locale_filter(&self) -> Option<&String> {
    self.locale_filter.as_ref()
  }

  pub fn reset_locale_filter(&mut self) {
    self.locale_filter = None;
  }

  pub fn set_tag_text(&mut self, tag_text: String) {
    self.tag_text = Some(tag_text);
  }

  pub fn with_tag_text(mut self, tag_text: String) -> GroupsV2GroupQuery {
    self.tag_text = Some(tag_text);
    self
  }

  pub fn tag_text(&self) -> Option<&String> {
    self.tag_text.as_ref()
  }

  pub fn reset_tag_text(&mut self) {
    self.tag_text = None;
  }

  pub fn set_items_per_page(&mut self, items_per_page: i32) {
    self.items_per_page = Some(items_per_page);
  }

  pub fn with_items_per_page(mut self, items_per_page: i32) -> GroupsV2GroupQuery {
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

  pub fn with_current_page(mut self, current_page: i32) -> GroupsV2GroupQuery {
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

  pub fn with_request_continuation_token(mut self, request_continuation_token: String) -> GroupsV2GroupQuery {
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



