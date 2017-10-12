/* 
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * OpenAPI spec version: 2.0.0
 * Contact: support@bungie.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DestinyConfigDestinyManifest : DestinyManifest is the external-facing contract for just the properties needed by those calling the Destiny Platform.

#[derive(Debug, Serialize, Deserialize)]
pub struct DestinyConfigDestinyManifest {
  #[serde(rename = "version")]
  version: Option<String>,
  #[serde(rename = "mobileAssetContentPath")]
  mobile_asset_content_path: Option<String>,
  #[serde(rename = "mobileGearAssetDataBases")]
  mobile_gear_asset_data_bases: Option<Vec<::models::DestinyConfigGearAssetDataBaseDefinition>>,
  #[serde(rename = "mobileWorldContentPaths")]
  mobile_world_content_paths: Option<::std::collections::HashMap<String, String>>,
  #[serde(rename = "mobileClanBannerDatabasePath")]
  mobile_clan_banner_database_path: Option<String>,
  #[serde(rename = "mobileGearCDN")]
  mobile_gear_cdn: Option<::std::collections::HashMap<String, String>>
}

impl DestinyConfigDestinyManifest {
  /// DestinyManifest is the external-facing contract for just the properties needed by those calling the Destiny Platform.
  pub fn new() -> DestinyConfigDestinyManifest {
    DestinyConfigDestinyManifest {
      version: None,
      mobile_asset_content_path: None,
      mobile_gear_asset_data_bases: None,
      mobile_world_content_paths: None,
      mobile_clan_banner_database_path: None,
      mobile_gear_cdn: None
    }
  }

  pub fn set_version(&mut self, version: String) {
    self.version = Some(version);
  }

  pub fn with_version(mut self, version: String) -> DestinyConfigDestinyManifest {
    self.version = Some(version);
    self
  }

  pub fn version(&self) -> Option<&String> {
    self.version.as_ref()
  }

  pub fn reset_version(&mut self) {
    self.version = None;
  }

  pub fn set_mobile_asset_content_path(&mut self, mobile_asset_content_path: String) {
    self.mobile_asset_content_path = Some(mobile_asset_content_path);
  }

  pub fn with_mobile_asset_content_path(mut self, mobile_asset_content_path: String) -> DestinyConfigDestinyManifest {
    self.mobile_asset_content_path = Some(mobile_asset_content_path);
    self
  }

  pub fn mobile_asset_content_path(&self) -> Option<&String> {
    self.mobile_asset_content_path.as_ref()
  }

  pub fn reset_mobile_asset_content_path(&mut self) {
    self.mobile_asset_content_path = None;
  }

  pub fn set_mobile_gear_asset_data_bases(&mut self, mobile_gear_asset_data_bases: Vec<::models::DestinyConfigGearAssetDataBaseDefinition>) {
    self.mobile_gear_asset_data_bases = Some(mobile_gear_asset_data_bases);
  }

  pub fn with_mobile_gear_asset_data_bases(mut self, mobile_gear_asset_data_bases: Vec<::models::DestinyConfigGearAssetDataBaseDefinition>) -> DestinyConfigDestinyManifest {
    self.mobile_gear_asset_data_bases = Some(mobile_gear_asset_data_bases);
    self
  }

  pub fn mobile_gear_asset_data_bases(&self) -> Option<&Vec<::models::DestinyConfigGearAssetDataBaseDefinition>> {
    self.mobile_gear_asset_data_bases.as_ref()
  }

  pub fn reset_mobile_gear_asset_data_bases(&mut self) {
    self.mobile_gear_asset_data_bases = None;
  }

  pub fn set_mobile_world_content_paths(&mut self, mobile_world_content_paths: ::std::collections::HashMap<String, String>) {
    self.mobile_world_content_paths = Some(mobile_world_content_paths);
  }

  pub fn with_mobile_world_content_paths(mut self, mobile_world_content_paths: ::std::collections::HashMap<String, String>) -> DestinyConfigDestinyManifest {
    self.mobile_world_content_paths = Some(mobile_world_content_paths);
    self
  }

  pub fn mobile_world_content_paths(&self) -> Option<&::std::collections::HashMap<String, String>> {
    self.mobile_world_content_paths.as_ref()
  }

  pub fn reset_mobile_world_content_paths(&mut self) {
    self.mobile_world_content_paths = None;
  }

  pub fn set_mobile_clan_banner_database_path(&mut self, mobile_clan_banner_database_path: String) {
    self.mobile_clan_banner_database_path = Some(mobile_clan_banner_database_path);
  }

  pub fn with_mobile_clan_banner_database_path(mut self, mobile_clan_banner_database_path: String) -> DestinyConfigDestinyManifest {
    self.mobile_clan_banner_database_path = Some(mobile_clan_banner_database_path);
    self
  }

  pub fn mobile_clan_banner_database_path(&self) -> Option<&String> {
    self.mobile_clan_banner_database_path.as_ref()
  }

  pub fn reset_mobile_clan_banner_database_path(&mut self) {
    self.mobile_clan_banner_database_path = None;
  }

  pub fn set_mobile_gear_cdn(&mut self, mobile_gear_cdn: ::std::collections::HashMap<String, String>) {
    self.mobile_gear_cdn = Some(mobile_gear_cdn);
  }

  pub fn with_mobile_gear_cdn(mut self, mobile_gear_cdn: ::std::collections::HashMap<String, String>) -> DestinyConfigDestinyManifest {
    self.mobile_gear_cdn = Some(mobile_gear_cdn);
    self
  }

  pub fn mobile_gear_cdn(&self) -> Option<&::std::collections::HashMap<String, String>> {
    self.mobile_gear_cdn.as_ref()
  }

  pub fn reset_mobile_gear_cdn(&mut self) {
    self.mobile_gear_cdn = None;
  }

}


