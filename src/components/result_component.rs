use fltk::{
    app::{self, Receiver},
    enums::{Color, Event, FrameType, Key},
    group::Pack,
    input::Input,
    prelude::*,
    window::Window,
};

use crate::components::list_element_component::*;
use crate::fal_action::FalAction;
pub struct ResultComponent {
    pub elements: Vec<ListElement>,
    pub selected_index: usize,
    pub width: i32,
    pub height: i32,
    pub pack: Pack,
    pub max_element_count: usize,
    pub element_count: usize,
}

impl ResultComponent {
    pub fn new(width: i32, height: i32) -> ResultComponent {
        let selected_index = 0;
        let max_element_count = 3;
        let pack = Pack::new(0, height, width, height, "");

        let mut elements: Vec<ListElement> = Vec::new();
        let programs: Vec<String> = vec![
            String::from("vs code"),
            String::from("calc"),
            String::from("chrome"),
            String::from("warframe"),
        ]
        .into_iter()
        .take(max_element_count)
        .collect();

        for program in programs {
            elements.push(ListElement::new(
                program.as_str(),
                width,
                height,
                FalAction::NONE,
            ));
        }
        elements[0].set_selected_state(SelectedState::Selected);

        pack.end();
        let count = elements.len();
        ResultComponent {
            elements,
            selected_index,
            width,
            height,
            pack,
            max_element_count,
            element_count: count,
        }
    }

    pub fn update_result(&mut self, new_elements: Vec<String>) {
        if new_elements.len() >= self.max_element_count {
            self.element_count = self.max_element_count;
            for (index, element) in self.elements.iter_mut().enumerate() {
                element.update_text_no_redraw(new_elements[index].as_str());
            }
        } else {
            self.element_count = new_elements.len();
            for (index, element) in self.elements.iter_mut().enumerate() {
                if index < new_elements.len() {
                    element.update_text_no_redraw(new_elements[index].as_str());
                } else {
                    element.update_text_no_redraw("");
                }
            }
        }
    }

    pub fn set_selected_element(&mut self, new_selected: usize) {
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

    pub fn execute_selected_element(&mut self) {
        self.elements.get(self.selected_index).unwrap().execute();
    }

    pub fn len(&self) -> usize {
        self.element_count
    }
}
