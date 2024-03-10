use std::str::from_utf8;

use ggez::{
    glam::Vec2,
    graphics::{Color, DrawParam, Drawable, PxScale, Rect, Text, TextFragment},
};

use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};

use super::CanvasModeHandler;

#[derive(Clone)]
pub struct InputTextConfig {
    pub text_min_width: f32,
    pub text_max_width: f32,
    pub text_font_size: f32,
    pub text_horizontal_padding: f32,
    pub text_vertical_padding: f32,
    pub text_default_placeholder: &'static str,
    pub text_placeholder_alpha: f32, // from 0 to 1
    pub text_color: Color,
    pub background_outline_width: f32,
    pub background_outline_color: Color,
    pub background_color: Color,
    pub y_position: f32,
    pub x_position: f32,
}

impl Default for InputTextConfig {
    fn default() -> Self {
        Self {
            text_min_width: 500.0,
            text_max_width: 1800.0,
            text_font_size: 32.0,
            text_horizontal_padding: 10.0,
            text_vertical_padding: 15.0,
            text_default_placeholder: "Enter text...",
            text_placeholder_alpha: 0.7,
            text_color: Color::BLACK,
            background_outline_width: 2.0,
            background_outline_color: Color::BLACK,
            background_color: Color::WHITE,
            y_position: SCREEN_HEIGHT / 2.0,
            x_position: SCREEN_WIDTH / 2.0,
        }
    }
}

pub struct InputTextHandler {
    pub config: InputTextConfig,
    pub background_rect: Rect,
    pub placeholder_text: Text,
    pub displayed_text: Text,
    pub entire_text: String,
}

impl InputTextHandler {
    pub fn get_outline_rect(&self) -> Rect {
        let mut outline_rect = self.background_rect;
        outline_rect.x -= self.config.background_outline_width;
        outline_rect.y -= self.config.background_outline_width;
        outline_rect.w += 2.0 * self.config.background_outline_width;
        outline_rect.h += 2.0 * self.config.background_outline_width;

        outline_rect
    }

    // Get the initial background rect that would be created with the given config when you call self.new
    pub fn get_initial_background_rect(config: &InputTextConfig) -> Rect {
        let width = config.text_min_width;
        let height = config.text_font_size + (2.0 * config.text_vertical_padding);

        Rect::new(
            config.x_position - width / 2.0,
            config.y_position - height / 2.0,
            width,
            height,
        )
    }
}

impl CanvasModeHandler for InputTextHandler {
    type ConfigData = InputTextConfig;
    type SetupData = ();

    fn new(_ggez_ctx: &mut ggez::Context, config: &Self::ConfigData) -> Self {
        let background_rect = {
            let width = config.text_min_width;
            let height = config.text_font_size + (2.0 * config.text_vertical_padding);

            Rect::new(
                config.x_position - width / 2.0,
                config.y_position - height / 2.0,
                width,
                height,
            )
        };

        let mut placeholder_color = config.text_color;
        placeholder_color.a = config.text_placeholder_alpha;

        let placeholder_text = Text::new(TextFragment {
            text: config.text_default_placeholder.to_string(),
            color: Some(placeholder_color),
            scale: Some(PxScale::from(config.text_font_size)),
            ..Default::default()
        });

        Self {
            config: config.clone(),
            background_rect,
            placeholder_text,
            displayed_text: Text::new(""),
            entire_text: String::new(),
        }
    }

    fn setup(&mut self, _ggez_ctx: &mut ggez::Context, _data: Self::SetupData) {
        // reset background rect
        self.background_rect.w = self.config.text_min_width;
        self.background_rect.x = self.config.x_position - self.config.text_min_width / 2.0;

        // reset text
        self.displayed_text = Text::new("");
        self.entire_text = String::new();
    }

