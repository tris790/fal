use fltk::{
    button::Button,
    draw::*,
    enums::{Align, Color, Event, FrameType, Key, Shortcut},
    frame::Frame,
    group::{Group, Pack},
    input::Input,
    prelude::*,
    text::TextDisplay,
    window::{DoubleWindow, Window},
};

use hotkey::{self, keys, modifiers, Listener};
use std::{process::Command, thread};

enum FalAction {
    NONE,
    LAUNCH(String),
}

impl FalAction {
    fn execute(&self) {
        match self {
            Self::LAUNCH(cmd) => {
                let output = if cfg!(target_os = "windows") {
                    Command::new("cmd")
                        .args(&["/C", cmd])
                        .output()
                        .expect("failed to execute process")
                } else {
                    Command::new("sh")
                        .arg("-c")
                        .arg(cmd)
                        .output()
                        .expect("failed to execute process")
                };
            }
            _ => (),
        }
    }
}
enum FalMessage {
    KeybindPressed(Keybind),
}
enum Keybind {
    KB_OPEN_FAL,
    KB_CLOSE_FAL,
}
struct ProgramResult {
    text: String,
    cmd: String,
}

fn get_programs() -> Vec<ProgramResult> {
    vec![
        ProgramResult {
            text: String::from("Terminal"),
            cmd: String::from("wt"),
        },
        ProgramResult {
            text: String::from("Vs Code"),
            cmd: String::from("code ."),
        },
        ProgramResult {
            text: String::from("Calculator"),
            cmd: String::from("calc.exe"),
        },
    ]
}

struct ListElement {
    inner: Button,
    action: FalAction,
}

impl ListElement {
    pub fn new(text: &str, action: FalAction) -> ListElement {
        let mut button = Button::default()
            .with_pos(10, 10)
            .with_size(80, 40)
            .with_label(text);
        button.set_down_frame(FrameType::FlatBox);
        button.set_frame(FrameType::FlatBox);
        button.set_color(Color::from_rgb(51, 51, 51));
        button.set_label(&text);
        button.set_label_size(20);
        button.set_selection_color(Color::Cyan);
        button.visible_focus(false);

        ListElement {
            inner: button,
            action,
        }
    }

    pub fn set_color(&mut self, color: Color) {
        self.inner.set_color(color);
        self.inner.redraw();
    }

    pub fn pressed(&self) {
        self.action.execute();
    }
}

struct FalApp {
    window: Window,
    app: app::App,
    selected_element: ListElement,
    search: Input,
}

impl FalApp {
    fn new() -> FalApp {
        let app = app::App::default();

        let mut window = Window::default()
            .with_size(WINDOW_WIDTH, WINDOW_HEIGHT)
            .center_screen();
        window.set_color(Color::from_hex(0x9CA3AF));
        window.set_border(false);

        let programs = get_programs();
        let mut input = Input::default()
            .with_pos(0, 0)
            .with_size(WINDOW_WIDTH, LIST_ITEM_HEIGHT);
        input.set_text_size(30);

        let pack = Pack::new(
            0,
            LIST_ITEM_HEIGHT,
            WINDOW_WIDTH,
            WINDOW_HEIGHT - LIST_ITEM_HEIGHT,
            "",
        );
        let mut selected_element = ListElement::new("", FalAction::NONE);
        for (index, program) in programs.iter().enumerate() {
            selected_element = ListElement::new(
                program.text.as_str(),
                FalAction::LAUNCH(program.cmd.to_string()),
            );
        }
        pack.end();

        FalApp {
            window,
            app,
            selected_element,
            search: input,
        }
    }

    fn toggle_visibilty(&mut self) {
        if self.window.visible() {
            println!("Window was visible");
            self.window.iconize();
        } else {
            println!("Window was hidden");
            self.window.show();
            println!("Window is now {}", self.window.visible());
        }
    }

    fn build(&mut self) {
        self.window.end();
        self.window.show();
        self.search.take_focus().ok();
        self.search.set_visible_focus();
    }
}

const WINDOW_WIDTH: i32 = 800;
const WINDOW_HEIGHT: i32 = 500;
const LIST_ITEM_WIDTH: i32 = WINDOW_WIDTH;
const LIST_ITEM_HEIGHT: i32 = 50;
const MAX_ITEM_COUNT: i32 = (WINDOW_HEIGHT / LIST_ITEM_HEIGHT) as i32;

static mut FAL_APP_GLOBAL: Option<FalApp> = None;

fn toggle_fal_visibility() {
    if let Some(fal_app) = unsafe { &mut FAL_APP_GLOBAL } {
        fal_app.toggle_visibilty();
    }
}

fn main() {
    unsafe { FAL_APP_GLOBAL = Some(FalApp::new()) };
    if let Some(fal_app) = unsafe { &mut FAL_APP_GLOBAL } {
        fal_app.build();

        let (send_channel, recv_channel) = app::channel::<FalMessage>();

        std::thread::spawn(move || {
            let mut hotkey = hotkey::Listener::new();
            hotkey
                .register_hotkey(modifiers::CONTROL, keys::SPACEBAR, move || {
                    println!("pressed");
                    send_channel.send(FalMessage::KeybindPressed(Keybind::KB_OPEN_FAL));
                })
                .unwrap();
            hotkey.listen();
        });

        while fal_app.app.wait() {
            match recv_channel.recv() {
                Some(FalMessage::KeybindPressed(keybind)) => match keybind {
                    Keybind::KB_OPEN_FAL => toggle_fal_visibility(),
                    _ => unimplemented!(),
                },
                _ => (),
            }
        }
    }
    println!("Oups");
}
