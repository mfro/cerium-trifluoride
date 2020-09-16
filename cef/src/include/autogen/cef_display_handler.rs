/// Implement this interface to handle events related to browser display state.
/// The methods of this class will be called on the UI thread.
#[allow(non_snake_case)]
pub trait DisplayHandler {
  fn on_address_change(&mut self, browser: crate::include::CefBrowser, frame: crate::include::CefFrame, url: &crate::include::internal::CefString) -> () { Default::default() }
  fn on_title_change(&mut self, browser: crate::include::CefBrowser, title: Option<&crate::include::internal::CefString>) -> () { Default::default() }
  fn on_favicon_urlchange(&mut self, browser: crate::include::CefBrowser, icon_urls: &crate::include::internal::CefStringList) -> () { Default::default() }
  fn on_fullscreen_mode_change(&mut self, browser: crate::include::CefBrowser, fullscreen: bool) -> () { Default::default() }
  fn on_tooltip(&mut self, browser: crate::include::CefBrowser, text: &mut crate::include::internal::CefString) -> bool { Default::default() }
  fn on_status_message(&mut self, browser: crate::include::CefBrowser, value: Option<&crate::include::internal::CefString>) -> () { Default::default() }
  fn on_console_message(&mut self, browser: crate::include::CefBrowser, level: crate::include::internal::CefLogSeverity, message: &crate::include::internal::CefString, source: &crate::include::internal::CefString, line: i32) -> bool { Default::default() }
  fn on_auto_resize(&mut self, browser: crate::include::CefBrowser, new_size: &crate::include::internal::CefSize) -> bool { Default::default() }
  fn on_loading_progress_change(&mut self, browser: crate::include::CefBrowser, progress: f64) -> () { Default::default() }
}
define_refcounted!(DisplayHandler, display_handler, on_address_change,on_title_change,on_favicon_urlchange,on_fullscreen_mode_change,on_tooltip,on_status_message,on_console_message,on_auto_resize,on_loading_progress_change,);
#[allow(non_snake_case)]
unsafe extern "C" fn cef_display_handler_t_on_address_change(_self: *mut cef_sys::cef_display_handler_t, browser: *mut cef_sys::cef_browser_t, frame: *mut cef_sys::cef_frame_t, url: *const cef_sys::cef_string_t) -> () {
  let ret = CefDisplayHandler::from_cef(_self, true).get().on_address_change(crate::include::CefBrowser::from_cef_own(browser).unwrap(),crate::include::CefFrame::from_cef_own(frame).unwrap(),&crate::include::internal::CefString::from_cef(url).unwrap(),);
  ret
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_display_handler_t_on_title_change(_self: *mut cef_sys::cef_display_handler_t, browser: *mut cef_sys::cef_browser_t, title: *const cef_sys::cef_string_t) -> () {
  let ret = CefDisplayHandler::from_cef(_self, true).get().on_title_change(crate::include::CefBrowser::from_cef_own(browser).unwrap(),match &crate::include::internal::CefString::from_cef(title) { Some(ref x) => Some(x), None => None },);
  ret
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_display_handler_t_on_favicon_urlchange(_self: *mut cef_sys::cef_display_handler_t, browser: *mut cef_sys::cef_browser_t, icon_urls: cef_sys::cef_string_list_t) -> () {
  let ret = CefDisplayHandler::from_cef(_self, true).get().on_favicon_urlchange(crate::include::CefBrowser::from_cef_own(browser).unwrap(),&icon_urls.into(),);
  ret
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_display_handler_t_on_fullscreen_mode_change(_self: *mut cef_sys::cef_display_handler_t, browser: *mut cef_sys::cef_browser_t, fullscreen: i32) -> () {
  let ret = CefDisplayHandler::from_cef(_self, true).get().on_fullscreen_mode_change(crate::include::CefBrowser::from_cef_own(browser).unwrap(),if fullscreen == 0 { false } else { true },);
  ret
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_display_handler_t_on_tooltip(_self: *mut cef_sys::cef_display_handler_t, browser: *mut cef_sys::cef_browser_t, text: *mut cef_sys::cef_string_t) -> i32 {
  let ret = CefDisplayHandler::from_cef(_self, true).get().on_tooltip(crate::include::CefBrowser::from_cef_own(browser).unwrap(),&mut crate::include::internal::CefString::from_cef(text).unwrap(),);
  if ret { 1 } else { 0 }
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_display_handler_t_on_status_message(_self: *mut cef_sys::cef_display_handler_t, browser: *mut cef_sys::cef_browser_t, value: *const cef_sys::cef_string_t) -> () {
  let ret = CefDisplayHandler::from_cef(_self, true).get().on_status_message(crate::include::CefBrowser::from_cef_own(browser).unwrap(),match &crate::include::internal::CefString::from_cef(value) { Some(ref x) => Some(x), None => None },);
  ret
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_display_handler_t_on_console_message(_self: *mut cef_sys::cef_display_handler_t, browser: *mut cef_sys::cef_browser_t, level: cef_sys::cef_log_severity_t, message: *const cef_sys::cef_string_t, source: *const cef_sys::cef_string_t, line: i32) -> i32 {
  let ret = CefDisplayHandler::from_cef(_self, true).get().on_console_message(crate::include::CefBrowser::from_cef_own(browser).unwrap(),level.into(),&crate::include::internal::CefString::from_cef(message).unwrap(),&crate::include::internal::CefString::from_cef(source).unwrap(),line,);
  if ret { 1 } else { 0 }
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_display_handler_t_on_auto_resize(_self: *mut cef_sys::cef_display_handler_t, browser: *mut cef_sys::cef_browser_t, new_size: *const cef_sys::cef_size_t) -> i32 {
  let ret = CefDisplayHandler::from_cef(_self, true).get().on_auto_resize(crate::include::CefBrowser::from_cef_own(browser).unwrap(),&*(new_size as *const _),);
  if ret { 1 } else { 0 }
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_display_handler_t_on_loading_progress_change(_self: *mut cef_sys::cef_display_handler_t, browser: *mut cef_sys::cef_browser_t, progress: f64) -> () {
  let ret = CefDisplayHandler::from_cef(_self, true).get().on_loading_progress_change(crate::include::CefBrowser::from_cef_own(browser).unwrap(),progress,);
  ret
}
