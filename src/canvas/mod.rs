use ggez::{
    graphics::{Color, DrawMode, DrawParam, Mesh, Rect},
    Context,
};
use serde::{Deserialize, Serialize};

use self::{
    input_text::{InputTextConfig, InputTextHandler},
    select::{SelectConfig, SelectHandler},
};

mod input_text;
mod select;

trait CanvasModeHandler {
    type ConfigData;
    type SetupData;

    // called to initialize data needed for every use (like fonts, etc.)
    fn new(ctx: &mut Context, config: &Self::ConfigData) -> Self;

    // called to set the widget up (with data like the width, height, etc.)
    fn setup(&mut self, ggez_ctx: &mut Context, data: Self::SetupData);

    fn draw(
        &self,
        ggez_ctx: &mut Context,
        ggez_canvas: &mut ggez::graphics::Canvas,
        canvas_ctx: &CanvasContext,
    );

    fn handle_text_input(&mut self, ggez_ctx: &Context, inputted_char: char);

    fn handle_backspace(&mut self, ggez_ctx: &Context);
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CanvasMode {
    InputText,
    Select(Vec<String>),
}

pub struct CanvasContext {
    // this mesh is used to draw any rectangle on screen
    // at first I had a separate rect mesh like this for every rectangle, but
    // it turns out you can just reuse a single one, so that's what this is for
    pub rect_mesh: Mesh,
}

impl CanvasContext {
    pub fn draw_rect(&self, ggez_canvas: &mut ggez::graphics::Canvas, rect: &Rect, color: &Color) {
        ggez_canvas.draw(
            &self.rect_mesh,
            DrawParam::new().dest_rect(*rect).color(*color),
        );
    }
}

pub struct Canvas {
    pub ctx: CanvasContext,
    pub current_mode: Option<CanvasMode>,
    pub handler_input_text: InputTextHandler,
    pub handler_select: SelectHandler,
}

impl Canvas {
    pub fn new(ggez_ctx: &mut Context) -> Self {
        let rect_mesh = Mesh::new_rectangle(
            &ggez_ctx.gfx,
            DrawMode::fill(),
            Rect::new(0.0, 0.0, 1.0, 1.0),
            Color::WHITE,
        )
        .unwrap();

        Self {
            ctx: CanvasContext { rect_mesh },
            current_mode: None,
            handler_input_text: InputTextHandler::new(ggez_ctx, &InputTextConfig::default()),
            handler_select: SelectHandler::new(ggez_ctx, &SelectConfig::default()),
        }
    }

    pub fn set_mode(&mut self, ggez_ctx: &mut Context, mode: Option<CanvasMode>) {
        self.current_mode = mode;

        match &self.current_mode {
            Some(CanvasMode::InputText) => self.handler_input_text.setup(ggez_ctx, ()),

            Some(CanvasMode::Select(options)) => {
                self.handler_select.setup(ggez_ctx, options.clone())
            }

            None => {}
        }
    }

    pub fn draw(&self, ggez_ctx: &mut Context, ggez_canvas: &mut ggez::graphics::Canvas) {
        match self.current_mode {
            Some(CanvasMode::InputText) => {
                self.handler_input_text
                    .draw(ggez_ctx, ggez_canvas, &self.ctx)
            }

            Some(CanvasMode::Select(_)) => {
                self.handler_select.draw(ggez_ctx, ggez_canvas, &self.ctx)
            }

            None => {}
        }
    }

    pub fn handle_text_input(&mut self, ggez_ctx: &Context, inputted_char: char) {
        match self.current_mode {
            Some(CanvasMode::InputText) => self
                .handler_input_text
                .handle_text_input(ggez_ctx, inputted_char),

            Some(CanvasMode::Select(_)) => self
                .handler_select
                .handle_text_input(ggez_ctx, inputted_char),

            None => {}
        }
    }

    pub fn handle_backspace(&mut self, ggez_ctx: &Context) {
        match self.current_mode {
            Some(CanvasMode::InputText) => self.handler_input_text.handle_backspace(ggez_ctx),

            Some(CanvasMode::Select(_)) => self.handler_select.handle_backspace(ggez_ctx),

            None => {}
        }
    }
}
