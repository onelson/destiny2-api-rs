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
pub struct SingleComponentResponseOfDestinyItemStatsComponent {
  #[serde(rename = "data")]
  data: Option<::models::DestinyEntitiesItemsDestinyItemStatsComponent>,
  #[serde(rename = "privacy")]
  privacy: Option<::models::ComponentsComponentPrivacySetting>
}

impl SingleComponentResponseOfDestinyItemStatsComponent {
  pub fn new() -> SingleComponentResponseOfDestinyItemStatsComponent {
    SingleComponentResponseOfDestinyItemStatsComponent {
      data: None,
      privacy: None
    }
  }

  pub fn set_data(&mut self, data: ::models::DestinyEntitiesItemsDestinyItemStatsComponent) {
    self.data = Some(data);
  }

  pub fn with_data(mut self, data: ::models::DestinyEntitiesItemsDestinyItemStatsComponent) -> SingleComponentResponseOfDestinyItemStatsComponent {
    self.data = Some(data);
    self
  }

  pub fn data(&self) -> Option<&::models::DestinyEntitiesItemsDestinyItemStatsComponent> {
    self.data.as_ref()
  }

  pub fn reset_data(&mut self) {
    self.data = None;
  }

  pub fn set_privacy(&mut self, privacy: ::models::ComponentsComponentPrivacySetting) {
    self.privacy = Some(privacy);
  }

  pub fn with_privacy(mut self, privacy: ::models::ComponentsComponentPrivacySetting) -> SingleComponentResponseOfDestinyItemStatsComponent {
    self.privacy = Some(privacy);
    self
  }

  pub fn privacy(&self) -> Option<&::models::ComponentsComponentPrivacySetting> {
    self.privacy.as_ref()
  }

  pub fn reset_privacy(&mut self) {
    self.privacy = None;
  }

}



