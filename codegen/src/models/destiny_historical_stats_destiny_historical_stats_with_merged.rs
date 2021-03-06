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
pub struct DestinyHistoricalStatsDestinyHistoricalStatsWithMerged {
  #[serde(rename = "results")]
  results: Option<::std::collections::HashMap<String, ::models::DestinyHistoricalStatsDestinyHistoricalStatsByPeriod>>,
  #[serde(rename = "merged")]
  merged: Option<::models::DestinyHistoricalStatsDestinyHistoricalStatsByPeriod>
}

impl DestinyHistoricalStatsDestinyHistoricalStatsWithMerged {
  pub fn new() -> DestinyHistoricalStatsDestinyHistoricalStatsWithMerged {
    DestinyHistoricalStatsDestinyHistoricalStatsWithMerged {
      results: None,
      merged: None
    }
  }

  pub fn set_results(&mut self, results: ::std::collections::HashMap<String, ::models::DestinyHistoricalStatsDestinyHistoricalStatsByPeriod>) {
    self.results = Some(results);
  }

  pub fn with_results(mut self, results: ::std::collections::HashMap<String, ::models::DestinyHistoricalStatsDestinyHistoricalStatsByPeriod>) -> DestinyHistoricalStatsDestinyHistoricalStatsWithMerged {
    self.results = Some(results);
    self
  }

  pub fn results(&self) -> Option<&::std::collections::HashMap<String, ::models::DestinyHistoricalStatsDestinyHistoricalStatsByPeriod>> {
    self.results.as_ref()
  }

  pub fn reset_results(&mut self) {
    self.results = None;
  }

  pub fn set_merged(&mut self, merged: ::models::DestinyHistoricalStatsDestinyHistoricalStatsByPeriod) {
    self.merged = Some(merged);
  }

  pub fn with_merged(mut self, merged: ::models::DestinyHistoricalStatsDestinyHistoricalStatsByPeriod) -> DestinyHistoricalStatsDestinyHistoricalStatsWithMerged {
    self.merged = Some(merged);
    self
  }

  pub fn merged(&self) -> Option<&::models::DestinyHistoricalStatsDestinyHistoricalStatsByPeriod> {
    self.merged.as_ref()
  }

  pub fn reset_merged(&mut self) {
    self.merged = None;
  }

}



