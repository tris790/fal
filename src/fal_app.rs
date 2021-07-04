use std::ffi::c_void;

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

const SEARCH_BAR_HEIGHT: i32 = 50;
const LIST_ITEM_HEIGHT: i32 = 50;

pub struct FalApp {
    max_result_displayed: u32,
    window_width: i32,
    window: Window,
    app: app::App,
    recv_channel: Receiver<FalMessage>,
    search_component: SearchComponent,
    command_parser: FalCommandParser,
    result_component: ResultsComponent,
    handle: *mut c_void,
}

impl FalApp {
    pub fn new() -> FalApp {
        let max_result_displayed = 3;
        let window_width = 800;

        let app = app::App::default();
        let (send_channel, recv_channel) = app::channel::<FalMessage>();
        let (send_channel_thread, _) = app::channel::<FalMessage>();
        let command_parser = FalCommandParser::new();

        let mut window = Window::default();
        window.set_color(Color::from_hex(0x9CA3AF));
        window.set_border(false);

        let search_component = SearchComponent::new(window_width, SEARCH_BAR_HEIGHT, send_channel);
        let result_component = ResultsComponent::new(
            0,
            SEARCH_BAR_HEIGHT,
            window_width,
            LIST_ITEM_HEIGHT * max_result_displayed as i32,
            max_result_displayed,
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

        let handle = std::ptr::null_mut();

        FalApp {
            max_result_displayed,
            window_width,
            window,
            app,
            recv_channel,
            search_component,
            command_parser,
            result_component,
            handle,
        }
    }

    fn toggle_visibilty(&mut self) {
        if platform_api::is_window_focused(self.handle) {
            platform_api::hide_window(self.handle);
        } else {
            platform_api::show_window(self.handle);
            platform_api::focus_window(self.handle);
        }
    }

    fn fit_to_elements(&mut self) {
        let max_window_height =
            self.max_result_displayed as i32 * LIST_ITEM_HEIGHT + self.search_component.height();
        let new_window_height = self.result_component.displayed_element_count() as i32
            * LIST_ITEM_HEIGHT
            + self.search_component.height();
        let new_window_width = self.window_width;

        let (screen_width, screen_height) = platform_api::get_screen_size(self.handle);

        let center_x = (screen_width - new_window_width) / 2;
        let center_y = (screen_height - max_window_height) / 2;

        self.window.set_size(new_window_width, new_window_height);
        self.window.set_pos(center_x as i32, center_y as i32);
        platform_api::focus_window(self.window.raw_handle());
    }

    pub fn run(&mut self) {
        self.window.end();
        self.window.show();
        self.handle = self.window.raw_handle().to_owned();

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
                    let results = self.command_parser.parse(text.as_str());
                    self.result_component.update_results(results);
                    self.fit_to_elements();
                }
                None => (),
            }
        }
    }
}
