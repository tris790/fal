use fltk::{
    app::{self, Receiver},
    enums::{Color, Event, FrameType, Key},
    group::Pack,
    input::Input,
    prelude::*,
    window::Window,
};
use hotkey::{self, keys, modifiers};

use crate::components::list_element_component::*;
use crate::components::search_component::SearchComponent;
use crate::fal_action::FalAction;
use crate::fal_command2::BestCommandParser;
// use crate::fal_command::{FalCommand, FalCommandParser};
use crate::fal_message::*;
use crate::platform_api;
use crate::program_lister::get_all_programs;

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
    recv_channel: Receiver<FalMessage>,
    selected_index: usize,
    search_component: SearchComponent,
    command_parser: BestCommandParser,
}

impl FalApp {
    pub fn new() -> FalApp {
        let app = app::App::default();
        let (send_channel, recv_channel) = app::channel::<FalMessage>();
        let (send_channel_thread, _) = app::channel::<FalMessage>();
        let command_parser = BestCommandParser::new();

        let mut window = Window::default().with_size(WINDOW_WIDTH, WINDOW_HEIGHT);
        window.set_color(Color::from_hex(0x9CA3AF));
        window.set_border(false);

        let search_component = SearchComponent::new(WINDOW_WIDTH, LIST_ITEM_HEIGHT, send_channel);

        let pack = Pack::new(
            0,
            LIST_ITEM_HEIGHT,
            WINDOW_WIDTH,
            WINDOW_HEIGHT - LIST_ITEM_HEIGHT,
            "",
        );

        let selected_index = 0;
        let mut elements: Vec<ListElement> = Vec::new();
        let programs = get_all_programs();
        for (index, program) in programs.iter().enumerate() {
            elements.push(ListElement::new(
                program.name.as_str(),
                WINDOW_WIDTH,
                LIST_ITEM_HEIGHT,
                FalAction::LAUNCH(program.path.to_string()),
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

        FalApp {
            window,
            app,
            elements,
            recv_channel,
            selected_index: 0,
            search_component,
            command_parser,
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
            self.elements.len() as i32 * LIST_ITEM_HEIGHT + self.search_component.height();
        let new_window_width = WINDOW_WIDTH;

        let (screen_width, screen_height) = platform_api::get_screen_size(self.window.raw_handle());
        println!("screen_work_area {:?}", app::screen_work_area(0));
        println!("screen_xywh {:?}", app::screen_xywh(0));
        println!("screen_coords {:?}", app::screen_coords());
        println!("screen_scale {:?}", app::screen_scale(0));

        println!("x: {}, y: {}", screen_width, screen_height);

        let center_x = (screen_width - new_window_width) / 2;
        let center_y = (screen_height - new_window_height) / 2;

        self.window.set_size(new_window_width, new_window_height);
        self.window.set_pos(center_x as i32, center_y as i32);
        platform_api::focus_window(self.window.raw_handle());
    }

    pub fn run(&mut self) {
        self.window.end();
        self.window.show();

        self.search_component.focus();
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
                Some(FalMessage::TextInput(text)) => {
                    let a = self.command_parser.on_textbox_changed(text);
                }
                None => (),
            }
        }
    }
}
