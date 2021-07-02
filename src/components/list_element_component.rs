use fltk::{
    button::Button,
    enums::{Color, FrameType},
    prelude::*,
};

fn bg_color() -> Color {
    Color::from_rgb(51, 51, 51)
}

fn selected_color() -> Color {
    Color::from_rgb(30, 30, 30)
}

pub enum SelectedState {
    NotSelected,
    Selected,
}

pub struct ListElement {
    inner: Button,
    pub action: &'static dyn Fn(&str),
}

impl ListElement {
    pub fn new(text: &str, width: i32, height: i32, action: &'static dyn Fn(&str)) -> ListElement {
        let mut button = Button::default().with_size(width, height).with_label(text);
        button.set_label_color(Color::from_rgb(200, 200, 170));
        button.set_down_frame(FrameType::FlatBox);
        button.set_frame(FrameType::FlatBox);
        button.set_color(bg_color());
        button.set_label(&text);
        button.set_label_size(20);

        ListElement {
            inner: button,
            action,
        }
    }

    pub fn set_color(&mut self, color: Color) {
        self.inner.set_color(color);
        self.inner.redraw();
    }

    pub fn execute(&self, data: &str) {
        (self.action)(data);
    }

    pub fn set_selected_state(&mut self, selected: SelectedState) {
        match selected {
            SelectedState::NotSelected => self.set_color(bg_color()),
            SelectedState::Selected => self.set_color(selected_color()),
        }
    }

    pub fn update(&mut self, text: &str, action: &'static dyn Fn(&str), selected: SelectedState) {
        self.inner.set_label(text);
        self.inner.set_damage(true);
        self.inner.redraw();

        self.action = action;

        self.set_selected_state(selected);
    }
}
