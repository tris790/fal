use fltk::{
    app::{self, event_key_down, Sender},
    enums::{Event, FrameType, Key},
    input::Input,
    prelude::*,
};

use crate::fal_message::*;

pub struct SearchComponent {
    search_input: Input,
}

impl SearchComponent {
    pub fn new(width: i32, height: i32, send_channel: Sender<FalMessage>) -> SearchComponent {
        let mut search_input = Input::default().with_size(width, height);

        search_input.set_frame(FrameType::FlatBox);
        search_input.set_text_size(30);

        search_input.handle(move |_, ev| match ev {
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

        SearchComponent { search_input }
    }

    pub fn focus(&mut self) {
        app::set_focus(&self.search_input);
    }

    pub fn height(&self) -> i32 {
        self.search_input.height()
    }
}
