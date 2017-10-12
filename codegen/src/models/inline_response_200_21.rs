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
pub struct InlineResponse20021 {
  #[serde(rename = "Response")]
  response: Option<::models::GroupsV2GroupMemberLeaveResult>,
  #[serde(rename = "ErrorCode")]
  error_code: Option<::models::ExceptionsPlatformErrorCodes>,
  #[serde(rename = "ThrottleSeconds")]
  throttle_seconds: Option<i32>,
  #[serde(rename = "ErrorStatus")]
  error_status: Option<String>,
  #[serde(rename = "Message")]
  message: Option<String>,
  #[serde(rename = "MessageData")]
  message_data: Option<::std::collections::HashMap<String, String>>
}

impl InlineResponse20021 {
  pub fn new() -> InlineResponse20021 {
    InlineResponse20021 {
      response: None,
      error_code: None,
      throttle_seconds: None,
      error_status: None,
      message: None,
      message_data: None
    }
  }

  pub fn set_response(&mut self, response: ::models::GroupsV2GroupMemberLeaveResult) {
    self.response = Some(response);
  }

  pub fn with_response(mut self, response: ::models::GroupsV2GroupMemberLeaveResult) -> InlineResponse20021 {
    self.response = Some(response);
    self
  }

  pub fn response(&self) -> Option<&::models::GroupsV2GroupMemberLeaveResult> {
    self.response.as_ref()
  }

  pub fn reset_response(&mut self) {
    self.response = None;
  }

  pub fn set_error_code(&mut self, error_code: ::models::ExceptionsPlatformErrorCodes) {
    self.error_code = Some(error_code);
  }

  pub fn with_error_code(mut self, error_code: ::models::ExceptionsPlatformErrorCodes) -> InlineResponse20021 {
    self.error_code = Some(error_code);
    self
  }

  pub fn error_code(&self) -> Option<&::models::ExceptionsPlatformErrorCodes> {
    self.error_code.as_ref()
  }

  pub fn reset_error_code(&mut self) {
    self.error_code = None;
  }

  pub fn set_throttle_seconds(&mut self, throttle_seconds: i32) {
    self.throttle_seconds = Some(throttle_seconds);
  }

  pub fn with_throttle_seconds(mut self, throttle_seconds: i32) -> InlineResponse20021 {
    self.throttle_seconds = Some(throttle_seconds);
    self
  }

  pub fn throttle_seconds(&self) -> Option<&i32> {
    self.throttle_seconds.as_ref()
  }

  pub fn reset_throttle_seconds(&mut self) {
    self.throttle_seconds = None;
  }

  pub fn set_error_status(&mut self, error_status: String) {
    self.error_status = Some(error_status);
  }

  pub fn with_error_status(mut self, error_status: String) -> InlineResponse20021 {
    self.error_status = Some(error_status);
    self
  }

  pub fn error_status(&self) -> Option<&String> {
    self.error_status.as_ref()
  }

  pub fn reset_error_status(&mut self) {
    self.error_status = None;
  }

  pub fn set_message(&mut self, message: String) {
    self.message = Some(message);
  }

  pub fn with_message(mut self, message: String) -> InlineResponse20021 {
    self.message = Some(message);
    self
  }

  pub fn message(&self) -> Option<&String> {
    self.message.as_ref()
  }

  pub fn reset_message(&mut self) {
    self.message = None;
  }

  pub fn set_message_data(&mut self, message_data: ::std::collections::HashMap<String, String>) {
    self.message_data = Some(message_data);
  }

  pub fn with_message_data(mut self, message_data: ::std::collections::HashMap<String, String>) -> InlineResponse20021 {
    self.message_data = Some(message_data);
    self
  }

  pub fn message_data(&self) -> Option<&::std::collections::HashMap<String, String>> {
    self.message_data.as_ref()
  }

  pub fn reset_message_data(&mut self) {
    self.message_data = None;
  }

}



