/* 
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * OpenAPI spec version: 2.0.0
 * Contact: support@bungie.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DestinyDefinitionsCommonDestinyDisplayPropertiesDefinition : Many Destiny*Definition contracts - the \"first order\" entities of Destiny that have their own tables in the Manifest Database - also have displayable information. This is the base class for that display information.

#[derive(Debug, Serialize, Deserialize)]
pub struct DestinyDefinitionsCommonDestinyDisplayPropertiesDefinition {
  #[serde(rename = "description")]
  description: Option<String>,
  #[serde(rename = "name")]
  name: Option<String>,
  /// Note that \"icon\" is sometimes misleading, and should be interpreted in the context of the entity. For instance, in Destiny 1 the DestinyRecordBookDefinition's icon was a big picture of a book.  But usually, it will be a small square image that you can use as... well, an icon.
  #[serde(rename = "icon")]
  icon: Option<String>,
  #[serde(rename = "hasIcon")]
  has_icon: Option<bool>
}

impl DestinyDefinitionsCommonDestinyDisplayPropertiesDefinition {
  /// Many Destiny*Definition contracts - the \"first order\" entities of Destiny that have their own tables in the Manifest Database - also have displayable information. This is the base class for that display information.
  pub fn new() -> DestinyDefinitionsCommonDestinyDisplayPropertiesDefinition {
    DestinyDefinitionsCommonDestinyDisplayPropertiesDefinition {
      description: None,
      name: None,
      icon: None,
      has_icon: None
    }
  }

  pub fn set_description(&mut self, description: String) {
    self.description = Some(description);
  }

  pub fn with_description(mut self, description: String) -> DestinyDefinitionsCommonDestinyDisplayPropertiesDefinition {
    self.description = Some(description);
    self
  }

  pub fn description(&self) -> Option<&String> {
    self.description.as_ref()
  }

  pub fn reset_description(&mut self) {
    self.description = None;
  }

  pub fn set_name(&mut self, name: String) {
    self.name = Some(name);
  }

  pub fn with_name(mut self, name: String) -> DestinyDefinitionsCommonDestinyDisplayPropertiesDefinition {
    self.name = Some(name);
    self
  }

  pub fn name(&self) -> Option<&String> {
    self.name.as_ref()
  }

  pub fn reset_name(&mut self) {
    self.name = None;
  }

  pub fn set_icon(&mut self, icon: String) {
    self.icon = Some(icon);
  }

  pub fn with_icon(mut self, icon: String) -> DestinyDefinitionsCommonDestinyDisplayPropertiesDefinition {
    self.icon = Some(icon);
    self
  }

  pub fn icon(&self) -> Option<&String> {
    self.icon.as_ref()
  }

  pub fn reset_icon(&mut self) {
    self.icon = None;
  }

  pub fn set_has_icon(&mut self, has_icon: bool) {
    self.has_icon = Some(has_icon);
  }

  pub fn with_has_icon(mut self, has_icon: bool) -> DestinyDefinitionsCommonDestinyDisplayPropertiesDefinition {
    self.has_icon = Some(has_icon);
    self
  }

  pub fn has_icon(&self) -> Option<&bool> {
    self.has_icon.as_ref()
  }

  pub fn reset_has_icon(&mut self) {
    self.has_icon = None;
  }

}


