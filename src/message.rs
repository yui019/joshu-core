use crate::ui::UiType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: Option<i32>,
    pub avatar_emotion: Option<String>,
    pub textbox_text: Option<String>,
    pub ui: Option<UiType>,
}
