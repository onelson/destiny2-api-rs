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
pub struct DestinyHistoricalStatsDestinyHistoricalStatsByPeriod {
  #[serde(rename = "allTime")]
  all_time: Option<::std::collections::HashMap<String, ::models::DestinyHistoricalStatsDestinyHistoricalStatsValue>>,
  #[serde(rename = "allTimeTier1")]
  all_time_tier1: Option<::std::collections::HashMap<String, ::models::DestinyHistoricalStatsDestinyHistoricalStatsValue>>,
  #[serde(rename = "allTimeTier2")]
  all_time_tier2: Option<::std::collections::HashMap<String, ::models::DestinyHistoricalStatsDestinyHistoricalStatsValue>>,
  #[serde(rename = "allTimeTier3")]
  all_time_tier3: Option<::std::collections::HashMap<String, ::models::DestinyHistoricalStatsDestinyHistoricalStatsValue>>,
  #[serde(rename = "daily")]
  daily: Option<Vec<::models::DestinyHistoricalStatsDestinyHistoricalStatsPeriodGroup>>,
  #[serde(rename = "monthly")]
  monthly: Option<Vec<::models::DestinyHistoricalStatsDestinyHistoricalStatsPeriodGroup>>
}

impl DestinyHistoricalStatsDestinyHistoricalStatsByPeriod {
  pub fn new() -> DestinyHistoricalStatsDestinyHistoricalStatsByPeriod {
    DestinyHistoricalStatsDestinyHistoricalStatsByPeriod {
      all_time: None,
      all_time_tier1: None,
      all_time_tier2: None,
      all_time_tier3: None,
      daily: None,
      monthly: None
    }
  }

  pub fn set_all_time(&mut self, all_time: ::std::collections::HashMap<String, ::models::DestinyHistoricalStatsDestinyHistoricalStatsValue>) {
    self.all_time = Some(all_time);
  }

  pub fn with_all_time(mut self, all_time: ::std::collections::HashMap<String, ::models::DestinyHistoricalStatsDestinyHistoricalStatsValue>) -> DestinyHistoricalStatsDestinyHistoricalStatsByPeriod {
    self.all_time = Some(all_time);
    self
  }

  pub fn all_time(&self) -> Option<&::std::collections::HashMap<String, ::models::DestinyHistoricalStatsDestinyHistoricalStatsValue>> {
    self.all_time.as_ref()
  }

  pub fn reset_all_time(&mut self) {
    self.all_time = None;
  }

  pub fn set_all_time_tier1(&mut self, all_time_tier1: ::std::collections::HashMap<String, ::models::DestinyHistoricalStatsDestinyHistoricalStatsValue>) {
    self.all_time_tier1 = Some(all_time_tier1);
  }

  pub fn with_all_time_tier1(mut self, all_time_tier1: ::std::collections::HashMap<String, ::models::DestinyHistoricalStatsDestinyHistoricalStatsValue>) -> DestinyHistoricalStatsDestinyHistoricalStatsByPeriod {
    self.all_time_tier1 = Some(all_time_tier1);
    self
  }

  pub fn all_time_tier1(&self) -> Option<&::std::collections::HashMap<String, ::models::DestinyHistoricalStatsDestinyHistoricalStatsValue>> {
    self.all_time_tier1.as_ref()
  }

  pub fn reset_all_time_tier1(&mut self) {
    self.all_time_tier1 = None;
  }

  pub fn set_all_time_tier2(&mut self, all_time_tier2: ::std::collections::HashMap<String, ::models::DestinyHistoricalStatsDestinyHistoricalStatsValue>) {
    self.all_time_tier2 = Some(all_time_tier2);
  }

  pub fn with_all_time_tier2(mut self, all_time_tier2: ::std::collections::HashMap<String, ::models::DestinyHistoricalStatsDestinyHistoricalStatsValue>) -> DestinyHistoricalStatsDestinyHistoricalStatsByPeriod {
    self.all_time_tier2 = Some(all_time_tier2);
    self
  }

  pub fn all_time_tier2(&self) -> Option<&::std::collections::HashMap<String, ::models::DestinyHistoricalStatsDestinyHistoricalStatsValue>> {
    self.all_time_tier2.as_ref()
  }

  pub fn reset_all_time_tier2(&mut self) {
    self.all_time_tier2 = None;
  }

  pub fn set_all_time_tier3(&mut self, all_time_tier3: ::std::collections::HashMap<String, ::models::DestinyHistoricalStatsDestinyHistoricalStatsValue>) {
    self.all_time_tier3 = Some(all_time_tier3);
  }

  pub fn with_all_time_tier3(mut self, all_time_tier3: ::std::collections::HashMap<String, ::models::DestinyHistoricalStatsDestinyHistoricalStatsValue>) -> DestinyHistoricalStatsDestinyHistoricalStatsByPeriod {
    self.all_time_tier3 = Some(all_time_tier3);
    self
  }

  pub fn all_time_tier3(&self) -> Option<&::std::collections::HashMap<String, ::models::DestinyHistoricalStatsDestinyHistoricalStatsValue>> {
    self.all_time_tier3.as_ref()
  }

  pub fn reset_all_time_tier3(&mut self) {
    self.all_time_tier3 = None;
  }

  pub fn set_daily(&mut self, daily: Vec<::models::DestinyHistoricalStatsDestinyHistoricalStatsPeriodGroup>) {
    self.daily = Some(daily);
  }

  pub fn with_daily(mut self, daily: Vec<::models::DestinyHistoricalStatsDestinyHistoricalStatsPeriodGroup>) -> DestinyHistoricalStatsDestinyHistoricalStatsByPeriod {
    self.daily = Some(daily);
    self
  }

  pub fn daily(&self) -> Option<&Vec<::models::DestinyHistoricalStatsDestinyHistoricalStatsPeriodGroup>> {
    self.daily.as_ref()
  }

  pub fn reset_daily(&mut self) {
    self.daily = None;
  }

  pub fn set_monthly(&mut self, monthly: Vec<::models::DestinyHistoricalStatsDestinyHistoricalStatsPeriodGroup>) {
    self.monthly = Some(monthly);
  }

  pub fn with_monthly(mut self, monthly: Vec<::models::DestinyHistoricalStatsDestinyHistoricalStatsPeriodGroup>) -> DestinyHistoricalStatsDestinyHistoricalStatsByPeriod {
    self.monthly = Some(monthly);
    self
  }

  pub fn monthly(&self) -> Option<&Vec<::models::DestinyHistoricalStatsDestinyHistoricalStatsPeriodGroup>> {
    self.monthly.as_ref()
  }

  pub fn reset_monthly(&mut self) {
    self.monthly = None;
  }

}


