fn main() {
    windows::build! {
        Windows::Win32::UI::KeyboardAndMouseInput::SetFocus,
        Windows::Win32::Foundation::HWND,
        Windows::Win32::Foundation::RECT,
        Windows::Win32::Graphics::Gdi::MONITORINFO,
        Windows::Win32::Graphics::Gdi::GetMonitorInfoW,
        Windows::Win32::Graphics::Gdi::MonitorFromWindow,
        Windows::Win32::Graphics::Gdi::MONITOR_FROM_FLAGS,
        Windows::Win32::UI::WindowsAndMessaging::GetForegroundWindow,
    };
}
