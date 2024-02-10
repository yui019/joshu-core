use std::{str::from_utf8, time};

use ggez::{
    glam::Vec2,
    graphics::{Canvas, Color, DrawParam, Image, PxScale, Text, TextFragment},
    Context,
};

use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};

pub struct Textbox {
    shown: bool,
    entire_text: String,
    displayed_text: Text,
    time_last_char_appeared: time::Duration,
    image: Image,
    bounds: Vec2,
}

impl Textbox {
    // avatar_image_width is used to compute the displayed text bounds
    pub fn new(ctx: &Context, avatar_image_width: f32) -> Self {
        let displayed_text = Text::new("");
        let image = Image::from_path(ctx, "/textbox.png").expect("Could not load textbox image!");

        let bounds = Vec2::new(
            SCREEN_WIDTH - avatar_image_width - 20.0,
            image.height() as f32 - 30.0,
        );

        Self {
            shown: false,
            displayed_text,
            entire_text: String::new(),
            time_last_char_appeared: ctx.time.time_since_start(),
            image,
            bounds,
        }
    }

    pub fn set_text(&mut self, ctx: &Context, text: &String) {
        self.shown = true;

        self.time_last_char_appeared = ctx.time.time_since_start();

        self.entire_text = text.clone();

        self.displayed_text = Text::new("");
        self.displayed_text.set_bounds(self.bounds);
    }

    pub fn hide(&mut self) {
        self.shown = false;
    }

    pub fn update(&mut self, ctx: &Context) {
        let elapsed = ctx.time.time_since_start() - self.time_last_char_appeared;

        if !self.entire_text.is_empty() && elapsed.as_millis() >= 30 {
            self.time_last_char_appeared = ctx.time.time_since_start();

            let index: usize = self.displayed_text.contents().len();

            if index < self.entire_text.len() {
                let char = self.entire_text.as_bytes()[index];
                let mut bytes = vec![char];
                // if the char is a space, add the next one straight away
                // ...pausing on spaces makes it look choppy...
                if char == b' ' {
                    if index + 1 < self.entire_text.len() {
                        let next_char = self.entire_text.as_bytes()[index + 1];
                        bytes.push(next_char);
                    }
                }

                let text = from_utf8(&bytes).unwrap();

                self.displayed_text.add(TextFragment {
                    text: text.to_string(),
                    color: Some(Color::WHITE),
                    scale: Some(PxScale::from(32.0)),
                    ..Default::default()
                });
            }
        }
    }

    pub fn draw(&self, canvas: &mut Canvas) {
        if self.shown {
            canvas.draw(
                &self.image,
                DrawParam::new().dest(Vec2::new(0.0, SCREEN_HEIGHT - self.image.height() as f32)),
            );

            canvas.draw(
                &self.displayed_text,
                DrawParam::new().dest(Vec2::new(
                    10.0,
                    SCREEN_HEIGHT - self.image.height() as f32 + 15.0,
                )),
            );
        }
    }
}
