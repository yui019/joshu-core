use ggez::conf::{WindowMode, WindowSetup};
use ggez::event::{self};
use ggez::ContextBuilder;
use joshu_core::app::App;
use joshu_core::message::Message;
use joshu_core::{SCREEN_HEIGHT, SCREEN_WIDTH};
use std::fs::{File, OpenOptions};
use std::io::Read;
use std::{env, path};
use std::{
    io,
    sync::mpsc::{channel, Receiver},
    thread,
};

fn main() {
    let args = env::args();

    let mut out_pipe: Option<File> = None;
    let mut in_pipe: Option<File> = None;
    if args.len() == 2 {
        let out_path = &args.collect::<Vec<String>>()[1];
        let out_fd = OpenOptions::new()
            .write(true)
            .open(out_path)
            .expect(&format!("Pipe {} does not exist", out_path));

        out_pipe = Some(out_fd);
    } else if args.len() == 3 {
        let args_vec = args.collect::<Vec<String>>();
        let out_path = &args_vec[1];
        let in_path = &args_vec[2];

        let out_fd = OpenOptions::new()
            .write(true)
            .open(out_path)
            .expect(&format!("Pipe {} does not exist", out_path));

        let in_fd = OpenOptions::new()
            .read(true)
            .open(in_path)
            .expect(&format!("Pipe {} does not exist", in_path));

        out_pipe = Some(out_fd);
        in_pipe = Some(in_fd);
    }

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

    let receiver = run_input_receiver(in_pipe);

    let my_game = App::new(&mut ctx, receiver, out_pipe);

    event::run(ctx, event_loop, my_game);
}

fn run_input_receiver(mut in_pipe: Option<File>) -> Receiver<Message> {
    let (sender, receiver) = channel();

    // this buffer is used to read data from the pipe
    // for some fucking reason, reading to a string doesn't work, so I'm just using a huge 0.5MB static buffer instead
    // TODO: either figure out why read_to_string doesn't work or add multiple smaller buffers together to only allocate the amount necessary
    let mut buffer = [0; 1000 * 500];

    thread::spawn(move || loop {
        let (message, size) = match &mut in_pipe {
            Some(pipe) => {
                let size = pipe.read(&mut buffer).unwrap();

                let buffer_str = std::str::from_utf8(&buffer[..size]).unwrap();

                (buffer_str.to_string(), size)
            }

            None => {
                let mut buffer = String::new();
                let size = io::stdin().read_line(&mut buffer).unwrap();

                (buffer, size)
            }
        };

        if size > 0 {
            if let Ok(message) = serde_json::from_str::<Message>(&message) {
                sender.send(message).unwrap();
            }
        }
    });

    return receiver;
}
