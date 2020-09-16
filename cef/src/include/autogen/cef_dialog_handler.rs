pub type CefFileDialogCallback = crate::include::base::CefProxy<cef_sys::cef_file_dialog_callback_t>;
#[allow(non_snake_case)]
impl CefFileDialogCallback {
  pub fn cont(&mut self, selected_accept_filter: i32, file_paths: &crate::include::internal::CefStringList) -> () {
    unsafe {
      let ret = match self.raw.as_ref().cont {
        Some(f) => f(self.raw.as_ptr(),selected_accept_filter,file_paths.into(),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn cancel(&mut self) -> () {
    unsafe {
      let ret = match self.raw.as_ref().cancel {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
}
/// Implement this interface to handle dialog events. The methods of this class
/// will be called on the browser process UI thread.
#[allow(non_snake_case)]
pub trait DialogHandler {
  fn on_file_dialog(&mut self, browser: crate::include::CefBrowser, mode: crate::include::internal::CefFileDialogMode, title: &crate::include::internal::CefString, default_file_path: &crate::include::internal::CefString, accept_filters: &crate::include::internal::CefStringList, selected_accept_filter: i32, callback: crate::include::CefFileDialogCallback) -> bool { Default::default() }
}
define_refcounted!(DialogHandler, dialog_handler, on_file_dialog,);
#[allow(non_snake_case)]
unsafe extern "C" fn cef_dialog_handler_t_on_file_dialog(_self: *mut cef_sys::cef_dialog_handler_t, browser: *mut cef_sys::cef_browser_t, mode: cef_sys::cef_file_dialog_mode_t, title: *const cef_sys::cef_string_t, default_file_path: *const cef_sys::cef_string_t, accept_filters: cef_sys::cef_string_list_t, selected_accept_filter: i32, callback: *mut cef_sys::cef_file_dialog_callback_t) -> i32 {
  let ret = CefDialogHandler::from_cef(_self, true).get().on_file_dialog(crate::include::CefBrowser::from_cef_own(browser).unwrap(),mode.into(),&crate::include::internal::CefString::from_cef(title).unwrap(),&crate::include::internal::CefString::from_cef(default_file_path).unwrap(),&accept_filters.into(),selected_accept_filter,crate::include::CefFileDialogCallback::from_cef_own(callback).unwrap(),);
  if ret { 1 } else { 0 }
}
