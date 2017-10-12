/* 
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * OpenAPI spec version: 2.0.0
 * Contact: support@bungie.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DestinyDefinitionsDestinyTalentNodeStepGroups : These properties are an attempt to categorize talent node steps by certain common properties. See the related enumerations for the type of properties being categorized.

#[derive(Debug, Serialize, Deserialize)]
pub struct DestinyDefinitionsDestinyTalentNodeStepGroups {
  #[serde(rename = "weaponPerformance")]
  weapon_performance: Option<::models::DestinyDefinitionsDestinyTalentNodeStepWeaponPerformances>,
  #[serde(rename = "impactEffects")]
  impact_effects: Option<::models::DestinyDefinitionsDestinyTalentNodeStepImpactEffects>,
  #[serde(rename = "guardianAttributes")]
  guardian_attributes: Option<::models::DestinyDefinitionsDestinyTalentNodeStepGuardianAttributes>,
  #[serde(rename = "lightAbilities")]
  light_abilities: Option<::models::DestinyDefinitionsDestinyTalentNodeStepLightAbilities>,
  #[serde(rename = "damageTypes")]
  damage_types: Option<::models::DestinyDefinitionsDestinyTalentNodeStepDamageTypes>
}

impl DestinyDefinitionsDestinyTalentNodeStepGroups {
  /// These properties are an attempt to categorize talent node steps by certain common properties. See the related enumerations for the type of properties being categorized.
  pub fn new() -> DestinyDefinitionsDestinyTalentNodeStepGroups {
    DestinyDefinitionsDestinyTalentNodeStepGroups {
      weapon_performance: None,
      impact_effects: None,
      guardian_attributes: None,
      light_abilities: None,
      damage_types: None
    }
  }

  pub fn set_weapon_performance(&mut self, weapon_performance: ::models::DestinyDefinitionsDestinyTalentNodeStepWeaponPerformances) {
    self.weapon_performance = Some(weapon_performance);
  }

  pub fn with_weapon_performance(mut self, weapon_performance: ::models::DestinyDefinitionsDestinyTalentNodeStepWeaponPerformances) -> DestinyDefinitionsDestinyTalentNodeStepGroups {
    self.weapon_performance = Some(weapon_performance);
    self
  }

  pub fn weapon_performance(&self) -> Option<&::models::DestinyDefinitionsDestinyTalentNodeStepWeaponPerformances> {
    self.weapon_performance.as_ref()
  }

  pub fn reset_weapon_performance(&mut self) {
    self.weapon_performance = None;
  }

  pub fn set_impact_effects(&mut self, impact_effects: ::models::DestinyDefinitionsDestinyTalentNodeStepImpactEffects) {
    self.impact_effects = Some(impact_effects);
  }

  pub fn with_impact_effects(mut self, impact_effects: ::models::DestinyDefinitionsDestinyTalentNodeStepImpactEffects) -> DestinyDefinitionsDestinyTalentNodeStepGroups {
    self.impact_effects = Some(impact_effects);
    self
  }

  pub fn impact_effects(&self) -> Option<&::models::DestinyDefinitionsDestinyTalentNodeStepImpactEffects> {
    self.impact_effects.as_ref()
  }

  pub fn reset_impact_effects(&mut self) {
    self.impact_effects = None;
  }

  pub fn set_guardian_attributes(&mut self, guardian_attributes: ::models::DestinyDefinitionsDestinyTalentNodeStepGuardianAttributes) {
    self.guardian_attributes = Some(guardian_attributes);
  }

  pub fn with_guardian_attributes(mut self, guardian_attributes: ::models::DestinyDefinitionsDestinyTalentNodeStepGuardianAttributes) -> DestinyDefinitionsDestinyTalentNodeStepGroups {
    self.guardian_attributes = Some(guardian_attributes);
    self
  }

  pub fn guardian_attributes(&self) -> Option<&::models::DestinyDefinitionsDestinyTalentNodeStepGuardianAttributes> {
    self.guardian_attributes.as_ref()
  }

  pub fn reset_guardian_attributes(&mut self) {
    self.guardian_attributes = None;
  }

  pub fn set_light_abilities(&mut self, light_abilities: ::models::DestinyDefinitionsDestinyTalentNodeStepLightAbilities) {
    self.light_abilities = Some(light_abilities);
  }

  pub fn with_light_abilities(mut self, light_abilities: ::models::DestinyDefinitionsDestinyTalentNodeStepLightAbilities) -> DestinyDefinitionsDestinyTalentNodeStepGroups {
    self.light_abilities = Some(light_abilities);
    self
  }

  pub fn light_abilities(&self) -> Option<&::models::DestinyDefinitionsDestinyTalentNodeStepLightAbilities> {
    self.light_abilities.as_ref()
  }

  pub fn reset_light_abilities(&mut self) {
    self.light_abilities = None;
  }

  pub fn set_damage_types(&mut self, damage_types: ::models::DestinyDefinitionsDestinyTalentNodeStepDamageTypes) {
    self.damage_types = Some(damage_types);
  }

  pub fn with_damage_types(mut self, damage_types: ::models::DestinyDefinitionsDestinyTalentNodeStepDamageTypes) -> DestinyDefinitionsDestinyTalentNodeStepGroups {
    self.damage_types = Some(damage_types);
    self
  }

  pub fn damage_types(&self) -> Option<&::models::DestinyDefinitionsDestinyTalentNodeStepDamageTypes> {
    self.damage_types.as_ref()
  }

  pub fn reset_damage_types(&mut self) {
    self.damage_types = None;
  }

}



