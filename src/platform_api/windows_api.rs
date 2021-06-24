use std::ffi::c_void;

use windows_bindings::{
    Windows::Win32::Foundation::HWND,
    Windows::Win32::Foundation::RECT,
    Windows::Win32::Graphics::Gdi::GetMonitorInfoW,
    Windows::Win32::Graphics::Gdi::MONITORINFO,
    Windows::Win32::Graphics::Gdi::MONITOR_FROM_FLAGS,
    Windows::Win32::UI::WindowsAndMessaging::GetForegroundWindow,
    Windows::Win32::UI::{KeyboardAndMouseInput::GetActiveWindow, WindowsAndMessaging::ShowWindow},
    Windows::Win32::UI::{
        KeyboardAndMouseInput::GetFocus,
        WindowsAndMessaging::{SW_HIDE, SW_SHOW},
    },
    Windows::Win32::UI::{KeyboardAndMouseInput::SetFocus, WindowsAndMessaging::BringWindowToTop},
    Windows::Win32::{
        Graphics::Gdi::MonitorFromWindow, UI::KeyboardAndMouseInput::SetActiveWindow,
    },
};

pub fn get_screen_size(handle: *mut c_void) -> (i32, i32) {
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

pub fn focus_window(handle: *mut c_void) {
    let hwnd = HWND(handle as isize);
    unsafe { BringWindowToTop(hwnd) };
    unsafe { SetFocus(hwnd) };
}

pub fn is_window_focused(handle: *mut c_void) -> bool {
    let is_focused = HWND(handle as isize) == unsafe { GetFocus() };
    println!("is focused {}", is_focused);
    return is_focused;
}

pub fn show_window(handle: *mut c_void) {
    unsafe { ShowWindow(HWND(handle as isize), SW_SHOW) };
}

pub fn hide_window(handle: *mut c_void) {
    unsafe { ShowWindow(HWND(handle as isize), SW_HIDE) };
}
