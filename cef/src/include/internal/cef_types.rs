use std::convert::TryFrom;
use std::os::raw::c_int;

use super::{FromCef, IntoCef};

macro_rules! cef_number {
    ($ty:ty) => {
        impl IntoCef for $ty {
            type CefType = c_int;
            fn into_cef(self) -> Self::CefType {
                Self::CefType::try_from(self).unwrap_or(0)
            }
        }

        impl FromCef<c_int> for $ty {
            fn from_cef(src: c_int) -> $ty {
                Self::try_from(src).unwrap_or(0)
            }
        }
    };
}

cef_number!(u8);
cef_number!(u16);
cef_number!(u32);
cef_number!(u64);
cef_number!(i8);
cef_number!(i16);
cef_number!(i32);
cef_number!(i64);

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CefColor {
    pub raw: cef_sys::cef_color_t,
}

impl CefColor {
    pub fn new(alpha: u8, red: u8, green: u8, blue: u8) -> CefColor {
        let raw = ((alpha as cef_sys::cef_color_t) << 24)
            | ((red as cef_sys::cef_color_t) << 16)
            | ((green as cef_sys::cef_color_t) << 8)
            | ((blue as cef_sys::cef_color_t) << 0);

        CefColor { raw }
    }

    pub fn alpha(&self) -> u8 {
        ((self.raw >> 24) & 0xFF) as u8
    }

    pub fn red(&self) -> u8 {
        ((self.raw >> 16) & 0xFF) as u8
    }

    pub fn green(&self) -> u8 {
        ((self.raw >> 8) & 0xFF) as u8
    }

    pub fn blue(&self) -> u8 {
        ((self.raw >> 0) & 0xFF) as u8
    }
}

impl From<CefColor> for cef_sys::cef_color_t {
    fn from(src: CefColor) -> cef_sys::cef_color_t {
        src.raw
    }
}

impl From<cef_sys::cef_color_t> for CefColor {
    fn from(raw: cef_sys::cef_color_t) -> CefColor {
        CefColor { raw }
    }
}
