use std::cmp::min;

use fltk::{group::Pack, prelude::*};

use crate::{components::list_element_component::SelectedState, fal_action::no_action};

use super::list_element_component::ListElement;

pub struct ResultComponent {
    all_results: Vec<String>,
    max_element_displayed: u32,
    displayed_elements: Vec<ListElement>,
    display_start_index: usize,
    selected_index: usize,
}

impl ResultComponent {
    pub fn new(
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        max_element_displayed: u32,
    ) -> ResultComponent {
        let display_start_index = 0;
        let all_results = vec![];
        let selected_index = 0;

        let pack = Pack::new(x, y, width, height, "");
        let element_height = height / max_element_displayed as i32;
        let mut displayed_elements: Vec<ListElement> = (0..max_element_displayed)
            .into_iter()
            .map(|_| ListElement::new("", width, element_height, &no_action))
            .collect();

        pack.end();
        displayed_elements[0].set_selected_state(SelectedState::Selected);

        ResultComponent {
            display_start_index,
            all_results,
            max_element_displayed,
            displayed_elements,
            selected_index,
        }
    }

    pub fn scroll_up(&mut self) {
        if self.all_results.len() == 0 {
            return;
        }

        if self.selected_index > 0 {
            self.selected_index -= 1;
            if self.display_start_index > 0 && self.selected_index < self.display_start_index {
                self.display_start_index -= 1;
            }
        } else {
            self.selected_index = self.all_results.len() - 1;
            self.display_start_index = self.all_results.len() - self.max_element_displayed as usize;
        }

        self.update_displayed();
    }

    pub fn scroll_down(&mut self) {
        if self.all_results.len() == 0 {
            return;
        }

        if self.selected_index < self.all_results.len() - 1 {
            self.selected_index += 1;
            let display_max_index =
                self.display_start_index + (self.max_element_displayed as usize);

            if (display_max_index < self.all_results.len())
                && self.selected_index >= display_max_index
            {
                self.display_start_index += 1;
            }
        } else {
            self.selected_index = 0;
            self.display_start_index = 0;
        }

        self.update_displayed();
    }

    fn update_displayed(&mut self) {
        let new_element_count = self.displayed_element_count();
        println!("updating displayed");

        // Update the display for all elements
        for index in 0..new_element_count {
            println!("updating [{}]", index);
            let result_index = index + self.display_start_index;
            let result = self
                .all_results
                .get(result_index)
                .expect(format!("[Result_Component] Cannot get result[{}]", index).as_str());

            let display_element = self.displayed_elements.get_mut(index).expect(
                format!("[Result_Component] Cannot get displayed element[{}]", index).as_str(),
            );

            display_element.update(&result.as_str(), &no_action, SelectedState::NotSelected);
        }

        // Select the current element
        let displayed_selected_index = self.selected_index - self.display_start_index;
        self.displayed_elements
            .get_mut(displayed_selected_index)
            .expect(
                format!(
                    "[Result Component] Cannot get displayed selected element[{}] from [{} {}]",
                    displayed_selected_index, self.selected_index, self.display_start_index
                )
                .as_str(),
            )
            .set_selected_state(SelectedState::Selected);
    }
    pub fn update_results(&mut self, new_results: Vec<String>) {
        self.all_results = new_results;
        println!("Updating results {} elements", self.all_results.len());
        self.update_displayed();
    }

    pub fn displayed_element_count(&self) -> usize {
        min(self.all_results.len(), self.max_element_displayed as usize)
    }
}
