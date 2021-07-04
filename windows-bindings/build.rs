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
        Windows::Win32::UI::KeyboardAndMouseInput::GetFocus,
        Windows::Win32::UI::KeyboardAndMouseInput::GetActiveWindow,
        Windows::Win32::UI::KeyboardAndMouseInput::SetActiveWindow,
        Windows::Win32::UI::WindowsAndMessaging::ShowWindow,
        Windows::Win32::UI::WindowsAndMessaging::SHOW_WINDOW_CMD,
        Windows::Win32::UI::WindowsAndMessaging::BringWindowToTop,
        Windows::Win32::UI::WindowsAndMessaging::SetForegroundWindow,
        Windows::Win32::System::Registry::HKEY,
        Windows::Win32::System::Registry::RegOpenKeyExW,
        Windows::Win32::System::Registry::HKEY_LOCAL_MACHINE,
        Windows::Win32::System::Registry::RegEnumKeyExW,
        Windows::Win32::System::Registry::RegCloseKey,
        Windows::Win32::System::Registry::RegQueryValueExW,
        Windows::Win32::Foundation::LSTATUS,
        Windows::Win32::System::Registry::REG_VALUE_TYPE
    };
}
