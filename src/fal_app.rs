use fltk::{
    app::{self, Receiver},
    enums::Color,
    prelude::*,
    window::Window,
};
use hotkey::{self, keys, modifiers};

use crate::components::{result_component::ResultsComponent, search_component::SearchComponent};
use crate::fal_command::FalCommandParser;
use crate::fal_message::*;
use crate::platform_api;

const MAX_RESULT_DISPLAYED: u32 = 3; //(WINDOW_HEIGHT / LIST_ITEM_HEIGHT) as u32;
const WINDOW_WIDTH: i32 = 800;
const SEARCH_BAR_HEIGHT: i32 = 50;
const LIST_ITEM_HEIGHT: i32 = 50;

pub struct FalApp {
    window: Window,
    app: app::App,
    recv_channel: Receiver<FalMessage>,
    search_component: SearchComponent,
    command_parser: FalCommandParser,
    result_component: ResultsComponent,
}

impl FalApp {
    pub fn new() -> FalApp {
        let app = app::App::default();
        let (send_channel, recv_channel) = app::channel::<FalMessage>();
        let (send_channel_thread, _) = app::channel::<FalMessage>();
        let command_parser = FalCommandParser::new();

        let mut window = Window::default();
        window.set_color(Color::from_hex(0x9CA3AF));
        window.set_border(false);

        let search_component = SearchComponent::new(WINDOW_WIDTH, SEARCH_BAR_HEIGHT, send_channel);
        let result_component = ResultsComponent::new(
            0,
            SEARCH_BAR_HEIGHT,
            WINDOW_WIDTH,
            LIST_ITEM_HEIGHT * MAX_RESULT_DISPLAYED as i32,
            MAX_RESULT_DISPLAYED,
        );

        std::thread::spawn(move || {
            let mut hotkey = hotkey::Listener::new();
            hotkey
                .register_hotkey(modifiers::CONTROL, keys::SPACEBAR, move || {
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
            search_component,
            command_parser,
            result_component,
        }
    }

    fn toggle_visibilty(&mut self) {
        if platform_api::is_window_focused(self.window.raw_handle()) {
            platform_api::hide_window(self.window.raw_handle());
        } else {
            platform_api::show_window(self.window.raw_handle());
            platform_api::focus_window(self.window.raw_handle());
        }
    }

    fn fit_to_elements(&mut self) {
        let max_window_height =
            MAX_RESULT_DISPLAYED as i32 * LIST_ITEM_HEIGHT + self.search_component.height();
        let new_window_height = self.result_component.displayed_element_count() as i32
            * LIST_ITEM_HEIGHT
            + self.search_component.height();
        let new_window_width = WINDOW_WIDTH;

        let (screen_width, screen_height) = platform_api::get_screen_size(self.window.raw_handle());
        // println!("x: {}, y: {}", screen_width, screen_height);

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
                    Keybind::SelectionUp => self.result_component.scroll_up(),
                    Keybind::SelectionDown => self.result_component.scroll_down(),
                    Keybind::Execute => {
                        self.result_component
                            .execute_selected(self.search_component.value().as_str());
                    }
                },
                Some(FalMessage::GlobalHotkeyTriggered(keybind)) => match keybind {
                    KeyboardHookKeybind::OpenToggleFalVisibilty => {
                        self.toggle_visibilty();
                    }
                },
                Some(FalMessage::TextInput(text)) => {
                    // println!("input {}", text);
                    let results = self.command_parser.parse(text.as_str());
                    self.result_component.update_results(results);
                    self.fit_to_elements();
                }
                None => (),
            }
        }
    }
}
