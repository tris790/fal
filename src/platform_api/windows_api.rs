use std::ffi::c_void;

use windows_bindings::{
    Windows::Win32::Foundation::HWND,
    Windows::Win32::Foundation::RECT,
    Windows::Win32::Graphics::Gdi::GetMonitorInfoW,
    Windows::Win32::Graphics::Gdi::MonitorFromWindow,
    Windows::Win32::Graphics::Gdi::MONITORINFO,
    Windows::Win32::Graphics::Gdi::MONITOR_FROM_FLAGS,
    Windows::Win32::UI::KeyboardAndMouseInput::GetFocus,
    Windows::Win32::UI::KeyboardAndMouseInput::SetFocus,
    Windows::Win32::UI::WindowsAndMessaging::ShowWindow,
    Windows::Win32::UI::WindowsAndMessaging::{
        GetForegroundWindow, SetForegroundWindow, SW_HIDE, SW_SHOW,
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
    unsafe { SetForegroundWindow(hwnd) };
    unsafe { SetFocus(hwnd) };
}

pub fn is_window_focused(handle: *mut c_void) -> bool {
    let is_focused = HWND(handle as isize) == unsafe { GetFocus() };
    return is_focused;
}

pub fn show_window(handle: *mut c_void) {
    unsafe { ShowWindow(HWND(handle as isize), SW_SHOW) };
}

pub fn hide_window(handle: *mut c_void) {
    unsafe { ShowWindow(HWND(handle as isize), SW_HIDE) };
}

pub struct Program {
    pub name: String,
    pub launch_cmd: String,
}

pub fn get_programs() -> Vec<Program> {
    vec![
        Program {
            name: String::from("VS Code"),
            launch_cmd: String::from("code"),
        },
        Program {
            name: String::from("Calculator"),
            launch_cmd: String::from("calc"),
        },
        Program {
            name: String::from("Edge"),
            launch_cmd: String::from("msedge"),
        },
        Program {
            name: String::from("VLC"),
            launch_cmd: String::from("vlc"),
        },
        Program {
            name: String::from("Terminal"),
            launch_cmd: String::from("wt"),
        },
        Program {
            name: String::from("Discord"),
            launch_cmd: String::from("discord"),
        },
        Program {
            name: String::from("File Explorer"),
            launch_cmd: String::from("explorer"),
        },
    ]
}
