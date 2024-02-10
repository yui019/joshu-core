use std::sync::mpsc::Receiver;

use ggez::{
    event::EventHandler,
    glam::Vec2,
    graphics::{Canvas, Color, DrawParam, Image},
    Context, GameResult,
};

use crate::{message::Message, textbox::Textbox, SCREEN_HEIGHT, SCREEN_WIDTH};

pub struct App {
    input_receiver: Receiver<Message>,
    default_avatar_image: Image,
    textbox: Textbox,
}

impl App {
    pub fn new(ctx: &mut Context, input_receiver: Receiver<Message>) -> App {
        let default_avatar_image = Image::from_path(ctx, "/kurisu/arms_crossed_normal.png")
            .expect("Could not load default kurisu image!");

        let textbox = Textbox::new(ctx, default_avatar_image.width() as f32);

        App {
            input_receiver,
            default_avatar_image,
            textbox,
        }
    }
}

impl EventHandler for App {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        match self.input_receiver.try_recv() {
            Ok(message) => {
                // println!("Message: {:?}", message);

                match message.textbox_text {
                    Some(text) => {
                        self.textbox.set_text(ctx, &text);
                    }
                    None => self.textbox.hide(),
                }
            }

            Err(std::sync::mpsc::TryRecvError::Empty) => {}
            Err(std::sync::mpsc::TryRecvError::Disconnected) => panic!("wtf just happened??"),
        }

        self.textbox.update(ctx);

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, Color::new(0.0, 0.0, 0.0, 0.0));

        self.textbox.draw(&mut canvas);

        canvas.draw(
            &self.default_avatar_image,
            DrawParam::new().dest(Vec2::new(
                SCREEN_WIDTH - self.default_avatar_image.width() as f32,
                SCREEN_HEIGHT - self.default_avatar_image.height() as f32,
            )),
        );

        canvas.finish(ctx)
    }
}
