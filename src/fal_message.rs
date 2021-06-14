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
pub enum FalMessageMainThread {
    KeybindPressed(Keybind),
}
#[derive(Debug)]
pub enum FalMessageKeyboardHookThread {
    GlobalHotkeyTriggered(KeyboardHookKeybind),
}
