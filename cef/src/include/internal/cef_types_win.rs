pub type CefCursorHandle = cef_sys::HCURSOR;
pub type CefEventHandle = *mut cef_sys::MSG;
pub type CefWindowHandle = cef_sys::HWND;
pub type CefPlatformThreadId = cef_sys::DWORD;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CefMainArgs {
    pub raw: cef_sys::cef_main_args_t,
}

impl CefMainArgs {
    pub unsafe fn new(instance: *mut cef_sys::HINSTANCE__) -> CefMainArgs {
        let raw = cef_sys::cef_main_args_t { instance };
        CefMainArgs { raw }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct CefWindowInfo {
    pub raw: cef_sys::cef_window_info_t
}

impl From<CefWindowInfo> for cef_sys::cef_window_info_t {
    fn from(src: CefWindowInfo) -> cef_sys::cef_window_info_t {
        src.raw
    }
}

impl From<cef_sys::cef_window_info_t> for CefWindowInfo {
    fn from(raw: cef_sys::cef_window_info_t) -> CefWindowInfo {
        CefWindowInfo { raw }
    }
}

#[rustfmt::skip]
impl CefWindowInfo {
    pub fn ex_style(&self) -> cef_sys::DWORD { self.raw.ex_style }
    pub fn window_name(&self) -> String { super::cef_string_to_string(&self.raw.window_name) }
    pub fn style(&self) -> cef_sys::DWORD { self.raw.style }
    pub fn x(&self) -> i32 { self.raw.x }
    pub fn y(&self) -> i32 { self.raw.y }
    pub fn width(&self) -> i32 { self.raw.width }
    pub fn height(&self) -> i32 { self.raw.height }
    pub fn parent_window(&self) -> cef_sys::HWND { self.raw.parent_window }
    pub fn menu(&self) -> cef_sys::HMENU { self.raw.menu }
    pub fn windowless_rendering_enabled(&self) -> i32 { self.raw.windowless_rendering_enabled }
    pub fn shared_texture_enabled(&self) -> i32 { self.raw.shared_texture_enabled }
    pub fn external_begin_frame_enabled(&self) -> i32 { self.raw.external_begin_frame_enabled }
    pub fn window(&self) -> cef_sys::HWND { self.raw.window }

    pub fn set_ex_style(&mut self, v: cef_sys::DWORD) -> &mut Self { self.raw.ex_style = v; self }
    pub fn set_window_name(&mut self, v: &str) -> &mut Self { super::str_to_cef_string(&mut self.raw.window_name, v); self }
    pub fn set_style(&mut self, v: cef_sys::DWORD) -> &mut Self { self.raw.style = v; self }
    pub fn set_x(&mut self, v: i32) -> &mut Self { self.raw.x = v; self }
    pub fn set_y(&mut self, v: i32) -> &mut Self { self.raw.y = v; self }
    pub fn set_width(&mut self, v: i32) -> &mut Self { self.raw.width = v; self }
    pub fn set_height(&mut self, v: i32) -> &mut Self { self.raw.height = v; self }
    pub fn set_parent_window(&mut self, v: cef_sys::HWND) -> &mut Self { self.raw.parent_window = v; self }
    pub fn set_menu(&mut self, v: cef_sys::HMENU) -> &mut Self { self.raw.menu = v; self }
    pub fn set_windowless_rendering_enabled(&mut self, v: i32) -> &mut Self { self.raw.windowless_rendering_enabled = v; self }
    pub fn set_shared_texture_enabled(&mut self, v: i32) -> &mut Self { self.raw.shared_texture_enabled = v; self }
    pub fn set_external_begin_frame_enabled(&mut self, v: i32) -> &mut Self { self.raw.external_begin_frame_enabled = v; self }
    pub fn set_window(&mut self, v: cef_sys::HWND) -> &mut Self { self.raw.window = v; self }

    pub fn build(&self) -> CefWindowInfo { CefWindowInfo { raw: self.raw } }
}
