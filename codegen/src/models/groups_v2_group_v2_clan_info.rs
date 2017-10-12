/* 
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * OpenAPI spec version: 2.0.0
 * Contact: support@bungie.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GroupsV2GroupV2ClanInfo : This contract contains clan-specific group information. It does not include any investment data.

#[derive(Debug, Serialize, Deserialize)]
pub struct GroupsV2GroupV2ClanInfo {
  #[serde(rename = "clanCallsign")]
  clan_callsign: Option<String>,
  #[serde(rename = "clanBannerData")]
  clan_banner_data: Option<::models::GroupsV2ClanBanner>
}

impl GroupsV2GroupV2ClanInfo {
  /// This contract contains clan-specific group information. It does not include any investment data.
  pub fn new() -> GroupsV2GroupV2ClanInfo {
    GroupsV2GroupV2ClanInfo {
      clan_callsign: None,
      clan_banner_data: None
    }
  }

  pub fn set_clan_callsign(&mut self, clan_callsign: String) {
    self.clan_callsign = Some(clan_callsign);
  }

  pub fn with_clan_callsign(mut self, clan_callsign: String) -> GroupsV2GroupV2ClanInfo {
    self.clan_callsign = Some(clan_callsign);
    self
  }

  pub fn clan_callsign(&self) -> Option<&String> {
    self.clan_callsign.as_ref()
  }

  pub fn reset_clan_callsign(&mut self) {
    self.clan_callsign = None;
  }

  pub fn set_clan_banner_data(&mut self, clan_banner_data: ::models::GroupsV2ClanBanner) {
    self.clan_banner_data = Some(clan_banner_data);
  }

  pub fn with_clan_banner_data(mut self, clan_banner_data: ::models::GroupsV2ClanBanner) -> GroupsV2GroupV2ClanInfo {
    self.clan_banner_data = Some(clan_banner_data);
    self
  }

  pub fn clan_banner_data(&self) -> Option<&::models::GroupsV2ClanBanner> {
    self.clan_banner_data.as_ref()
  }

  pub fn reset_clan_banner_data(&mut self) {
    self.clan_banner_data = None;
  }

}



