use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MessageType {
    #[serde(rename = "system")]
    SystemMessage,
    #[serde(rename = "ai")]
    AIMessage,
    #[serde(rename = "human")]
    HumanMessage,
}

impl Default for MessageType {
    fn default() -> Self {
        Self::SystemMessage
    }
}

/// A Message for priming AI behavior, usually passed in as the first of a sequence
/// of input messages.
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Message {
    pub content: String,
    pub message_type: MessageType,
}

impl Message {
    // Function to create a new Human message
    pub fn new_human_message(content: &str) -> Self {
        Message {
            content: String::from(content),
            message_type: MessageType::HumanMessage,
        }
    }

    // Function to create a new System message
    pub fn new_system_message(content: &str) -> Self {
        Message {
            content: String::from(content),
            message_type: MessageType::SystemMessage,
        }
    }

    // Function to create a new AI message
    pub fn new_ai_message(content: &str) -> Self {
        Message {
            content: String::from(content),
            message_type: MessageType::AIMessage,
        }
    }
}
