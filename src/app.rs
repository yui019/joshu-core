use std::{
    collections::{HashMap, VecDeque},
    fs::File,
    io::Write,
    sync::mpsc::{channel, Receiver, Sender},
};

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

#[derive(PartialEq, Eq)]
pub enum AppState {
    Idle,
    ExecutingCommand(Message),
}

pub struct App {
    out_pipe: Option<File>,
    input_receiver: Receiver<Message>,
    finished_receiver: Receiver<FinishedMessage>,
    current_state: AppState,
    default_avatar_image: String,
    current_avatar_image: String,
    avatar_images: HashMap<String, Image>,
    textbox: Textbox,
    canvas: Canvas,
    message_queue: VecDeque<Message>,
    executed_command: bool,
}

impl App {
    pub fn new(
        ctx: &mut Context,
        input_receiver: Receiver<Message>,
        out_pipe: Option<File>,
    ) -> App {
        let avatar_images = HashMap::from([
            (
                String::from("normal"),
                Image::from_path(ctx, "/kurisu/normal.png").unwrap(),
            ),
            (
                String::from("embarrassed"),
                Image::from_path(ctx, "/kurisu/embarrassed.png").unwrap(),
            ),
            (
                String::from("emotionless"),
                Image::from_path(ctx, "/kurisu/emotionless.png").unwrap(),
            ),
            (
                String::from("pleased"),
                Image::from_path(ctx, "/kurisu/pleased.png").unwrap(),
            ),
            (
                String::from("winking"),
                Image::from_path(ctx, "/kurisu/winking.png").unwrap(),
            ),
            (
                String::from("angry1"),
                Image::from_path(ctx, "/kurisu/angry1.png").unwrap(),
            ),
            (
                String::from("angry2"),
                Image::from_path(ctx, "/kurisu/angry2.png").unwrap(),
            ),
            (
                String::from("angry3"),
                Image::from_path(ctx, "/kurisu/angry3.png").unwrap(),
            ),
        ]);

        let (finished_sender, finished_receiver): (
            Sender<FinishedMessage>,
            Receiver<FinishedMessage>,
        ) = channel();

        let textbox = Textbox::new(
            ctx,
            avatar_images["normal"].width() as f32,
            finished_sender.clone(),
        );

        let canvas = Canvas::new(ctx, finished_sender.clone());

        App {
            out_pipe,
            input_receiver,
            finished_receiver,
            current_state: AppState::Idle,
            default_avatar_image: String::from("normal"),
            current_avatar_image: String::from("normal"),
            avatar_images,
            textbox,
            canvas,
            message_queue: VecDeque::new(),
            executed_command: false,
        }
    }

    fn handle_message(&mut self, ctx: &mut Context, message: Message) {
        if matches!(self.current_state, AppState::ExecutingCommand(_)) {
            // if there's a command currently executing, add the message to the queue
            self.message_queue.push_back(message);
        } else {
            self.current_state = AppState::ExecutingCommand(message.clone());

            // handle textbox_text inside message
            match message.textbox_text {
                Some(text) => {
                    // only enable finished_listener for the textbox if there's no canvas_mode in the message
                    let finished_sender_enabled = message.canvas_mode.is_none();
                    self.textbox.set_text(ctx, &text, finished_sender_enabled);
                }
                None => self.textbox.hide(),
            }

            // handle canvas_mode inside message
            self.canvas.set_mode(ctx, message.canvas_mode);

            // handle avatar image
            match message.avatar_emotion {
                Some(emotion) => {
                    if self.avatar_images.contains_key(&emotion) {
                        self.current_avatar_image = emotion;
                    }
                }
                None => {}
            }
        }
    }

    fn make_finished_message(data: String, id: Option<String>) -> String {
        match id {
            Some(id) => format!("{{\"id\": \"{}\", \"data\": \"{}\"}}", id, data),
            None => format!("{{\"data\": \"{}\"}}", data),
        }
    }

    fn output_message(&mut self, data: String, id: Option<String>) {
        let message = format!("{}\n", Self::make_finished_message(data, id));

        match self.out_pipe.as_mut() {
            Some(out_pipe) => out_pipe.write_all(message.as_bytes()).unwrap(),

            None => print!("{}", message),
        }
    }
}

impl EventHandler for App {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // input was received
        match self.input_receiver.try_recv() {
            Ok(message) => self.handle_message(ctx, message),

            Err(std::sync::mpsc::TryRecvError::Empty) => {}
            Err(std::sync::mpsc::TryRecvError::Disconnected) => panic!("wtf just happened??"),
        }

        // command has finished
        match self.finished_receiver.try_recv() {
            Ok(message) => {
                let executed_command_message = match &self.current_state {
                    AppState::Idle => unreachable!(),
                    AppState::ExecutingCommand(m) => m.clone(),
                };

                // return to idle state
                self.current_state = AppState::Idle;

                // reset avatar image
                self.current_avatar_image = self.default_avatar_image.clone();

                // marks that at least 1 command has been executed
                self.executed_command = true;

                let finished_message = match message {
                    FinishedMessage::Textbox => format!("Finished displaying text"),
                    FinishedMessage::UserInput(str) => {
                        self.canvas.set_mode(ctx, None);
                        format!("{}", str)
                    }
                };

                self.output_message(finished_message, executed_command_message.id);
            }

            Err(std::sync::mpsc::TryRecvError::Empty) => {}
            Err(std::sync::mpsc::TryRecvError::Disconnected) => panic!("wtf just happened??"),
        }

        self.textbox.update(ctx);

        let idle = self.current_state == AppState::Idle;
        if idle {
            let messages_left = !self.message_queue.is_empty();

            if messages_left {
                let message = self.message_queue.pop_front().unwrap();
                self.handle_message(ctx, message);
            } else if self.executed_command {
                // if there's no more commands to execute, and at least 1 has been executed already, then quit
                ctx.request_quit();
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = ggez::graphics::Canvas::from_frame(ctx, Color::new(0.0, 0.0, 0.0, 0.0));

        self.textbox.draw(&mut canvas);

        // draw avatar
        let avatar_image = &self.avatar_images[&self.current_avatar_image];
        canvas.draw(
            avatar_image,
            DrawParam::new().dest(Vec2::new(
                SCREEN_WIDTH - avatar_image.width() as f32,
                SCREEN_HEIGHT - avatar_image.height() as f32,
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

            Some(VirtualKeyCode::Escape) => {
                // if a command was executing, then send a message saying that joshu is quitting
                // but maybe the command that's currently executing was submitted by a message with an id, so extract that here first
                match &self.current_state {
                    AppState::ExecutingCommand(Message { id: Some(id), .. }) => {
                        self.output_message(format!("Quitting..."), Some(id.clone()));
                    }

                    _ => {}
                };

                ctx.request_quit();
            }

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
