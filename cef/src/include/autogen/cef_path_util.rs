/// Retrieve the path associated with the specified |key|. Returns true on
/// success. Can be called on any thread in the browser process.
#[allow(non_snake_case)]
pub fn cef_get_path(key: crate::include::internal::CefPathKey, path: &mut crate::include::internal::CefString, ) -> bool {
  unsafe {
    let ret = cef_sys::cef_get_path(key.into(),path as *mut _ as *mut _,);
    if ret == 0 { false } else { true }
  }
}
