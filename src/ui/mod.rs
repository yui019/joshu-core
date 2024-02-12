use self::input_text::InputText;
use ggez::{graphics::Canvas, Context};
use serde::{Deserialize, Serialize};

mod input_text;
mod util;

trait UiWidget {
    type SetupData;

    // called to initialize data needed for every use (like fonts, etc.)
    fn new(ctx: &mut Context) -> Self;

    // called to initialize the widget (with data like the width, height, etc.)
    fn setup(&mut self, ctx: &mut Context, data: Self::SetupData);

    fn draw(&mut self, ctx: &mut Context, canvas: &mut Canvas);

    fn handle_text_input(&mut self, ctx: &Context, inputted_char: char);

    fn handle_backspace(&mut self, ctx: &Context);
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UiWidgetSetupData {
    InputText,
    Select(Vec<String>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UiWidgetType {
    InputText,
    Select,
}

pub struct Ui {
    current_widget: Option<UiWidgetType>,
    input_text: InputText,
}

impl Ui {
    pub fn new(ctx: &mut Context) -> Self {
        Self {
            current_widget: None,
            input_text: InputText::new(ctx),
        }
    }

    pub fn set_type(&mut self, ctx: &mut Context, data: Option<UiWidgetSetupData>) {
        match data {
            Some(UiWidgetSetupData::InputText) => {
                self.current_widget = Some(UiWidgetType::InputText);
                self.input_text.setup(ctx, ());
            }

            Some(UiWidgetSetupData::Select(_)) => {}

            None => self.current_widget = None,
        }
    }

    pub fn draw(&mut self, ctx: &mut Context, canvas: &mut Canvas) {
        match &self.current_widget {
            Some(UiWidgetType::InputText) => {
                self.input_text.draw(ctx, canvas);
            }

            Some(UiWidgetType::Select) => {}

            None => {}
        }
    }

    pub fn handle_text_input(&mut self, ctx: &Context, inputted_char: char) {
        match &mut self.current_widget {
            Some(UiWidgetType::InputText) => self.input_text.handle_text_input(ctx, inputted_char),

            Some(UiWidgetType::Select) => {}

            _ => {}
        }
    }

    pub fn handle_backspace(&mut self, ctx: &Context) {
        match &mut self.current_widget {
            Some(UiWidgetType::InputText) => self.input_text.handle_backspace(ctx),

            Some(UiWidgetType::Select) => {}

            _ => {}
        }
    }
}
