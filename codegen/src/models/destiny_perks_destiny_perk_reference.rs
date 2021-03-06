/* 
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * OpenAPI spec version: 2.0.0
 * Contact: support@bungie.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DestinyPerksDestinyPerkReference : The list of perks to display in an item tooltip - and whether or not they have been activated.  Perks apply a variety of effects to a character, and are generally either intrinsic to the item or provided in activated talent nodes or sockets.

#[derive(Debug, Serialize, Deserialize)]
pub struct DestinyPerksDestinyPerkReference {
  /// The hash identifier for the perk, which can be used to look up DestinySandboxPerkDefinition if it exists. Be warned, perks frequently do not have user-viewable information. You should examine whether you actually found a name/description in the perk's definition before you show it to the user.
  #[serde(rename = "perkHash")]
  perk_hash: Option<i32>,
  /// The icon for the perk.
  #[serde(rename = "iconPath")]
  icon_path: Option<String>,
  /// Whether this perk is currently active. (We may return perks that you have not actually activated yet: these represent perks that you should show in the item's tooltip, but that the user has not yet activated.)
  #[serde(rename = "isActive")]
  is_active: Option<bool>,
  /// Some perks provide benefits, but aren't visible in the UI. This value will let you know if this is perk should be shown in your UI.
  #[serde(rename = "visible")]
  visible: Option<bool>
}

impl DestinyPerksDestinyPerkReference {
  /// The list of perks to display in an item tooltip - and whether or not they have been activated.  Perks apply a variety of effects to a character, and are generally either intrinsic to the item or provided in activated talent nodes or sockets.
  pub fn new() -> DestinyPerksDestinyPerkReference {
    DestinyPerksDestinyPerkReference {
      perk_hash: None,
      icon_path: None,
      is_active: None,
      visible: None
    }
  }

  pub fn set_perk_hash(&mut self, perk_hash: i32) {
    self.perk_hash = Some(perk_hash);
  }

  pub fn with_perk_hash(mut self, perk_hash: i32) -> DestinyPerksDestinyPerkReference {
    self.perk_hash = Some(perk_hash);
    self
  }

  pub fn perk_hash(&self) -> Option<&i32> {
    self.perk_hash.as_ref()
  }

  pub fn reset_perk_hash(&mut self) {
    self.perk_hash = None;
  }

  pub fn set_icon_path(&mut self, icon_path: String) {
    self.icon_path = Some(icon_path);
  }

  pub fn with_icon_path(mut self, icon_path: String) -> DestinyPerksDestinyPerkReference {
    self.icon_path = Some(icon_path);
    self
  }

  pub fn icon_path(&self) -> Option<&String> {
    self.icon_path.as_ref()
  }

  pub fn reset_icon_path(&mut self) {
    self.icon_path = None;
  }

  pub fn set_is_active(&mut self, is_active: bool) {
    self.is_active = Some(is_active);
  }

  pub fn with_is_active(mut self, is_active: bool) -> DestinyPerksDestinyPerkReference {
    self.is_active = Some(is_active);
    self
  }

  pub fn is_active(&self) -> Option<&bool> {
    self.is_active.as_ref()
  }

  pub fn reset_is_active(&mut self) {
    self.is_active = None;
  }

  pub fn set_visible(&mut self, visible: bool) {
    self.visible = Some(visible);
  }

  pub fn with_visible(mut self, visible: bool) -> DestinyPerksDestinyPerkReference {
    self.visible = Some(visible);
    self
  }

  pub fn visible(&self) -> Option<&bool> {
    self.visible.as_ref()
  }

  pub fn reset_visible(&mut self) {
    self.visible = None;
  }

}



