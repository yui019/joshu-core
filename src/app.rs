use std::sync::mpsc::Receiver;

use ggez::{
    event::EventHandler,
    glam::Vec2,
    graphics::{Canvas, Color, DrawParam, Image},
    winit::event::VirtualKeyCode,
    Context, GameResult,
};

use crate::{message::Message, textbox::Textbox, ui::Ui, SCREEN_HEIGHT, SCREEN_WIDTH};

pub struct App {
    input_receiver: Receiver<Message>,
    default_avatar_image: Image,
    textbox: Textbox,
    ui: Ui,
}

impl App {
    pub fn new(ctx: &mut Context, input_receiver: Receiver<Message>) -> App {
        let default_avatar_image = Image::from_path(ctx, "/kurisu/arms_crossed_normal.png")
            .expect("Could not load default kurisu image!");

        let textbox = Textbox::new(ctx, default_avatar_image.width() as f32);

        let ui = Ui::new(ctx);

        App {
            input_receiver,
            default_avatar_image,
            textbox,
            ui,
        }
    }

    fn handle_message(&mut self, ctx: &mut Context, message: Message) {
        match message.textbox_text {
            Some(text) => {
                self.textbox.set_text(ctx, &text);
            }
            None => self.textbox.hide(),
        }

        self.ui.set_type(ctx, message.ui);
    }
}

impl EventHandler for App {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        match self.input_receiver.try_recv() {
            Ok(message) => self.handle_message(ctx, message),

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

        self.ui.draw(ctx, &mut canvas);

        canvas.finish(ctx)
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        input: ggez::input::keyboard::KeyInput,
        _repeated: bool,
    ) -> Result<(), ggez::GameError> {
        match input.keycode {
            Some(VirtualKeyCode::Back) => self.ui.handle_backspace(ctx),

            Some(VirtualKeyCode::Escape) => ctx.request_quit(),
            _ => {}
        }
        Ok(())
    }

    fn text_input_event(&mut self, ctx: &mut Context, ch: char) -> Result<(), ggez::GameError> {
        self.ui.handle_text_input(ctx, ch);

        Ok(())
    }
}
