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
pub struct DictionaryComponentResponseOfint64AndDestinyCharacterActivitiesComponent {
  #[serde(rename = "data")]
  data: Option<::std::collections::HashMap<String, ::models::DestinyEntitiesCharactersDestinyCharacterActivitiesComponent>>,
  #[serde(rename = "privacy")]
  privacy: Option<::models::ComponentsComponentPrivacySetting>
}

impl DictionaryComponentResponseOfint64AndDestinyCharacterActivitiesComponent {
  pub fn new() -> DictionaryComponentResponseOfint64AndDestinyCharacterActivitiesComponent {
    DictionaryComponentResponseOfint64AndDestinyCharacterActivitiesComponent {
      data: None,
      privacy: None
    }
  }

  pub fn set_data(&mut self, data: ::std::collections::HashMap<String, ::models::DestinyEntitiesCharactersDestinyCharacterActivitiesComponent>) {
    self.data = Some(data);
  }

  pub fn with_data(mut self, data: ::std::collections::HashMap<String, ::models::DestinyEntitiesCharactersDestinyCharacterActivitiesComponent>) -> DictionaryComponentResponseOfint64AndDestinyCharacterActivitiesComponent {
    self.data = Some(data);
    self
  }

  pub fn data(&self) -> Option<&::std::collections::HashMap<String, ::models::DestinyEntitiesCharactersDestinyCharacterActivitiesComponent>> {
    self.data.as_ref()
  }

  pub fn reset_data(&mut self) {
    self.data = None;
  }

  pub fn set_privacy(&mut self, privacy: ::models::ComponentsComponentPrivacySetting) {
    self.privacy = Some(privacy);
  }

  pub fn with_privacy(mut self, privacy: ::models::ComponentsComponentPrivacySetting) -> DictionaryComponentResponseOfint64AndDestinyCharacterActivitiesComponent {
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



