use fltk::{
    app::{self, event_key_down, Receiver},
    enums::{Color, Event, FrameType, Key},
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

        let mut search = Input::default()
            .with_pos(0, 0)
            .with_size(WINDOW_WIDTH, LIST_ITEM_HEIGHT);
        search.set_frame(FrameType::FlatBox);
        search.set_text_size(30);

        let pack = Pack::new(
            0,
            LIST_ITEM_HEIGHT,
            WINDOW_WIDTH,
            WINDOW_HEIGHT - LIST_ITEM_HEIGHT,
            "",
        );

        let selected_index = 0;
        let mut elements: Vec<ListElement> = Vec::new();
        let programs = get_programs();
        for (index, program) in programs.iter().enumerate() {
            elements.push(ListElement::new(
                program.text.as_str(),
                WINDOW_WIDTH,
                LIST_ITEM_HEIGHT,
                FalAction::LAUNCH(program.cmd.to_string()),
            ));

            if index == selected_index {
                elements[selected_index].set_selected_state(SelectedState::Selected);
            }
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

        search.handle(move |_, ev| match ev {
            Event::KeyDown => {
                if event_key_down(Key::Down) {
                    send_channel.send(FalMessage::KeybindPressed(Keybind::SelectionDown));
                    return true;
                } else if event_key_down(Key::Up) {
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
        selected_element.set_selected_state(SelectedState::NotSelected);

        if new_selected >= self.elements.len() {
            self.selected_index = 0;
        } else {
            self.selected_index = new_selected;
        }

        let new_selected_element = self.elements.get_mut(self.selected_index).unwrap();
        new_selected_element.set_selected_state(SelectedState::Selected);
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

    fn fit_to_elements(&mut self) {
        let new_window_height =
            self.elements.len() as i32 * LIST_ITEM_HEIGHT + self.search.height();
        let new_window_width = WINDOW_WIDTH;

        let (screen_width, screen_height) = app::screen_size();

        let center_x = (screen_width / 2 as f64) - (new_window_width as f64 / 2 as f64);
        let center_y = (screen_height / 2 as f64) - (new_window_height as f64 / 2 as f64);

        self.window.set_size(new_window_width, new_window_height);
        self.window.set_pos(center_x as i32, center_y as i32);
    }

    pub fn run(&mut self) {
        self.window.end();
        self.window.show();
        self.search.take_focus().ok();
        self.search.set_visible_focus();

        self.fit_to_elements();

        while self.app.wait() {
            match self.recv_channel.recv() {
                Some(FalMessage::KeybindPressed(keybind)) => match keybind {
                    Keybind::SelectionUp => {
                        if self.selected_index == 0 {
                            self.set_selected_element(self.elements.len() - 1)
                        } else {
                            self.set_selected_element(self.selected_index - 1);
                        }
                    }
                    Keybind::SelectionDown => {
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
                None => (),
            }
        }
    }
}
