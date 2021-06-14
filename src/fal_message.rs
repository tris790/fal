use fltk::enums::Key;

#[derive(Debug)]
pub enum Keybind {
    SelectionUp,
    SelectionDown,
    Execute,
}

#[derive(Debug)]
pub enum KeyboardHookKeybind {
    OpenToggleFalVisibilty,
}

#[derive(Debug)]
pub enum FalMessage {
    KeybindPressed(Keybind),
    GlobalHotkeyTriggered(KeyboardHookKeybind),
}
