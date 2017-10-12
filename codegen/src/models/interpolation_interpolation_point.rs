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
pub struct InterpolationInterpolationPoint {
  #[serde(rename = "value")]
  value: Option<i32>,
  #[serde(rename = "weight")]
  weight: Option<i32>
}

impl InterpolationInterpolationPoint {
  pub fn new() -> InterpolationInterpolationPoint {
    InterpolationInterpolationPoint {
      value: None,
      weight: None
    }
  }

  pub fn set_value(&mut self, value: i32) {
    self.value = Some(value);
  }

  pub fn with_value(mut self, value: i32) -> InterpolationInterpolationPoint {
    self.value = Some(value);
    self
  }

  pub fn value(&self) -> Option<&i32> {
    self.value.as_ref()
  }

  pub fn reset_value(&mut self) {
    self.value = None;
  }

  pub fn set_weight(&mut self, weight: i32) {
    self.weight = Some(weight);
  }

  pub fn with_weight(mut self, weight: i32) -> InterpolationInterpolationPoint {
    self.weight = Some(weight);
    self
  }

  pub fn weight(&self) -> Option<&i32> {
    self.weight.as_ref()
  }

  pub fn reset_weight(&mut self) {
    self.weight = None;
  }

}



