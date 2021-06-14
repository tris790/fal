use std::sync::Arc;

use fltk::{
    app::{self, event_key, event_key_down, Receiver},
    enums::{Color, Event, Key, Shortcut},
    group::Pack,
    input::Input,
    prelude::*,
    window::Window,
};
use hotkey::{self, keys, modifiers};

use crate::fal_list_element::ListElement;
use crate::fal_message::*;
use crate::{fal_action::FalAction, fal_list_element::SelectedState};

const WINDOW_WIDTH: i32 = 800;
const WINDOW_HEIGHT: i32 = 500;
const LIST_ITEM_WIDTH: i32 = WINDOW_WIDTH;
const LIST_ITEM_HEIGHT: i32 = 50;
const MAX_ITEM_COUNT: i32 = (WINDOW_HEIGHT / LIST_ITEM_HEIGHT) as i32;

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

pub struct FalApp {
    window: Window,
    app: app::App,
    elements: Vec<ListElement>,
    search: Input,
    recv_channel: Receiver<FalMessage>,
    selected_index: usize,
}

impl FalApp {
    pub fn new() -> FalApp {
        let app = app::App::default();
        let (send_channel, recv_channel) = app::channel::<FalMessage>();
        let (send_channel_thread, _) = app::channel::<FalMessage>();

        let mut window = Window::default()
            .with_size(WINDOW_WIDTH, WINDOW_HEIGHT)
            .center_screen();
        window.set_color(Color::from_hex(0x9CA3AF));
        window.set_border(false);

        let programs = get_programs();
        let mut search = Input::default()
            .with_pos(0, 0)
            .with_size(WINDOW_WIDTH, LIST_ITEM_HEIGHT);
        search.set_text_size(30);

        let pack = Pack::new(
            0,
            LIST_ITEM_HEIGHT,
            WINDOW_WIDTH,
            WINDOW_HEIGHT - LIST_ITEM_HEIGHT,
            "",
        );

        let mut elements: Vec<ListElement> = Vec::new();
        for (index, program) in programs.iter().enumerate() {
            elements.push(ListElement::new(
                program.text.as_str(),
                FalAction::LAUNCH(program.cmd.to_string()),
            ));
        }
        pack.end();

        std::thread::spawn(move || {
            let mut hotkey = hotkey::Listener::new();
            hotkey
                .register_hotkey(modifiers::CONTROL, keys::SPACEBAR, move || {
                    println!("global hotkey");
                    send_channel_thread.send(FalMessage::GlobalHotkeyTriggered(
                        KeyboardHookKeybind::OpenToggleFalVisibilty,
                    ));
                })
                .unwrap();
            hotkey.listen();
        });

        search.handle(move |a, ev| match ev {
            Event::KeyDown => {
                println!("key pressed");
                if event_key_down(Key::Down) {
                    println!("key down");
                    send_channel.send(FalMessage::KeybindPressed(Keybind::SelectionDown));
                    return true;
                } else if event_key_down(Key::Up) {
                    println!("key up");
                    send_channel.send(FalMessage::KeybindPressed(Keybind::SelectionUp));
                    return true;
                } else if event_key_down(Key::Enter) {
                    send_channel.send(FalMessage::KeybindPressed(Keybind::Execute));
                    return true;
                } else {
                    return false;
                }
            }
            _ => false,
        });

        FalApp {
            window,
            app,
            elements,
            search,
            recv_channel,
            selected_index: 0,
        }
    }

    fn set_selected_element(&mut self, new_selected: usize) {
        let selected_element = self.elements.get_mut(self.selected_index).unwrap();
        selected_element.set_selected(SelectedState::NotSelected);

        if new_selected >= self.elements.len() {
            println!("overflow");
            self.selected_index = 0;
        } else {
            self.selected_index = new_selected;
        }

        println!(
            "select {} size {}",
            self.selected_index,
            self.elements.len()
        );
        let new_selected_element = self.elements.get_mut(self.selected_index).unwrap();
        new_selected_element.set_selected(SelectedState::Selected);
    }

    fn execute_selected_element(&mut self) {
        self.elements.get(self.selected_index).unwrap().execute();
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

    pub fn run(&mut self) {
        self.window.end();
        self.window.show();
        // self.search.take_focus().ok();
        // self.search.set_visible_focus();

        while self.app.wait() {
            match self.recv_channel.recv() {
                Some(FalMessage::KeybindPressed(keybind)) => match keybind {
                    Keybind::SelectionUp => {
                        println!("recv up");
                        if self.selected_index == 0 {
                            self.set_selected_element(self.elements.len() - 1)
                        } else {
                            self.set_selected_element(self.selected_index - 1);
                        }
                    }
                    Keybind::SelectionDown => {
                        println!("recv down");
                        self.set_selected_element(self.selected_index + 1);
                    }
                    Keybind::Execute => {
                        self.execute_selected_element();
                    }
                },
                Some(FalMessage::GlobalHotkeyTriggered(keybind)) => match keybind {
                    KeyboardHookKeybind::OpenToggleFalVisibilty => {
                        self.toggle_visibilty();
                    }
                },
                Some(msg) => println!("invalid keyboard hook msg: {:?}", msg),
                _ => (),
            }
        }
    }
}
