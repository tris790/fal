fn main() {
    windows::build! {
        Windows::Win32::UI::KeyboardAndMouseInput::SetFocus,
        Windows::Win32::Foundation::HWND
    };
}
