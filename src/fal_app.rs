use fltk::{
    app::{self, Receiver},
    enums::{Color, Event, FrameType, Key},
    group::Pack,
    input::Input,
    prelude::*,
    window::Window,
};
use hotkey::{self, keys, modifiers};

use crate::components::{result_component::ResultComponent, search_component::SearchComponent};
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

pub struct FalApp {
    window: Window,
    app: app::App,
    recv_channel: Receiver<FalMessage>,
    selected_index: usize,
    search_component: SearchComponent,
    command_parser: BestCommandParser,
    result_component: ResultComponent,
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
        let result_component = ResultComponent::new(WINDOW_WIDTH, LIST_ITEM_HEIGHT);

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
            recv_channel,
            selected_index: 0,
            search_component,
            command_parser,
            result_component,
        }
    }

    fn set_results(&mut self, results: Vec<String>) {
        self.result_component.update_result(results);
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
        let max_window_height = self.result_component.max_element_count as i32 * LIST_ITEM_HEIGHT
            + self.search_component.height();
        let new_window_height =
            self.result_component.len() as i32 * LIST_ITEM_HEIGHT + self.search_component.height();
        let new_window_width = WINDOW_WIDTH;

        let (screen_width, screen_height) = platform_api::get_screen_size(self.window.raw_handle());
        println!("screen_work_area {:?}", app::screen_work_area(0));
        println!("screen_xywh {:?}", app::screen_xywh(0));
        println!("screen_coords {:?}", app::screen_coords());
        println!("screen_scale {:?}", app::screen_scale(0));

        println!("x: {}, y: {}", screen_width, screen_height);

        let center_x = (screen_width - new_window_width) / 2;
        let center_y = (screen_height - max_window_height) / 2;

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
                            self.result_component
                                .set_selected_element(self.result_component.len() - 1)
                        } else {
                            self.result_component
                                .set_selected_element(self.selected_index - 1);
                        }
                    }
                    Keybind::SelectionDown => {
                        self.result_component
                            .set_selected_element(self.selected_index + 1);
                    }
                    Keybind::Execute => {
                        // self.execute_selected_element();
                    }
                },
                Some(FalMessage::GlobalHotkeyTriggered(keybind)) => match keybind {
                    KeyboardHookKeybind::OpenToggleFalVisibilty => {
                        self.toggle_visibilty();
                    }
                },
                Some(FalMessage::TextInput(text)) => {
                    println!("input {}", text);
                    let result = self.command_parser.on_textbox_changed(text);
                    self.set_results(result);
                    self.fit_to_elements();
                }
                None => (),
            }
        }
    }
}
