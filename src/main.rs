use ggez::conf::{WindowMode, WindowSetup};
use ggez::event::{self};
use ggez::ContextBuilder;
use joshu_core::app::App;
use joshu_core::canvas::CanvasMode;
use joshu_core::message::Message;
use joshu_core::{SCREEN_HEIGHT, SCREEN_WIDTH};
use std::path;
use std::{
    io,
    sync::mpsc::{channel, Receiver},
    thread,
};

fn main() {
    let resource_dir = path::PathBuf::from("./res");

    let (mut ctx, event_loop) = ContextBuilder::new("Joshu", "")
        .window_setup(WindowSetup {
            title: String::from("Project Joshu"),
            ..Default::default()
        })
        .window_mode(WindowMode {
            width: SCREEN_WIDTH,
            height: SCREEN_HEIGHT,
            transparent: true,
            fullscreen_type: ggez::conf::FullscreenType::True,
            ..Default::default()
        })
        .add_resource_path(resource_dir)
        .build()
        .expect("Could not create ggez context!");

    let receiver = run_input_receiver();

    let my_game = App::new(&mut ctx, receiver);

    event::run(ctx, event_loop, my_game);
}

fn run_input_receiver() -> Receiver<Message> {
    let (sender, receiver) = channel();
    thread::spawn(move || loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();

        if let Ok(message) = serde_json::from_str::<Message>(&buffer) {
            sender.send(message).unwrap();
        }
    });

    return receiver;
}
