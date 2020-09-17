use cef_sys::{cef_string_t, cef_string_userfree_utf16_alloc, cef_string_userfree_utf16_free};

use std::char::decode_utf16;
use std::fmt::{Debug, Display, Formatter, Write};
use std::ptr::NonNull;

#[repr(C)]
pub struct CefString {
    raw: cef_string_t,
}

pub struct CefStringUserFree {
    raw: NonNull<cef_string_t>,
}

impl CefString {
    pub fn new(value: &str) -> CefString {
        let mut raw = Default::default();
        crate::include::helpers::str_to_cef_string(&mut raw, value);
        CefString { raw }
    }
}

impl CefStringUserFree {
    pub fn new(value: &str) -> CefStringUserFree {
        let raw = unsafe { cef_string_userfree_utf16_alloc() };
        let mut raw = NonNull::new(raw).unwrap();
        unsafe { crate::include::helpers::str_to_cef_string(raw.as_mut(), value) };
        CefStringUserFree { raw }
    }

    pub unsafe fn from_cef(raw: *const cef_string_t) -> Option<CefStringUserFree> {
        match NonNull::new(raw as *mut _) {
            Some(raw) => Some(CefStringUserFree { raw }),
            None => None,
        }
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
    fn drop(&mut self) {
        unsafe {
            cef_string_userfree_utf16_free(self.raw.as_ptr());
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
        let slice = unsafe { std::slice::from_raw_parts(self.raw.str_, self.raw.length as usize) };

        for x in decode_utf16(slice.iter().cloned()) {
            match x {
                Ok(c) => f.write_char(c)?,
                Err(_) => return Err(std::fmt::Error),
            }
        }

        Ok(())
    }
}

impl Debug for CefStringUserFree {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        Display::fmt(self, f)
    }
}

impl Display for CefStringUserFree {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let slice = unsafe {
            std::slice::from_raw_parts(self.raw.as_ref().str_, self.raw.as_ref().length as usize)
        };

        for x in decode_utf16(slice.iter().cloned()) {
            match x {
                Ok(c) => f.write_char(c)?,
                Err(_) => return Err(std::fmt::Error),
            }
        }

        Ok(())
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