    fn draw(
        &self,
        _ggez_ctx: &mut ggez::Context,
        ggez_canvas: &mut ggez::graphics::Canvas,
        canvas_ctx: &super::CanvasContext,
    ) {
        // draw background outline
        canvas_ctx.draw_rect(
            ggez_canvas,
            &self.get_outline_rect(),
            &self.config.background_outline_color,
        );

        // draw background
        canvas_ctx.draw_rect(
            ggez_canvas,
            &self.background_rect,
            &self.config.background_color,
        );

        let text_x = self.background_rect.x + self.config.text_horizontal_padding;
        let text_y = self.background_rect.y + self.background_rect.h / 2.0
            - (self.config.text_font_size / 2.0);

        if self.entire_text.len() == 0 {
            // display placeholder if there's no inputted text
            ggez_canvas.draw(
                &self.placeholder_text,
                DrawParam::new().dest(Vec2::new(text_x, text_y)),
            );
        } else {
            // display inputted text
            ggez_canvas.draw(
                &self.displayed_text,
                DrawParam::new().dest(Vec2::new(text_x, text_y)),
            );
        }
    }

    fn handle_text_input(&mut self, ggez_ctx: &ggez::Context, inputted_char: char) {
        if !(inputted_char.is_alphanumeric()
            || inputted_char.is_ascii_punctuation()
            || inputted_char == ' ')
        {
            return;
        }

        // add inputted char to text
        self.entire_text = format!("{}{}", self.entire_text, inputted_char);
        self.displayed_text.add(TextFragment {
            text: inputted_char.to_string(),
            color: Some(Color::BLACK),
            scale: Some(PxScale::from(self.config.text_font_size)),
            ..Default::default()
        });

        // handle text overflowing the input field
        let text_rect = self.displayed_text.dimensions(&ggez_ctx.gfx).unwrap();

        if text_rect.w + (2.0 * self.config.text_horizontal_padding) >= self.background_rect.w {
            let new_width = text_rect.w + (2.0 * self.config.text_horizontal_padding);

            if new_width <= self.config.text_max_width {
                // just expand the input field if there's still space left
                self.background_rect.w = new_width;
                self.background_rect.x = self.config.x_position - new_width / 2.0;
            } else {
                // if the input field is already maximally expanded, then scroll the text so its end is visible
                self.displayed_text = {
                    let mut new_text = Text::new("");

                    let start = self.entire_text.len() - self.displayed_text.contents().len() + 1;
                    for i in start..self.entire_text.len() {
                        let text = from_utf8(&[self.entire_text.as_bytes()[i]])
                            .unwrap()
                            .to_string();

                        new_text.add(TextFragment {
                            text,
                            color: Some(Color::BLACK),
                            scale: Some(PxScale::from(self.config.text_font_size)),
                            ..Default::default()
                        });
                    }

                    new_text
                };
            }
        }
    }

    fn handle_backspace(&mut self, ggez_ctx: &ggez::Context) {
        // remove last character
        if self.entire_text.len() == 0 {
            return;
        }

        self.entire_text.remove(self.entire_text.len() - 1);
        self.displayed_text = {
            let old_displayed_len = self.displayed_text.contents().len();

            if old_displayed_len - 1 < self.entire_text.len() {
                // if there's more text than what's being displayed, handle that properly
                let mut new_text = Text::new("");

                let start = self.entire_text.len() - old_displayed_len;
                for i in start..self.entire_text.len() {
                    let text = from_utf8(&[self.entire_text.as_bytes()[i]])
                        .unwrap()
                        .to_string();

                    new_text.add(TextFragment {
                        text,
                        color: Some(Color::BLACK),
                        scale: Some(PxScale::from(self.config.text_font_size)),
                        ..Default::default()
                    });
                }

                new_text
            } else {
                let remaining_fragments = self.displayed_text.fragments()
                    [..(self.displayed_text.fragments().len() - 1)]
                    .to_vec();

                let mut new_text = Text::new("");
                for fr in remaining_fragments {
                    new_text.add(fr);
                }

                new_text
            }
        };

        // reduce input width (up to the minimum)
        let text_rect = self.displayed_text.dimensions(&ggez_ctx.gfx).unwrap();
        let new_width = text_rect.w + (2.0 * self.config.text_horizontal_padding);
        if new_width >= self.config.text_min_width {
            self.background_rect.w = new_width;
            self.background_rect.x = self.config.x_position - new_width / 2.0;
        }
    }

    fn handle_arrow_key(
        &mut self,
        _ggez_ctx: &ggez::Context,
        _keycode: ggez::input::keyboard::KeyCode,
    ) {
    }
}
