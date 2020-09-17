use cef_sys::cef_string_map_t;

pub struct CefStringMap {
    raw: cef_string_map_t,
}

impl From<cef_string_map_t> for CefStringMap {
    fn from(raw: cef_string_map_t) -> Self {
        CefStringMap { raw }
    }
}

impl From<CefStringMap> for cef_string_map_t {
    fn from(from: CefStringMap) -> Self {
        from.raw
    }
}
