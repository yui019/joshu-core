use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: Option<i32>,
    pub avatar_emotion: Option<String>,
    pub textbox_text: Option<String>,
    pub input: Option<MessageUserInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageUserInput {
    pub text: bool,
    pub select: Vec<String>,
}
