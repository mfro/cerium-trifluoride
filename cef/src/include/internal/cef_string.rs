use cef_sys::cef_string_t;

use std::fmt::{Debug, Display, Formatter};
use std::ptr::NonNull;

#[repr(C)]
pub struct CefString {
    raw: cef_string_t,
}

pub struct CefStringUserFree {
    raw: Option<NonNull<cef_string_t>>,
}

impl CefString {
    pub fn new(value: &str) -> CefString {
        let mut raw = Default::default();
        crate::include::helpers::str_to_cef_string(&mut raw, value);
        CefString { raw }
    }
}

impl CefStringUserFree {
    #[cfg(cef_string_type_utf8)]
    pub fn new(value: &str) -> CefStringUserFree {
        let raw = unsafe { cef_sys::cef_string_userfree_utf8_alloc() };
        let raw = NonNull::new(raw);
        unsafe { crate::include::helpers::str_to_cef_string(raw.unwrap().as_mut(), value) };
        CefStringUserFree { raw }
    }

    #[cfg(cef_string_type_utf16)]
    pub fn new(value: &str) -> CefStringUserFree {
        let raw = unsafe { cef_sys::cef_string_userfree_utf16_alloc() };
        let raw = NonNull::new(raw);
        unsafe { crate::include::helpers::str_to_cef_string(raw.unwrap().as_mut(), value) };
        CefStringUserFree { raw }
    }

    pub unsafe fn from_cef(raw: *mut cef_string_t) -> CefStringUserFree {
        let raw = NonNull::new(raw);
        CefStringUserFree { raw }
    }
}

impl Default for CefString {
    fn default() -> Self {
        CefString::new("")
    }
}

impl Default for CefStringUserFree {
    fn default() -> Self {
        CefStringUserFree::new("")
    }
}

impl Drop for CefString {
    fn drop(&mut self) {
        if let Some(dtor) = self.raw.dtor {
            unsafe { dtor(self.raw.str_) };
        }
    }
}

impl Drop for CefStringUserFree {
    #[cfg(cef_string_type_utf8)]
    fn drop(&mut self) {
        unsafe {
            if let Some(ptr) = self.raw {
                cef_sys::cef_string_userfree_utf8_free(ptr.as_ptr());
            }
        }
    }

    #[cfg(cef_string_type_utf16)]
    fn drop(&mut self) {
        unsafe {
            if let Some(ptr) = self.raw {
                cef_sys::cef_string_userfree_utf16_free(ptr.as_ptr());
            }
        }
    }
}

impl Debug for CefString {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        Display::fmt(self, f)
    }
}

impl Display for CefString {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let value = crate::include::helpers::cef_string_to_string(&self.raw);
        f.write_str(&value)
    }
}

impl Debug for CefStringUserFree {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        Display::fmt(self, f)
    }
}

impl Display for CefStringUserFree {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self.raw {
            Some(raw) => {
                let value = crate::include::helpers::cef_string_to_string(unsafe { raw.as_ref() });
                f.write_str(&value)
            }
            None => Ok(()),
        }
    }
}

// can't implement generically for AsRef<&'str> without specialization
// https://github.com/rust-lang/rust/issues/31844
impl From<&str> for CefString {
    fn from(raw: &str) -> CefString {
        CefString::new(raw)
    }
}

impl From<&String> for CefString {
    fn from(raw: &String) -> CefString {
        CefString::new(raw)
    }
}

// can't implement generically for AsRef<&'str> without specialization
// https://github.com/rust-lang/rust/issues/31844
impl From<&str> for CefStringUserFree {
    fn from(raw: &str) -> CefStringUserFree {
        CefStringUserFree::new(raw)
    }
}

impl From<&String> for CefStringUserFree {
    fn from(raw: &String) -> CefStringUserFree {
        CefStringUserFree::new(raw)
    }
}
