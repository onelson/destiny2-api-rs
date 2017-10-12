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
pub struct ForumPollResult {
  #[serde(rename = "answerText")]
  answer_text: Option<String>,
  #[serde(rename = "answerSlot")]
  answer_slot: Option<i32>,
  #[serde(rename = "lastVoteDate")]
  last_vote_date: Option<String>,
  #[serde(rename = "votes")]
  votes: Option<i32>,
  #[serde(rename = "requestingUserVoted")]
  requesting_user_voted: Option<bool>
}

impl ForumPollResult {
  pub fn new() -> ForumPollResult {
    ForumPollResult {
      answer_text: None,
      answer_slot: None,
      last_vote_date: None,
      votes: None,
      requesting_user_voted: None
    }
  }

  pub fn set_answer_text(&mut self, answer_text: String) {
    self.answer_text = Some(answer_text);
  }

  pub fn with_answer_text(mut self, answer_text: String) -> ForumPollResult {
    self.answer_text = Some(answer_text);
    self
  }

  pub fn answer_text(&self) -> Option<&String> {
    self.answer_text.as_ref()
  }

  pub fn reset_answer_text(&mut self) {
    self.answer_text = None;
  }

  pub fn set_answer_slot(&mut self, answer_slot: i32) {
    self.answer_slot = Some(answer_slot);
  }

  pub fn with_answer_slot(mut self, answer_slot: i32) -> ForumPollResult {
    self.answer_slot = Some(answer_slot);
    self
  }

  pub fn answer_slot(&self) -> Option<&i32> {
    self.answer_slot.as_ref()
  }

  pub fn reset_answer_slot(&mut self) {
    self.answer_slot = None;
  }

  pub fn set_last_vote_date(&mut self, last_vote_date: String) {
    self.last_vote_date = Some(last_vote_date);
  }

  pub fn with_last_vote_date(mut self, last_vote_date: String) -> ForumPollResult {
    self.last_vote_date = Some(last_vote_date);
    self
  }

  pub fn last_vote_date(&self) -> Option<&String> {
    self.last_vote_date.as_ref()
  }

  pub fn reset_last_vote_date(&mut self) {
    self.last_vote_date = None;
  }

  pub fn set_votes(&mut self, votes: i32) {
    self.votes = Some(votes);
  }

  pub fn with_votes(mut self, votes: i32) -> ForumPollResult {
    self.votes = Some(votes);
    self
  }

  pub fn votes(&self) -> Option<&i32> {
    self.votes.as_ref()
  }

  pub fn reset_votes(&mut self) {
    self.votes = None;
  }

  pub fn set_requesting_user_voted(&mut self, requesting_user_voted: bool) {
    self.requesting_user_voted = Some(requesting_user_voted);
  }

  pub fn with_requesting_user_voted(mut self, requesting_user_voted: bool) -> ForumPollResult {
    self.requesting_user_voted = Some(requesting_user_voted);
    self
  }

  pub fn requesting_user_voted(&self) -> Option<&bool> {
    self.requesting_user_voted.as_ref()
  }

  pub fn reset_requesting_user_voted(&mut self) {
    self.requesting_user_voted = None;
  }

}



