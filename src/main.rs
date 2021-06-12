use fltk::{
    app,
    button::Button,
    draw::*,
    enums::{Align, Color, FrameType, Key, Shortcut},
    frame::Frame,
    group::{Group, Pack},
    input::Input,
    prelude::*,
    text::TextDisplay,
    window::Window,
};
use inputbot::{KeybdKey::*, MouseButton::*, *};

struct FalAction {}

struct ProgramResult {
    text: String,
    action: FalAction,
}

enum Keybinds {
    KB_OPEN_FAL,
    KB_CLOSE_FAL,
}

fn get_programs() -> Vec<ProgramResult> {
    vec![
        ProgramResult {
            text: String::from("Chrome"),
            action: FalAction {},
        },
        ProgramResult {
            text: String::from("Calculator"),
            action: FalAction {},
        },
        ProgramResult {
            text: String::from("Warframe"),
            action: FalAction {},
        },
    ]
}

struct ListElement {
    inner: Button,
}

impl ListElement {
    pub fn new(text: &str) -> ListElement {
        let mut button = Button::default()
            .with_pos(10, 10)
            .with_size(80, 40)
            .with_label(text);
        button.set_down_frame(FrameType::FlatBox);
        button.set_frame(FrameType::FlatBox);
        button.set_color(Color::from_rgb(51, 51, 51));
        button.set_label(&text);
        button.set_label_size(20);
        button.set_selection_color(Color::Cyan);
        button.visible_focus(false);

        ListElement { inner: button }
    }

    pub fn set_color(&mut self, color: Color) {
        self.inner.set_color(color);
        self.inner.redraw();
    }
}

struct FalApp {
    window: Window,
    app: app::App,
    selected_element: ListElement,
}

impl FalApp {
    fn new() -> FalApp {
        let app = app::App::default();

        let mut window = Window::default()
            .with_size(WINDOW_WIDTH, WINDOW_HEIGHT)
            .center_screen()
            .with_label("FAL");
        window.set_color(Color::from_hex(0x9CA3AF));
        window.set_border(false);

        let programs = get_programs();
        let mut input = Input::default()
            .with_pos(0, 0)
            .with_size(WINDOW_WIDTH, LIST_ITEM_HEIGHT);
        input.set_text_size(30);

        let pack = Pack::new(
            0,
            LIST_ITEM_HEIGHT,
            WINDOW_WIDTH,
            WINDOW_HEIGHT - LIST_ITEM_HEIGHT,
            "",
        );
        let mut selected_element = ListElement::new("");
        for (index, program) in programs.iter().enumerate() {
            selected_element = ListElement::new(program.text.as_str());
        }
        pack.end();

        FalApp {
            window,
            app,
            selected_element,
        }
    }

    fn build(&mut self) {
        self.window.end();
        self.window.show();
        self.app.run();
    }
}

const WINDOW_WIDTH: i32 = 800;
const WINDOW_HEIGHT: i32 = 500;
const LIST_ITEM_WIDTH: i32 = WINDOW_WIDTH;
const LIST_ITEM_HEIGHT: i32 = 50;
const MAX_ITEM_COUNT: i32 = (WINDOW_HEIGHT / LIST_ITEM_HEIGHT) as i32;

static mut FAL_APP_GLOBAL: Option<FalApp> = None;

fn main() {
    unsafe { FAL_APP_GLOBAL = Some(FalApp::new()) };

    CapsLockKey.bind(|| {
        println!("pressed");
        if let Some(fal_app) = unsafe { &mut FAL_APP_GLOBAL } {
            if CapsLockKey.is_toggled() {
                fal_app
                    .selected_element
                    .set_color(Color::from_rgb(37, 37, 38));
            }
        }
    });

    // Call this to start listening for bound inputs.
    handle_input_events();
    if let Some(fal_app) = unsafe { &mut FAL_APP_GLOBAL } {
        fal_app.build();
    }
}
