/* 
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * OpenAPI spec version: 2.0.0
 * Contact: support@bungie.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DestinyDefinitionsDestinyGenderDefinition : Gender is a social construct, and as such we have definitions for Genders. Right now there happens to only be two, but we'll see what the future holds.

#[derive(Debug, Serialize, Deserialize)]
pub struct DestinyDefinitionsDestinyGenderDefinition {
  /// This is a quick reference enumeration for all of the currently defined Genders. We use the enumeration for quicker lookups in related data, like DestinyClassDefinition.genderedClassNames.
  #[serde(rename = "genderType")]
  gender_type: Option<Object>,
  #[serde(rename = "displayProperties")]
  display_properties: Option<::models::DestinyDefinitionsCommonDestinyDisplayPropertiesDefinition>,
  /// The unique identifier for this entity. Guaranteed to be unique for the type of entity, but not globally.  When entities refer to each other in Destiny content, it is this hash that they are referring to.
  #[serde(rename = "hash")]
  hash: Option<i32>,
  /// The index of the entity as it was found in the investment tables.
  #[serde(rename = "index")]
  index: Option<i32>,
  /// If this is true, then there is an entity with this identifier/type combination, but BNet is not yet allowed to show it. Sorry!
  #[serde(rename = "redacted")]
  redacted: Option<bool>
}

impl DestinyDefinitionsDestinyGenderDefinition {
  /// Gender is a social construct, and as such we have definitions for Genders. Right now there happens to only be two, but we'll see what the future holds.
  pub fn new() -> DestinyDefinitionsDestinyGenderDefinition {
    DestinyDefinitionsDestinyGenderDefinition {
      gender_type: None,
      display_properties: None,
      hash: None,
      index: None,
      redacted: None
    }
  }

  pub fn set_gender_type(&mut self, gender_type: Object) {
    self.gender_type = Some(gender_type);
  }

  pub fn with_gender_type(mut self, gender_type: Object) -> DestinyDefinitionsDestinyGenderDefinition {
    self.gender_type = Some(gender_type);
    self
  }

  pub fn gender_type(&self) -> Option<&Object> {
    self.gender_type.as_ref()
  }

  pub fn reset_gender_type(&mut self) {
    self.gender_type = None;
  }

  pub fn set_display_properties(&mut self, display_properties: ::models::DestinyDefinitionsCommonDestinyDisplayPropertiesDefinition) {
    self.display_properties = Some(display_properties);
  }

  pub fn with_display_properties(mut self, display_properties: ::models::DestinyDefinitionsCommonDestinyDisplayPropertiesDefinition) -> DestinyDefinitionsDestinyGenderDefinition {
    self.display_properties = Some(display_properties);
    self
  }

  pub fn display_properties(&self) -> Option<&::models::DestinyDefinitionsCommonDestinyDisplayPropertiesDefinition> {
    self.display_properties.as_ref()
  }

  pub fn reset_display_properties(&mut self) {
    self.display_properties = None;
  }

  pub fn set_hash(&mut self, hash: i32) {
    self.hash = Some(hash);
  }

  pub fn with_hash(mut self, hash: i32) -> DestinyDefinitionsDestinyGenderDefinition {
    self.hash = Some(hash);
    self
  }

  pub fn hash(&self) -> Option<&i32> {
    self.hash.as_ref()
  }

  pub fn reset_hash(&mut self) {
    self.hash = None;
  }

  pub fn set_index(&mut self, index: i32) {
    self.index = Some(index);
  }

  pub fn with_index(mut self, index: i32) -> DestinyDefinitionsDestinyGenderDefinition {
    self.index = Some(index);
    self
  }

  pub fn index(&self) -> Option<&i32> {
    self.index.as_ref()
  }

  pub fn reset_index(&mut self) {
    self.index = None;
  }

  pub fn set_redacted(&mut self, redacted: bool) {
    self.redacted = Some(redacted);
  }

  pub fn with_redacted(mut self, redacted: bool) -> DestinyDefinitionsDestinyGenderDefinition {
    self.redacted = Some(redacted);
    self
  }

  pub fn redacted(&self) -> Option<&bool> {
    self.redacted.as_ref()
  }

  pub fn reset_redacted(&mut self) {
    self.redacted = None;
  }

}



