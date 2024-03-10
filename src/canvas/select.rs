use ggez::{
    glam::Vec2,
    graphics::{Color, DrawParam, PxScale, Rect, Text, TextFragment},
    winit::event::VirtualKeyCode,
};

use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};

use super::{
    input_text::{InputTextConfig, InputTextHandler},
    CanvasModeHandler,
};

#[derive(Clone)]
pub struct SelectConfig {
    pub input_text_config: InputTextConfig,
    pub max_options_shown: u32,
    pub x_position: f32,
    pub y_position: f32,
}

impl Default for SelectConfig {
    fn default() -> Self {
        Self {
            input_text_config: Default::default(),
            max_options_shown: 5,
            x_position: SCREEN_WIDTH / 2.0,
            y_position: SCREEN_HEIGHT / 2.0,
        }
    }
}

pub struct SelectHandler {
    pub config: SelectConfig,
    pub input_text_handler: InputTextHandler,
    pub background_rect: Rect,
    pub all_options_strings: Vec<String>,
    pub all_options: Vec<Text>,
    pub filtered_options_indexes: Vec<usize>,
    pub filtered_options_visible_indexes: Vec<usize>,
    pub selected_option: Option<usize>, // from 0 to filtered_options_visible_indexes.len()
}

impl SelectHandler {
    fn strings_match(original: &str, query: &str) -> bool {
        let lowercase_original = original.trim().to_lowercase();
        let lowercase_query = query.trim().to_lowercase();

        lowercase_original.contains(&lowercase_query)
    }

    fn filter_options(&mut self) {
        self.filtered_options_indexes.clear();
        self.filtered_options_visible_indexes.clear();

        let mut num_visible = 0;
        for i in 0..self.all_options.len() {
            let option = &self.all_options_strings[i];
            let query = &self.input_text_handler.entire_text;

            if Self::strings_match(option, query) {
                self.filtered_options_indexes.push(i);

                if num_visible < self.config.max_options_shown as usize {
                    self.filtered_options_visible_indexes.push(i);
                    num_visible += 1;
                }
            }
        }

        // reset selected option
        if self.filtered_options_visible_indexes.is_empty() {
            self.selected_option = None;
        } else {
            self.selected_option = Some(0);
        }
    }
}

impl CanvasModeHandler for SelectHandler {
    type ConfigData = SelectConfig;
    type SetupData = Vec<String>;

    fn new(ctx: &mut ggez::Context, config: &Self::ConfigData) -> Self {
        let mut config = config.clone();

        let background_rect = {
            let input_text_background =
                InputTextHandler::get_initial_background_rect(&config.input_text_config);

            let option_height = config.input_text_config.text_font_size
                + (2.0 * config.input_text_config.text_vertical_padding);

            let total_height =
                input_text_background.h + option_height * config.max_options_shown as f32;

            Rect::new(
                config.x_position - input_text_background.w / 2.0,
                config.y_position - total_height / 2.0,
                input_text_background.w,
                total_height,
            )
        };

        // Set the input text's y position to the top of the entire background rect
        config.input_text_config.y_position = background_rect.y;

        let input_text_handler = InputTextHandler::new(ctx, &config.input_text_config);

        Self {
            config,
            input_text_handler,
            background_rect,
            all_options: vec![],
            all_options_strings: vec![],
            filtered_options_indexes: vec![],
            filtered_options_visible_indexes: vec![],
            selected_option: None,
        }
    }

    fn setup(&mut self, ggez_ctx: &mut ggez::Context, data: Self::SetupData) {
        self.input_text_handler.setup(ggez_ctx, ());

        self.all_options.clear();
        self.all_options_strings.clear();
        for option in data {
            self.all_options_strings.push(option.clone());

            let fragment = TextFragment {
                text: option,
                color: Some(Color::BLACK),
                scale: Some(PxScale::from(self.input_text_handler.config.text_font_size)),
                ..Default::default()
            };

            self.all_options.push(Text::new(fragment.clone()));
        }

        self.filtered_options_indexes.clear();
        self.filtered_options_visible_indexes.clear();
        for i in 0..self.all_options.len() {
            self.filtered_options_indexes.push(i);

            if i < self.config.max_options_shown as usize {
                self.filtered_options_visible_indexes.push(i);
            }
        }

        self.selected_option = Some(0);
    }

