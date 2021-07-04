use std::{borrow::BorrowMut, ffi::c_void, fs::read_dir};
const SUCCESS: LSTATUS = LSTATUS(0);

use regex::{Regex, RegexBuilder};
use walkdir::WalkDir;
use windows_bindings::{
    Windows::Win32::Foundation::HWND,
    Windows::Win32::Foundation::RECT,
    Windows::Win32::Graphics::Gdi::GetMonitorInfoW,
    Windows::Win32::Graphics::Gdi::MONITORINFO,
    Windows::Win32::Graphics::Gdi::MONITOR_FROM_FLAGS,
    Windows::Win32::{
        Foundation::LSTATUS,
        System::Registry::{RegCloseKey, RegQueryValueExW, HKEY, KEY_ALL_ACCESS, REG_VALUE_TYPE},
        UI::WindowsAndMessaging::ShowWindow,
    },
    Windows::Win32::{
        Foundation::PWSTR,
        UI::WindowsAndMessaging::{GetForegroundWindow, SetForegroundWindow, SW_HIDE, SW_SHOW},
    },
    Windows::Win32::{
        Graphics::Gdi::MonitorFromWindow,
        System::Registry::{RegOpenKeyExW, KEY_READ},
    },
    Windows::Win32::{System::Registry::RegEnumKeyExW, UI::KeyboardAndMouseInput::SetFocus},
    Windows::Win32::{System::Registry::HKEY_LOCAL_MACHINE, UI::KeyboardAndMouseInput::GetFocus},
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
    let user_defined = vec![
        Program {
            name: String::from("VS Code"),
            launch_cmd: String::from("code"),
        },
        Program {
            name: String::from("Calculator"),
            launch_cmd: String::from("calc"),
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
    ];

    println!("Fetching programs");
    let mut programs = get_programs_from_shortcuts_folder();

    user_defined
        .into_iter()
        .for_each(|program| programs.push(program));

    programs
}

fn get_programs_from_shortcuts_folder() -> Vec<Program> {
    let mut programs: Vec<Program> = vec![];
    let shortcut_folder_path = "C:\\ProgramData\\Microsoft\\Windows\\Start Menu\\Programs";

    for entry in WalkDir::new(shortcut_folder_path) {
        let path = entry.unwrap();
        let path = path.path();
        let path_str = path.to_str().unwrap().to_owned();
        let name = path
            .with_extension("")
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned();

        println!("program: {} - {}", name, path_str);
        programs.push(Program {
            launch_cmd: path_str,
            name,
        });
    }

    programs
}

fn get_programs_from_registry() -> Vec<Program> {
    let s_root = "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Uninstall";

    let mut h_uninst_key = HKEY::default();
    let mut h_app_key = HKEY::default();
    let mut reg_type: REG_VALUE_TYPE = REG_VALUE_TYPE(KEY_ALL_ACCESS.0);

    if unsafe { RegOpenKeyExW(HKEY_LOCAL_MACHINE, s_root, 0, KEY_READ, &mut h_uninst_key) }
        != LSTATUS(0)
    {
        return vec![];
    };

    let mut dw_index = 0;
    let mut l_result: LSTATUS = SUCCESS;

    let mut app_key_buf: [u16; 1024] = unsafe { std::mem::zeroed() };
    let mut s_app_key_name =
        unsafe { std::mem::transmute::<*mut u16, PWSTR>(app_key_buf.as_mut_ptr()) };
    let mut app_key_size: u32 = 1024;

    let mut sub_key_buf: [u16; 1024] = unsafe { std::mem::zeroed() };
    let mut s_sub_key = unsafe { std::mem::transmute::<*mut u16, PWSTR>(sub_key_buf.as_mut_ptr()) };
    let mut sub_key_size = 1024;

    let mut display_name_buf: [u16; 1024] = unsafe { std::mem::zeroed() };
    let mut s_display_name =
        unsafe { std::mem::transmute::<*mut u16, *mut u8>(display_name_buf.as_mut_ptr()) };
    let mut display_name_size = 1024;

    while l_result == SUCCESS {
        l_result = unsafe {
            RegEnumKeyExW(
                h_uninst_key,
                dw_index,
                s_app_key_name,
                &mut app_key_size,
                std::ptr::null_mut(),
                PWSTR::NULL,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            )
        };

        if l_result == SUCCESS {
            // print name ?
            if unsafe { RegOpenKeyExW(HKEY_LOCAL_MACHINE, s_sub_key, 0, KEY_READ, &mut h_app_key) }
                != SUCCESS
            {
                unsafe { RegCloseKey(h_app_key) };
                unsafe { RegCloseKey(h_uninst_key) };
                return vec![];
            }

            if unsafe {
                RegQueryValueExW(
                    h_app_key,
                    "DisplayName",
                    std::ptr::null_mut(),
                    &mut reg_type,
                    s_display_name,
                    &mut display_name_size,
                )
            } == SUCCESS
            {}

            unsafe { RegCloseKey(h_app_key) };
        }

        dw_index += 1;
    }
    unsafe { RegCloseKey(h_uninst_key) };
    todo!()
}
