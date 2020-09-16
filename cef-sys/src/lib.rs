#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub type cef_cursor_handle_t = HCURSOR;
pub type cef_event_handle_t = *mut MSG;
pub type cef_window_handle_t = HWND;
