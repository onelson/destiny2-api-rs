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
pub struct DestinyItemComponentSetOfint32 {
  #[serde(rename = "instances")]
  instances: Option<::models::DictionaryComponentResponseOfint32AndDestinyItemInstanceComponent>,
  #[serde(rename = "objectives")]
  objectives: Option<::models::DictionaryComponentResponseOfint32AndDestinyItemObjectivesComponent>,
  #[serde(rename = "perks")]
  perks: Option<::models::DictionaryComponentResponseOfint32AndDestinyItemPerksComponent>,
  #[serde(rename = "renderData")]
  render_data: Option<::models::DictionaryComponentResponseOfint32AndDestinyItemRenderComponent>,
  #[serde(rename = "stats")]
  stats: Option<::models::DictionaryComponentResponseOfint32AndDestinyItemStatsComponent>,
  #[serde(rename = "sockets")]
  sockets: Option<::models::DictionaryComponentResponseOfint32AndDestinyItemSocketsComponent>,
  #[serde(rename = "talentGrids")]
  talent_grids: Option<::models::DictionaryComponentResponseOfint32AndDestinyItemTalentGridComponent>,
  #[serde(rename = "plugStates")]
  plug_states: Option<::models::DictionaryComponentResponseOfuint32AndDestinyItemPlugComponent>
}

impl DestinyItemComponentSetOfint32 {
  pub fn new() -> DestinyItemComponentSetOfint32 {
    DestinyItemComponentSetOfint32 {
      instances: None,
      objectives: None,
      perks: None,
      render_data: None,
      stats: None,
      sockets: None,
      talent_grids: None,
      plug_states: None
    }
  }

  pub fn set_instances(&mut self, instances: ::models::DictionaryComponentResponseOfint32AndDestinyItemInstanceComponent) {
    self.instances = Some(instances);
  }

  pub fn with_instances(mut self, instances: ::models::DictionaryComponentResponseOfint32AndDestinyItemInstanceComponent) -> DestinyItemComponentSetOfint32 {
    self.instances = Some(instances);
    self
  }

  pub fn instances(&self) -> Option<&::models::DictionaryComponentResponseOfint32AndDestinyItemInstanceComponent> {
    self.instances.as_ref()
  }

  pub fn reset_instances(&mut self) {
    self.instances = None;
  }

  pub fn set_objectives(&mut self, objectives: ::models::DictionaryComponentResponseOfint32AndDestinyItemObjectivesComponent) {
    self.objectives = Some(objectives);
  }

  pub fn with_objectives(mut self, objectives: ::models::DictionaryComponentResponseOfint32AndDestinyItemObjectivesComponent) -> DestinyItemComponentSetOfint32 {
    self.objectives = Some(objectives);
    self
  }

  pub fn objectives(&self) -> Option<&::models::DictionaryComponentResponseOfint32AndDestinyItemObjectivesComponent> {
    self.objectives.as_ref()
  }

  pub fn reset_objectives(&mut self) {
    self.objectives = None;
  }

  pub fn set_perks(&mut self, perks: ::models::DictionaryComponentResponseOfint32AndDestinyItemPerksComponent) {
    self.perks = Some(perks);
  }

  pub fn with_perks(mut self, perks: ::models::DictionaryComponentResponseOfint32AndDestinyItemPerksComponent) -> DestinyItemComponentSetOfint32 {
    self.perks = Some(perks);
    self
  }

  pub fn perks(&self) -> Option<&::models::DictionaryComponentResponseOfint32AndDestinyItemPerksComponent> {
    self.perks.as_ref()
  }

  pub fn reset_perks(&mut self) {
    self.perks = None;
  }

  pub fn set_render_data(&mut self, render_data: ::models::DictionaryComponentResponseOfint32AndDestinyItemRenderComponent) {
    self.render_data = Some(render_data);
  }

  pub fn with_render_data(mut self, render_data: ::models::DictionaryComponentResponseOfint32AndDestinyItemRenderComponent) -> DestinyItemComponentSetOfint32 {
    self.render_data = Some(render_data);
    self
  }

  pub fn render_data(&self) -> Option<&::models::DictionaryComponentResponseOfint32AndDestinyItemRenderComponent> {
    self.render_data.as_ref()
  }

  pub fn reset_render_data(&mut self) {
    self.render_data = None;
  }

  pub fn set_stats(&mut self, stats: ::models::DictionaryComponentResponseOfint32AndDestinyItemStatsComponent) {
    self.stats = Some(stats);
  }

  pub fn with_stats(mut self, stats: ::models::DictionaryComponentResponseOfint32AndDestinyItemStatsComponent) -> DestinyItemComponentSetOfint32 {
    self.stats = Some(stats);
    self
  }

  pub fn stats(&self) -> Option<&::models::DictionaryComponentResponseOfint32AndDestinyItemStatsComponent> {
    self.stats.as_ref()
  }

  pub fn reset_stats(&mut self) {
    self.stats = None;
  }

  pub fn set_sockets(&mut self, sockets: ::models::DictionaryComponentResponseOfint32AndDestinyItemSocketsComponent) {
    self.sockets = Some(sockets);
  }

  pub fn with_sockets(mut self, sockets: ::models::DictionaryComponentResponseOfint32AndDestinyItemSocketsComponent) -> DestinyItemComponentSetOfint32 {
    self.sockets = Some(sockets);
    self
  }

  pub fn sockets(&self) -> Option<&::models::DictionaryComponentResponseOfint32AndDestinyItemSocketsComponent> {
    self.sockets.as_ref()
  }

  pub fn reset_sockets(&mut self) {
    self.sockets = None;
  }

  pub fn set_talent_grids(&mut self, talent_grids: ::models::DictionaryComponentResponseOfint32AndDestinyItemTalentGridComponent) {
    self.talent_grids = Some(talent_grids);
  }

  pub fn with_talent_grids(mut self, talent_grids: ::models::DictionaryComponentResponseOfint32AndDestinyItemTalentGridComponent) -> DestinyItemComponentSetOfint32 {
    self.talent_grids = Some(talent_grids);
    self
  }

  pub fn talent_grids(&self) -> Option<&::models::DictionaryComponentResponseOfint32AndDestinyItemTalentGridComponent> {
    self.talent_grids.as_ref()
  }

  pub fn reset_talent_grids(&mut self) {
    self.talent_grids = None;
  }

  pub fn set_plug_states(&mut self, plug_states: ::models::DictionaryComponentResponseOfuint32AndDestinyItemPlugComponent) {
    self.plug_states = Some(plug_states);
  }

  pub fn with_plug_states(mut self, plug_states: ::models::DictionaryComponentResponseOfuint32AndDestinyItemPlugComponent) -> DestinyItemComponentSetOfint32 {
    self.plug_states = Some(plug_states);
    self
  }

  pub fn plug_states(&self) -> Option<&::models::DictionaryComponentResponseOfuint32AndDestinyItemPlugComponent> {
    self.plug_states.as_ref()
  }

  pub fn reset_plug_states(&mut self) {
    self.plug_states = None;
  }

}


