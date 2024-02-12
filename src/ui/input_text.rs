use std::str::from_utf8;

use ggez::{
    glam::Vec2,
    graphics::{Color, DrawMode, DrawParam, Drawable, Mesh, PxScale, Rect, Text, TextFragment},
};

use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};

use super::{
    util::{rect_mesh_set_w, rect_mesh_set_x},
    UiWidget,
};

const TEXT_MIN_WIDTH: f32 = 500.0;
const TEXT_MAX_WIDTH: f32 = 1800.0;
const TEXT_FONT_SIZE: f32 = 32.0;
const TEXT_HORIZONTAL_PADDING: f32 = 10.0;
const TEXT_VERTICAL_PADDING: f32 = 15.0;
const TEXT_DEFAULT_PLACEHOLDER: &str = "Enter text...";

pub struct InputText {
    background_rect: Mesh,
    placeholder_text: Text,

    displayed_text: Text,
    entire_text: String,
}

impl UiWidget for InputText {
    type SetupData = ();

    fn new(ctx: &mut ggez::Context) -> Self {
        let background_rect = {
            let width = TEXT_MIN_WIDTH;
            let height = TEXT_FONT_SIZE + (2.0 * TEXT_VERTICAL_PADDING);

            Mesh::new_rectangle(
                &ctx.gfx,
                DrawMode::fill(),
                Rect::new(
                    SCREEN_WIDTH / 2.0 - width / 2.0,
                    SCREEN_HEIGHT / 2.0 - height / 2.0,
                    width,
                    height,
                ),
                Color::WHITE,
            )
            .unwrap()
        };

        let placeholder_text = Text::new(TextFragment {
            text: TEXT_DEFAULT_PLACEHOLDER.to_string(),
            color: Some(Color::from_rgba(0, 0, 0, 180)), // 180 alpha (from 0 to 255)
            scale: Some(PxScale::from(TEXT_FONT_SIZE)),
            ..Default::default()
        });

        Self {
            background_rect,
            placeholder_text,
            displayed_text: Text::new(""),
            entire_text: String::new(),
        }
    }

    fn setup(&mut self, ctx: &mut ggez::Context, data: Self::SetupData) {
        // reset input width
        rect_mesh_set_w(ctx, &mut self.background_rect, Color::WHITE, TEXT_MIN_WIDTH);
        rect_mesh_set_x(
            ctx,
            &mut self.background_rect,
            Color::WHITE,
            SCREEN_WIDTH / 2.0 - TEXT_MIN_WIDTH / 2.0,
        );

        self.displayed_text = Text::new("");
        self.entire_text = String::new();
    }

    fn draw(&mut self, ctx: &mut ggez::Context, canvas: &mut ggez::graphics::Canvas) {
        canvas.draw(&self.background_rect, DrawParam::new());

        let background_rect = self.background_rect.dimensions(&ctx.gfx).unwrap();
        let text_x = background_rect.x + TEXT_HORIZONTAL_PADDING;
        let text_y = background_rect.y + background_rect.h / 2.0 - (TEXT_FONT_SIZE / 2.0);

        if self.entire_text.len() == 0 {
            // display placeholder if there's no inputted text
            canvas.draw(
                &self.placeholder_text,
                DrawParam::new().dest(Vec2::new(text_x, text_y)),
            );
        } else {
            // display inputted text
            canvas.draw(
                &self.displayed_text,
                DrawParam::new().dest(Vec2::new(text_x, text_y)),
            );
        }
    }

    fn handle_text_input(&mut self, ctx: &ggez::Context, inputted_char: char) {
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
            scale: Some(PxScale::from(TEXT_FONT_SIZE)),
            ..Default::default()
        });

        // handle text overflowing the input field
        let text_rect = self.displayed_text.dimensions(&ctx.gfx).unwrap();
        let background_rect = self.background_rect.dimensions(&ctx.gfx).unwrap();

        if text_rect.w + (2.0 * TEXT_HORIZONTAL_PADDING) >= background_rect.w {
            let new_width = text_rect.w + (2.0 * TEXT_HORIZONTAL_PADDING);

            if new_width <= TEXT_MAX_WIDTH {
                // just expand the input field if there's still space left
                rect_mesh_set_w(ctx, &mut self.background_rect, Color::WHITE, new_width);
                rect_mesh_set_x(
                    ctx,
                    &mut self.background_rect,
                    Color::WHITE,
                    SCREEN_WIDTH / 2.0 - new_width / 2.0,
                );
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
                            scale: Some(PxScale::from(TEXT_FONT_SIZE)),
                            ..Default::default()
                        });
                    }

                    new_text
                };
            }
        }
    }

    fn handle_backspace(&mut self, ctx: &ggez::Context) {
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
                        scale: Some(PxScale::from(TEXT_FONT_SIZE)),
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
        let text_rect = self.displayed_text.dimensions(&ctx.gfx).unwrap();
        let new_width = text_rect.w + (2.0 * TEXT_HORIZONTAL_PADDING);
        if new_width >= TEXT_MIN_WIDTH {
            let background_rect = self.background_rect.dimensions(&ctx.gfx).unwrap();

            rect_mesh_set_w(ctx, &mut self.background_rect, Color::WHITE, new_width);
            rect_mesh_set_x(
                ctx,
                &mut self.background_rect,
                Color::WHITE,
                SCREEN_WIDTH / 2.0 - new_width / 2.0,
            );
        }
    }
}
