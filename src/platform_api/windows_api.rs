use std::ffi::c_void;

use windows_bindings::{
    Windows::Win32::Foundation::HWND, Windows::Win32::Foundation::RECT,
    Windows::Win32::Graphics::Gdi::GetMonitorInfoW,
    Windows::Win32::Graphics::Gdi::MonitorFromWindow, Windows::Win32::Graphics::Gdi::MONITORINFO,
    Windows::Win32::Graphics::Gdi::MONITOR_FROM_FLAGS,
    Windows::Win32::UI::KeyboardAndMouseInput::SetFocus,
    Windows::Win32::UI::WindowsAndMessaging::GetForegroundWindow,
};

pub fn get_screen_size() -> (i32, i32) {
    let MONITOR_DEFAULTTONEAREST: MONITOR_FROM_FLAGS = MONITOR_FROM_FLAGS::from(0x2);
    let hwnd = unsafe { GetForegroundWindow() };
    let monitor = unsafe { MonitorFromWindow(hwnd, MONITOR_DEFAULTTONEAREST) };

    let mut info: MONITORINFO = unsafe { std::mem::zeroed() };
    info.cbSize = std::mem::size_of::<MONITORINFO>() as u32;

    unsafe {
        GetMonitorInfoW(monitor, &mut info);
    }
    (info.rcMonitor.right, info.rcMonitor.bottom)
}

pub fn focus_window() {
    unsafe { SetFocus(HWND(0 as isize)) };
}
