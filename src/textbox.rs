use std::{str::from_utf8, sync::mpsc::Sender, time};

use ggez::{
    glam::Vec2,
    graphics::{Canvas, Color, DrawParam, Drawable, Image, PxScale, Text, TextFragment},
    Context,
};

use crate::{app::FinishedMessage, SCREEN_HEIGHT, SCREEN_WIDTH};

const FONT_SIZE: f32 = 32.0;
const HORIZONTAL_PADDING: f32 = 10.0;
const VERTICAL_PADDING: f32 = 15.0;
const TEXT_ANIMATION_SPEED: u128 = 30; // amount of time in milliseconds in between characters appearing
const DISAPPEAR_SPEED: u128 = 2000; // amount of time in milliseconds before the textbox disappears

pub struct Textbox {
    shown: bool,
    entire_text: String,
    displayed_text: Text,
    time_last_char_appeared: time::Duration,
    time_finished: Option<time::Duration>,
    image: Image,
    bounds: Vec2,
    finished_sender: Sender<FinishedMessage>,
    finished_sender_enabled: bool,
}

impl Textbox {
    // avatar_image_width is used to compute the displayed text bounds
    pub fn new(
        ctx: &Context,
        avatar_image_width: f32,
        finished_sender: Sender<FinishedMessage>,
    ) -> Self {
        let displayed_text = Text::new("");
        let image = Image::from_path(ctx, "/textbox.png").expect("Could not load textbox image!");

        let bounds = Vec2::new(
            SCREEN_WIDTH - avatar_image_width - (2.0 * HORIZONTAL_PADDING),
            f32::MAX,
        );

        Self {
            shown: false,
            displayed_text,
            entire_text: String::new(),
            time_last_char_appeared: ctx.time.time_since_start(),
            time_finished: None,
            image,
            bounds,
            finished_sender,
            finished_sender_enabled: false,
        }
    }

    pub fn set_text(&mut self, ctx: &Context, text: &String, finished_listener_enabled: bool) {
        self.shown = true;

        self.time_finished = None;
        self.time_last_char_appeared = ctx.time.time_since_start();

        self.entire_text = text.clone();

        self.displayed_text = Text::new("");
        self.displayed_text.set_bounds(self.bounds);

        self.finished_sender_enabled = finished_listener_enabled;
    }

    pub fn hide(&mut self) {
        self.shown = false;
    }

    pub fn update(&mut self, ctx: &Context) {
        if self.shown {
            match self.time_finished {
                // if text has finished displaying
                Some(time_finished) => {
                    if self.finished_sender_enabled {
                        let elapsed = ctx.time.time_since_start() - time_finished;

                        if elapsed.as_millis() >= DISAPPEAR_SPEED {
                            self.hide();

                            self.finished_sender.send(FinishedMessage::Textbox).unwrap();
                        }
                    }
                }

                // if text hasn't finished displaying
                None => {
                    let elapsed = ctx.time.time_since_start() - self.time_last_char_appeared;

                    if !self.entire_text.is_empty() && elapsed.as_millis() >= TEXT_ANIMATION_SPEED {
                        self.time_last_char_appeared = ctx.time.time_since_start();

                        let index: usize = self.displayed_text.contents().len();

                        if index < self.entire_text.len() {
                            self.add_char(ctx);
                        } else {
                            // text has finished displaying
                            self.time_finished = Some(ctx.time.time_since_start());
                        }
                    }
                }
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
                    HORIZONTAL_PADDING,
                    SCREEN_HEIGHT - self.image.height() as f32 + VERTICAL_PADDING,
                )),
            );
        }
    }

    fn add_char(&mut self, ctx: &Context) {
        let index: usize = self.displayed_text.contents().len();

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
            scale: Some(PxScale::from(FONT_SIZE)),
            ..Default::default()
        });

        // if the text overflows past the bottom of the screen
        match self.displayed_text.dimensions(&ctx.gfx) {
            Some(r) => {
                let max_height = self.image.height() as f32 - (2.0 * VERTICAL_PADDING);

                if r.h >= max_height {
                    // find first space from the end
                    let mut i = index;
                    while self.entire_text.as_bytes()[i] != b' ' {
                        i -= 1;
                    }

                    self.entire_text = self.entire_text[(i + 1)..].to_string();
                    self.displayed_text = Text::new("");
                    self.displayed_text.set_bounds(self.bounds);
                }
            }
            None => {}
        }
    }
}
