use cef_sys::cef_string_multimap_t;

pub struct CefStringMultimap {
    raw: cef_string_multimap_t,
    userfree: Option<fn(cef_string_multimap_t)>,
}

impl From<cef_string_multimap_t> for CefStringMultimap {
    fn from(raw: cef_string_multimap_t) -> Self {
        CefStringMultimap {
            raw,
            userfree: None,
        }
    }
}

impl From<CefStringMultimap> for cef_string_multimap_t {
    fn from(from: CefStringMultimap) -> Self {
        from.raw
    }
}
