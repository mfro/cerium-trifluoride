use cef_sys::{cef_string_t, cef_string_userfree_utf16_alloc, cef_string_userfree_utf16_free};

use std::char::decode_utf16;
use std::fmt::{Debug, Display, Formatter, Write};
use std::ptr::NonNull;

use super::IntoCef;

fn userfree_helper(src: *mut cef_string_t) {
    unsafe { cef_string_userfree_utf16_free(src) };
}

pub struct CefString {
    raw: NonNull<cef_string_t>,
    userfree: Option<fn(*mut cef_string_t)>,
}

impl CefString {
    pub fn new(value: &str) -> CefString {
        let raw = unsafe {
            let raw = cef_string_userfree_utf16_alloc();
            super::str_to_cef_string(&mut *raw, value);
            raw
        };

        CefString {
            raw: NonNull::new(raw).unwrap(),
            userfree: Some(userfree_helper),
        }
    }

    pub unsafe fn userfree(raw: *const cef_string_t) -> CefString {
        match NonNull::new(raw as *mut _) {
            Some(raw) => CefString {
                raw,
                userfree: Some(userfree_helper),
            },
            None => panic!(),
        }
    }

    pub unsafe fn from_cef(raw: *const cef_string_t) -> Option<CefString> {
        match NonNull::new(raw as *mut _) {
            Some(raw) => Some(CefString {
                raw,
                userfree: None,
            }),
            None => None,
        }
    }
}

impl Drop for CefString {
    fn drop(&mut self) {
        if let Some(f) = self.userfree {
            f(self.raw.as_ptr());
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

impl CefString {
    pub fn assign(&mut self, value: &str) {
        unsafe {
            if let Some(f) = self.raw.as_ref().dtor {
                f(self.raw.as_ref().str_)
            }

            let v: Vec<_> = value.encode_utf16().collect();
            let length = v.len();
            let str_ = Box::into_raw(v.into_boxed_slice());

            self.raw.as_mut().str_ = str_ as *mut _;
            self.raw.as_mut().length = length as u64;
            todo!("add string dtor")
        }
    }
}

impl From<*const cef_string_t> for CefString {
    fn from(raw: *const cef_string_t) -> CefString {
        CefString {
            raw: NonNull::new(raw as *mut _).unwrap(),
            userfree: None,
        }
    }
}

impl From<*mut cef_string_t> for CefString {
    fn from(raw: *mut cef_string_t) -> CefString {
        CefString {
            raw: NonNull::new(raw).unwrap(),
            userfree: None,
        }
    }
}

impl IntoCef for &CefString {
    type CefType = *const cef_string_t;

    fn into_cef(self) -> Self::CefType {
        self.raw.as_ptr()
    }
}

impl IntoCef for &mut CefString {
    type CefType = *mut cef_string_t;

    fn into_cef(self) -> Self::CefType {
        self.raw.as_ptr()
    }
}

impl IntoCef for Option<&CefString> {
    type CefType = *const cef_string_t;

    fn into_cef(self) -> Self::CefType {
        match self {
            Some(from) => from.into_cef(),
            None => std::ptr::null(),
        }
    }
}

impl IntoCef for Option<&mut CefString> {
    type CefType = *mut cef_string_t;

    fn into_cef(self) -> Self::CefType {
        match self {
            Some(from) => from.into_cef(),
            None => std::ptr::null_mut(),
        }
    }
}
