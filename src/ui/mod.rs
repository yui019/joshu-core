use std::str::from_utf8;

use ggez::{
    glam::Vec2,
    graphics::{
        Canvas, Color, DrawMode, DrawParam, Drawable, Mesh, PxScale, Rect, Text, TextFragment,
    },
    Context,
};
use serde::{Deserialize, Serialize};

use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};

const INPUT_TEXT_MIN_WIDTH: f32 = 500.0;
const INPUT_TEXT_MAX_WIDTH: f32 = 1800.0;
const INPUT_TEXT_FONT_SIZE: f32 = 32.0;
const INPUT_TEXT_HORIZONTAL_PADDING: f32 = 10.0;
const INPUT_TEXT_VERTICAL_PADDING: f32 = 15.0;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UiType {
    InputText,
    Select(Vec<String>),
}

enum UiState {
    InputText {
        displayed_text: Text,
        entire_text: String,
    },
    Select {
        options: Vec<String>,
        selected_option: Option<usize>,
    },
}

pub struct Ui {
    state: Option<UiState>,
    input_text_background: Mesh,
}

impl Ui {
    pub fn new(ctx: &mut Context) -> Self {
        let input_text_background = {
            let width = INPUT_TEXT_MIN_WIDTH;
            let height = INPUT_TEXT_FONT_SIZE + (2.0 * INPUT_TEXT_VERTICAL_PADDING);

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

        Self {
            state: None,
            input_text_background,
        }
    }

    pub fn set_type(&mut self, ctx: &Context, t: Option<UiType>) {
        match t {
            Some(UiType::InputText) => {
                // reset input width
                let rect = self.input_text_background.dimensions(&ctx.gfx).unwrap();
                self.input_text_background = Mesh::new_rectangle(
                    &ctx.gfx,
                    DrawMode::fill(),
                    Rect::new(
                        SCREEN_WIDTH / 2.0 - INPUT_TEXT_MIN_WIDTH / 2.0,
                        rect.y,
                        INPUT_TEXT_MIN_WIDTH,
                        rect.h,
                    ),
                    Color::WHITE,
                )
                .unwrap();

                self.state = Some(UiState::InputText {
                    displayed_text: Text::new(""),
                    entire_text: String::new(),
                });
            }

            Some(UiType::Select(options)) => {
                self.state = Some(UiState::Select {
                    options,
                    selected_option: None,
                });
            }

            None => {
                self.state = None;
            }
        }
    }

    pub fn draw(&self, ctx: &Context, canvas: &mut Canvas) {
        match &self.state {
            Some(UiState::InputText {
                displayed_text,
                entire_text: _,
            }) => {
                canvas.draw(&self.input_text_background, DrawParam::new());

                let background_rect = self.input_text_background.dimensions(&ctx.gfx).unwrap();
                let text_x = background_rect.x + INPUT_TEXT_HORIZONTAL_PADDING;
                let text_y =
                    background_rect.y + background_rect.h / 2.0 - (INPUT_TEXT_FONT_SIZE / 2.0);
                canvas.draw(
                    displayed_text,
                    DrawParam::new().dest(Vec2::new(text_x, text_y)),
                );
            }

            Some(UiState::Select {
                options: _,
                selected_option: _,
            }) => {}

            None => {}
        }
    }

    pub fn handle_text_input(&mut self, ctx: &Context, inputted_char: char) {
        match &mut self.state {
            Some(UiState::InputText {
                displayed_text,
                entire_text,
            }) => {
                if !(inputted_char.is_alphanumeric()
                    || inputted_char.is_ascii_punctuation()
                    || inputted_char == ' ')
                {
                    return;
                }

                // add inputted char to text
                *entire_text = format!("{}{}", entire_text, inputted_char);
                displayed_text.add(TextFragment {
                    text: inputted_char.to_string(),
                    color: Some(Color::BLACK),
                    scale: Some(PxScale::from(INPUT_TEXT_FONT_SIZE)),
                    ..Default::default()
                });

                // handle text overflowing the input field
                let text_rect = displayed_text.dimensions(&ctx.gfx).unwrap();
                let background_rect = self.input_text_background.dimensions(&ctx.gfx).unwrap();

                if text_rect.w + (2.0 * INPUT_TEXT_HORIZONTAL_PADDING) >= background_rect.w {
                    let new_width = text_rect.w + (2.0 * INPUT_TEXT_HORIZONTAL_PADDING);

                    if new_width <= INPUT_TEXT_MAX_WIDTH {
                        // just expand the input field if there's still space left
                        self.input_text_background = Mesh::new_rectangle(
                            &ctx.gfx,
                            DrawMode::fill(),
                            Rect::new(
                                SCREEN_WIDTH / 2.0 - new_width / 2.0,
                                background_rect.y,
                                new_width,
                                background_rect.h,
                            ),
                            Color::WHITE,
                        )
                        .unwrap();
                    } else {
                        // if the input field is already maximally expanded, then scroll the text so its end is visible
                        *displayed_text = {
                            let mut new_text = Text::new("");

                            let start = entire_text.len() - displayed_text.contents().len() + 1;
                            for i in start..entire_text.len() {
                                let text =
                                    from_utf8(&[entire_text.as_bytes()[i]]).unwrap().to_string();

                                new_text.add(TextFragment {
                                    text,
                                    color: Some(Color::BLACK),
                                    scale: Some(PxScale::from(INPUT_TEXT_FONT_SIZE)),
                                    ..Default::default()
                                });
                            }

                            new_text
                        };
                    }
                }
            }

            _ => {}
        }
    }

    pub fn handle_backspace(&mut self, ctx: &Context) {
        match &mut self.state {
            Some(UiState::InputText {
                displayed_text,
                entire_text,
            }) => {
                // remove last character
                if entire_text.len() == 0 {
                    return;
                }

                entire_text.remove(entire_text.len() - 1);
                *displayed_text = {
                    let old_displayed_len = displayed_text.contents().len();

                    if old_displayed_len - 1 < entire_text.len() {
                        // if there's more text than what's being displayed, handle that properly
                        let mut new_text = Text::new("");
                        for i in (entire_text.len() - old_displayed_len)..entire_text.len() {
                            let text = from_utf8(&[entire_text.as_bytes()[i]]).unwrap().to_string();

                            new_text.add(TextFragment {
                                text,
                                color: Some(Color::BLACK),
                                scale: Some(PxScale::from(INPUT_TEXT_FONT_SIZE)),
                                ..Default::default()
                            });
                        }

                        new_text
                    } else {
                        let remaining_fragments = displayed_text.fragments()
                            [..(displayed_text.fragments().len() - 1)]
                            .to_vec();

                        let mut new_text = Text::new("");
                        for fr in remaining_fragments {
                            new_text.add(fr);
                        }

                        new_text
                    }
                };

                // reduce input width (up to the minimum)
                let text_rect = displayed_text.dimensions(&ctx.gfx).unwrap();
                let new_width = text_rect.w + (2.0 * INPUT_TEXT_HORIZONTAL_PADDING);
                if new_width >= INPUT_TEXT_MIN_WIDTH {
                    let background_rect = self.input_text_background.dimensions(&ctx.gfx).unwrap();

                    self.input_text_background = Mesh::new_rectangle(
                        &ctx.gfx,
                        DrawMode::fill(),
                        Rect::new(
                            SCREEN_WIDTH / 2.0 - new_width / 2.0,
                            background_rect.y,
                            new_width,
                            background_rect.h,
                        ),
                        Color::WHITE,
                    )
                    .unwrap();
                }
            }

            _ => {}
        }
    }
}
