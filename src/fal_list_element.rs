use fltk::{
    button::Button,
    enums::{Color, FrameType},
    prelude::*,
};

use crate::fal_action::FalAction;

fn bg_color() -> Color {
    Color::from_rgb(31, 31, 31)
}

fn selected_color() -> Color {
    Color::from_rgb(255, 0, 0)
}

pub enum SelectedState {
    NotSelected,
    Selected,
}

pub struct ListElement {
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
        button.set_color(bg_color());
        button.set_label(&text);
        button.set_label_size(20);
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

    pub fn execute(&self) {
        self.action.execute();
    }

    pub fn set_selected(&mut self, selected: SelectedState) {
        match selected {
            SelectedState::NotSelected => self.set_color(bg_color()),
            SelectedState::Selected => self.set_color(selected_color()),
        }
    }
}
