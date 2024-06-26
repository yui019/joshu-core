use crate::canvas::CanvasMode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Message {
    pub id: Option<String>,
    pub avatar_emotion: Option<String>,
    pub textbox_text: Option<String>,
    pub canvas_mode: Option<CanvasMode>,
}
