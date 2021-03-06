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
pub struct MessagesResponsesSaveMessageResult {
  #[serde(rename = "conversationId")]
  conversation_id: Option<i64>,
  #[serde(rename = "messageId")]
  message_id: Option<i64>
}

impl MessagesResponsesSaveMessageResult {
  pub fn new() -> MessagesResponsesSaveMessageResult {
    MessagesResponsesSaveMessageResult {
      conversation_id: None,
      message_id: None
    }
  }

  pub fn set_conversation_id(&mut self, conversation_id: i64) {
    self.conversation_id = Some(conversation_id);
  }

  pub fn with_conversation_id(mut self, conversation_id: i64) -> MessagesResponsesSaveMessageResult {
    self.conversation_id = Some(conversation_id);
    self
  }

  pub fn conversation_id(&self) -> Option<&i64> {
    self.conversation_id.as_ref()
  }

  pub fn reset_conversation_id(&mut self) {
    self.conversation_id = None;
  }

  pub fn set_message_id(&mut self, message_id: i64) {
    self.message_id = Some(message_id);
  }

  pub fn with_message_id(mut self, message_id: i64) -> MessagesResponsesSaveMessageResult {
    self.message_id = Some(message_id);
    self
  }

  pub fn message_id(&self) -> Option<&i64> {
    self.message_id.as_ref()
  }

  pub fn reset_message_id(&mut self) {
    self.message_id = None;
  }

}



