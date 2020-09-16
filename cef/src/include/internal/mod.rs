mod cef_string;
pub use cef_string::*;

mod cef_string_list;
pub use cef_string_list::*;

mod cef_string_map;
pub use cef_string_map::*;

mod cef_string_multimap;
pub use cef_string_multimap::*;

mod cef_time;
pub use cef_time::*;

#[cfg(target_os = "windows")]
mod cef_types_win;
#[cfg(target_os = "windows")]
pub use cef_types_win::*;

mod cef_types;
pub use cef_types::*;

pub use autogen::*;
mod autogen {
    mod cef_types;
    pub use cef_types::*;
}

pub trait IntoCef {
    type CefType;
    fn into_cef(self) -> Self::CefType;
}

pub trait FromCef<T> {
    fn from_cef(raw: T) -> Self;
}

pub trait RefHelper<T> {
    fn as_ref(&self) -> &T;
}

pub trait MutHelper<T>: RefHelper<T> {
    fn as_mut(&mut self) -> &mut T;
}

pub fn cef_string_to_string(src: &cef_sys::cef_string_t) -> String {
    unsafe {
        if src.length == 0 {
            String::new()
        } else {
            let slice = std::slice::from_raw_parts(src.str_, src.length as usize);
            String::from_utf16_lossy(slice)
        }
    }
}

pub fn str_to_cef_string(field: &mut cef_sys::cef_string_t, value: &str) {
    let mut v: Vec<_> = value.encode_utf16().collect();
    field.str_ = v.as_mut_ptr();
    field.length = v.len() as u64;
    field.dtor = Some(string_dtor);
    std::mem::forget(v);
}

unsafe extern "C" fn string_dtor(str_: *mut u16) {
    drop(Box::from_raw(str_))
}
