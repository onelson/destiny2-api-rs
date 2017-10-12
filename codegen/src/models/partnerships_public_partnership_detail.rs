/* 
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * OpenAPI spec version: 2.0.0
 * Contact: support@bungie.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// PartnershipsPublicPartnershipDetail : All the partnership info that's fit to expose externally, if we care to do so.

#[derive(Debug, Serialize, Deserialize)]
pub struct PartnershipsPublicPartnershipDetail {
  #[serde(rename = "partnerType")]
  partner_type: Option<::models::PartnershipsPartnershipType>,
  #[serde(rename = "identifier")]
  identifier: Option<String>,
  #[serde(rename = "name")]
  name: Option<String>,
  #[serde(rename = "icon")]
  icon: Option<String>
}

impl PartnershipsPublicPartnershipDetail {
  /// All the partnership info that's fit to expose externally, if we care to do so.
  pub fn new() -> PartnershipsPublicPartnershipDetail {
    PartnershipsPublicPartnershipDetail {
      partner_type: None,
      identifier: None,
      name: None,
      icon: None
    }
  }

  pub fn set_partner_type(&mut self, partner_type: ::models::PartnershipsPartnershipType) {
    self.partner_type = Some(partner_type);
  }

  pub fn with_partner_type(mut self, partner_type: ::models::PartnershipsPartnershipType) -> PartnershipsPublicPartnershipDetail {
    self.partner_type = Some(partner_type);
    self
  }

  pub fn partner_type(&self) -> Option<&::models::PartnershipsPartnershipType> {
    self.partner_type.as_ref()
  }

  pub fn reset_partner_type(&mut self) {
    self.partner_type = None;
  }

  pub fn set_identifier(&mut self, identifier: String) {
    self.identifier = Some(identifier);
  }

  pub fn with_identifier(mut self, identifier: String) -> PartnershipsPublicPartnershipDetail {
    self.identifier = Some(identifier);
    self
  }

  pub fn identifier(&self) -> Option<&String> {
    self.identifier.as_ref()
  }

  pub fn reset_identifier(&mut self) {
    self.identifier = None;
  }

  pub fn set_name(&mut self, name: String) {
    self.name = Some(name);
  }

  pub fn with_name(mut self, name: String) -> PartnershipsPublicPartnershipDetail {
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

  pub fn with_icon(mut self, icon: String) -> PartnershipsPublicPartnershipDetail {
    self.icon = Some(icon);
    self
  }

  pub fn icon(&self) -> Option<&String> {
    self.icon.as_ref()
  }

  pub fn reset_icon(&mut self) {
    self.icon = None;
  }

}



