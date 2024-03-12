use std::sync::mpsc::{channel, Receiver, Sender};

use ggez::{
    event::EventHandler,
    glam::Vec2,
    graphics::{Color, DrawParam, Image},
    winit::event::VirtualKeyCode,
    Context, GameResult,
};

use crate::{canvas::Canvas, message::Message, textbox::Textbox, SCREEN_HEIGHT, SCREEN_WIDTH};

// A message sent to App when a command has finished (textbox finished displaying text or user inputted something when asked)
pub enum FinishedMessage {
    Textbox,
    UserInput(String),
}

pub struct App {
    input_receiver: Receiver<Message>,
    finished_receiver: Receiver<FinishedMessage>,
    default_avatar_image: Image,
    textbox: Textbox,
    canvas: Canvas,
}

impl App {
    pub fn new(ctx: &mut Context, input_receiver: Receiver<Message>) -> App {
        let default_avatar_image = Image::from_path(ctx, "/kurisu/arms_crossed_normal.png")
            .expect("Could not load default kurisu image!");

        let (finished_sender, finished_receiver): (
            Sender<FinishedMessage>,
            Receiver<FinishedMessage>,
        ) = channel();

        let textbox = Textbox::new(
            ctx,
            default_avatar_image.width() as f32,
            finished_sender.clone(),
        );

        let canvas = Canvas::new(ctx, finished_sender.clone());

        App {
            input_receiver,
            finished_receiver,
            default_avatar_image,
            textbox,
            canvas,
        }
    }

    fn handle_message(&mut self, ctx: &mut Context, message: Message) {
        match message.textbox_text {
            Some(text) => {
                let finished_sender_enabled = message.canvas_mode.is_none();
                self.textbox.set_text(ctx, &text, finished_sender_enabled);
            }
            None => self.textbox.hide(),
        }

        self.canvas.set_mode(ctx, message.canvas_mode);
    }
}

impl EventHandler for App {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        match self.input_receiver.try_recv() {
            Ok(message) => self.handle_message(ctx, message),

            Err(std::sync::mpsc::TryRecvError::Empty) => {}
            Err(std::sync::mpsc::TryRecvError::Disconnected) => panic!("wtf just happened??"),
        }

        match self.finished_receiver.try_recv() {
            Ok(FinishedMessage::Textbox) => println!("Finished displaying text"),
            Ok(FinishedMessage::UserInput(str)) => {
                self.canvas.set_mode(ctx, None);
                println!("{}", str);
            }

            Err(std::sync::mpsc::TryRecvError::Empty) => {}
            Err(std::sync::mpsc::TryRecvError::Disconnected) => panic!("wtf just happened??"),
        }

        self.textbox.update(ctx);

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = ggez::graphics::Canvas::from_frame(ctx, Color::new(0.0, 0.0, 0.0, 0.0));

        self.textbox.draw(&mut canvas);

        canvas.draw(
            &self.default_avatar_image,
            DrawParam::new().dest(Vec2::new(
                SCREEN_WIDTH - self.default_avatar_image.width() as f32,
                SCREEN_HEIGHT - self.default_avatar_image.height() as f32,
            )),
        );

        self.canvas.draw(ctx, &mut canvas);

        canvas.finish(ctx)
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        input: ggez::input::keyboard::KeyInput,
        _repeated: bool,
    ) -> Result<(), ggez::GameError> {
        match input.keycode {
            Some(VirtualKeyCode::Back) => self.canvas.handle_backspace(ctx),

            Some(VirtualKeyCode::Return) => self.canvas.handle_enter(&ctx),

            Some(VirtualKeyCode::Escape) => ctx.request_quit(),

            Some(VirtualKeyCode::Left)
            | Some(VirtualKeyCode::Right)
            | Some(VirtualKeyCode::Up)
            | Some(VirtualKeyCode::Down) => {
                self.canvas.handle_arrow_key(&ctx, input.keycode.unwrap())
            }

            _ => {}
        }
        Ok(())
    }

    fn text_input_event(&mut self, ctx: &mut Context, ch: char) -> Result<(), ggez::GameError> {
        self.canvas.handle_text_input(ctx, ch);

        Ok(())
    }
}