    fn draw(
        &self,
        ggez_ctx: &mut ggez::Context,
        ggez_canvas: &mut ggez::graphics::Canvas,
        canvas_ctx: &super::CanvasContext,
    ) {
        // draw select background
        canvas_ctx.draw_rect(ggez_canvas, &self.background_rect, &Color::WHITE);

        // draw input text
        self.input_text_handler
            .draw(ggez_ctx, ggez_canvas, canvas_ctx);

        // draw selected option background
        match self.selected_option {
            Some(selected_option) => {
                let vertical_padding = self.input_text_handler.config.text_vertical_padding;
                let font_size = self.input_text_handler.config.text_font_size;
                let input_height = self.input_text_handler.background_rect.h;
                let input_outline = self.input_text_handler.config.background_outline_width;

                let x = self.background_rect.x;

                let y = self.background_rect.y - input_height / 2.0
                    + font_size / 2.0
                    + input_height
                    + input_outline
                    + (selected_option as f32) * (font_size + 2.0 * vertical_padding);

                let rect = Rect::new(
                    x,
                    y,
                    self.background_rect.w,
                    font_size + 2.0 * vertical_padding,
                );

                canvas_ctx.draw_rect(ggez_canvas, &rect, &Color::from_rgba(0, 0, 0, 100));
            }

            None => {}
        }

        // draw all options
        let mut i = 0;
        for option_index in &self.filtered_options_visible_indexes {
            let horizontal_padding = self.input_text_handler.config.text_horizontal_padding;
            let vertical_padding = self.input_text_handler.config.text_vertical_padding;
            let font_size = self.input_text_handler.config.text_font_size;
            let input_height = self.input_text_handler.background_rect.h;
            let input_outline = self.input_text_handler.config.background_outline_width;

            let x = self.background_rect.x + horizontal_padding;

            let y = self.background_rect.y - input_height / 2.0
                + font_size / 2.0
                + input_height
                + input_outline
                + vertical_padding
                + (i as f32) * (font_size + 2.0 * vertical_padding);

            ggez_canvas.draw(
                &self.all_options[*option_index],
                DrawParam::new().dest(Vec2::new(x, y)),
            );

            i += 1;
        }
    }

    fn handle_text_input(&mut self, ggez_ctx: &ggez::Context, inputted_char: char) {
        self.input_text_handler
            .handle_text_input(ggez_ctx, inputted_char);

        self.background_rect.w = self.input_text_handler.background_rect.w;
        self.background_rect.x = self.input_text_handler.background_rect.x;

        self.filter_options();
    }

    fn handle_backspace(&mut self, ggez_ctx: &ggez::Context) {
        self.input_text_handler.handle_backspace(ggez_ctx);

        self.background_rect.w = self.input_text_handler.background_rect.w;
        self.background_rect.x = self.input_text_handler.background_rect.x;

        self.filter_options();
    }

    fn handle_arrow_key(&mut self, _ggez_ctx: &ggez::Context, keycode: VirtualKeyCode) {
        match keycode {
            VirtualKeyCode::Up => match self.selected_option {
                Some(selected_option) => {
                    if selected_option > 0 {
                        self.selected_option = Some(selected_option - 1);
                    } else {
                        let visible_first = self.filtered_options_visible_indexes.first().unwrap();
                        let mut start = 0;
                        for (i, option) in self.filtered_options_indexes.iter().enumerate() {
                            if option == visible_first {
                                start = i;
                                break;
                            }
                        }

                        let new_index = start as i32 - 1;
                        if new_index >= 0 {
                            let new_first_option = self.filtered_options_indexes[start - 1];
                            self.filtered_options_visible_indexes
                                .insert(0, new_first_option);

                            let last_index = self.filtered_options_visible_indexes.len() - 1;
                            self.filtered_options_visible_indexes.remove(last_index);
                        }
                    }
                }

                None => {}
            },

            VirtualKeyCode::Down => match self.selected_option {
                Some(selected_option) => {
                    if selected_option < self.filtered_options_visible_indexes.len() - 1 {
                        self.selected_option = Some(selected_option + 1);
                    } else {
                        let visible_last = self.filtered_options_visible_indexes.last().unwrap();
                        let mut end = 0;
                        for (i, option) in self.filtered_options_indexes.iter().enumerate() {
                            if option == visible_last {
                                end = i;
                                break;
                            }
                        }

                        if end + 1 < self.filtered_options_indexes.len() {
                            let new_last_option = self.filtered_options_indexes[end + 1];
                            self.filtered_options_visible_indexes.push(new_last_option);
                            self.filtered_options_visible_indexes.remove(0);
                        }
                    }
                }

                None => {}
            },

            _ => {}
        }
    }
}
