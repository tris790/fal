use std::ffi::c_void;

use windows_bindings::Windows::Win32::Foundation::HWND;
use windows_bindings::Windows::Win32::UI::KeyboardAndMouseInput::SetFocus;

pub fn get_screen_size() -> (i32, i32) {
    (2560, 1440)
}

pub fn focus_window(handle: *mut c_void) {
    unsafe { SetFocus(HWND(handle as isize)) };
}
