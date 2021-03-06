/* 
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * OpenAPI spec version: 2.0.0
 * Contact: support@bungie.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DestinyComponentsKiosksDestinyKiosksComponent : A Kiosk is a Vendor (DestinyVendorDefinition) that sells items based on whether you have already acquired that item before.  This component returns information about what Kiosk items are available to you on a *Profile* level. It is theoretically possible for Kiosks to have items gated by specific Character as well. If you ever have those, you will find them on the individual character's DestinyCharacterKiosksComponent.  Note that, because this component returns vendorItemIndexes (that is to say, indexes into the Kiosk Vendor's itemList property), these results are necessarily content version dependent. Make sure that you have the latest version of the content manifest databases before using this data.

#[derive(Debug, Serialize, Deserialize)]
pub struct DestinyComponentsKiosksDestinyKiosksComponent {
  /// A dictionary keyed by the Kiosk Vendor's hash identifier (use it to look up the DestinyVendorDefinition for the relevant kiosk vendor), and whose value is a list of all the items that the user can \"see\" in the Kiosk, and any other interesting metadata.
  #[serde(rename = "kioskItems")]
  kiosk_items: Option<::std::collections::HashMap<String, Vec<::models::DestinyComponentsKiosksDestinyKioskItem>>>
}

impl DestinyComponentsKiosksDestinyKiosksComponent {
  /// A Kiosk is a Vendor (DestinyVendorDefinition) that sells items based on whether you have already acquired that item before.  This component returns information about what Kiosk items are available to you on a *Profile* level. It is theoretically possible for Kiosks to have items gated by specific Character as well. If you ever have those, you will find them on the individual character's DestinyCharacterKiosksComponent.  Note that, because this component returns vendorItemIndexes (that is to say, indexes into the Kiosk Vendor's itemList property), these results are necessarily content version dependent. Make sure that you have the latest version of the content manifest databases before using this data.
  pub fn new() -> DestinyComponentsKiosksDestinyKiosksComponent {
    DestinyComponentsKiosksDestinyKiosksComponent {
      kiosk_items: None
    }
  }

  pub fn set_kiosk_items(&mut self, kiosk_items: ::std::collections::HashMap<String, Vec<::models::DestinyComponentsKiosksDestinyKioskItem>>) {
    self.kiosk_items = Some(kiosk_items);
  }

  pub fn with_kiosk_items(mut self, kiosk_items: ::std::collections::HashMap<String, Vec<::models::DestinyComponentsKiosksDestinyKioskItem>>) -> DestinyComponentsKiosksDestinyKiosksComponent {
    self.kiosk_items = Some(kiosk_items);
    self
  }

  pub fn kiosk_items(&self) -> Option<&::std::collections::HashMap<String, Vec<::models::DestinyComponentsKiosksDestinyKioskItem>>> {
    self.kiosk_items.as_ref()
  }

  pub fn reset_kiosk_items(&mut self) {
    self.kiosk_items = None;
  }

}



