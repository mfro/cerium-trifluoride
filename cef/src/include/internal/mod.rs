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

    pub type CefURLParts = CefUrlparts;
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
