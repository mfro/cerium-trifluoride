use cef_sys::cef_string_list_t;

pub struct CefStringList {
    raw: cef_string_list_t,
}

impl From<cef_string_list_t> for CefStringList {
    fn from(raw: cef_string_list_t) -> Self {
        CefStringList { raw }
    }
}

impl From<&CefStringList> for cef_string_list_t {
    fn from(from: &CefStringList) -> Self {
        from.raw
    }
}

impl From<CefStringList> for cef_string_list_t {
    fn from(from: CefStringList) -> Self {
        from.raw
    }
}
