/// Implement this interface to handle events related to browser display state.
/// The methods of this class will be called on the UI thread.
#[allow(non_snake_case)]
#[allow(unused_variables)]
pub trait DisplayHandler {
  /// Called when a frame's address has changed.
  fn on_address_change(&mut self, browser: crate::include::CefBrowser, frame: crate::include::CefFrame, url: &crate::include::internal::CefString) -> () { Default::default() }
  /// Called when the page title changes.
  fn on_title_change(&mut self, browser: crate::include::CefBrowser, title: Option<&crate::include::internal::CefString>) -> () { Default::default() }
  /// Called when the page icon changes.
  fn on_favicon_urlchange(&mut self, browser: crate::include::CefBrowser, icon_urls: &crate::include::internal::CefStringList) -> () { Default::default() }
  /// Called when web content in the page has toggled fullscreen mode. If
  /// |fullscreen| is true the content will automatically be sized to fill the
  /// browser content area. If |fullscreen| is false the content will
  /// automatically return to its original size and position. The client is
  /// responsible for resizing the browser if desired.
  fn on_fullscreen_mode_change(&mut self, browser: crate::include::CefBrowser, fullscreen: bool) -> () { Default::default() }
  /// Called when the browser is about to display a tooltip. |text| contains the
  /// text that will be displayed in the tooltip. To handle the display of the
  /// tooltip yourself return true. Otherwise, you can optionally modify |text|
  /// and then return false to allow the browser to display the tooltip.
  /// When window rendering is disabled the application is responsible for
  /// drawing tooltips and the return value is ignored.
  fn on_tooltip(&mut self, browser: crate::include::CefBrowser, text: &mut crate::include::internal::CefString) -> bool { Default::default() }
  /// Called when the browser receives a status message. |value| contains the
  /// text that will be displayed in the status message.
  fn on_status_message(&mut self, browser: crate::include::CefBrowser, value: Option<&crate::include::internal::CefString>) -> () { Default::default() }
  /// Called to display a console message. Return true to stop the message from
  /// being output to the console.
  fn on_console_message(&mut self, browser: crate::include::CefBrowser, level: crate::include::internal::CefLogSeverity, message: Option<&crate::include::internal::CefString>, source: Option<&crate::include::internal::CefString>, line: i32) -> bool { Default::default() }
  /// Called when auto-resize is enabled via CefBrowserHost::SetAutoResizeEnabled
  /// and the contents have auto-resized. |new_size| will be the desired size in
  /// view coordinates. Return true if the resize was handled or false for
  /// default handling.
  fn on_auto_resize(&mut self, browser: crate::include::CefBrowser, new_size: &crate::include::internal::CefSize) -> bool { Default::default() }
  /// Called when the overall page loading progress has changed. |progress|
  /// ranges from 0.0 to 1.0.
  fn on_loading_progress_change(&mut self, browser: crate::include::CefBrowser, progress: f64) -> () { Default::default() }
}
define_refcounted!(DisplayHandler, CefDisplayHandler, cef_display_handler_t, on_address_change: cef_display_handler_t_on_address_change,on_title_change: cef_display_handler_t_on_title_change,on_favicon_urlchange: cef_display_handler_t_on_favicon_urlchange,on_fullscreen_mode_change: cef_display_handler_t_on_fullscreen_mode_change,on_tooltip: cef_display_handler_t_on_tooltip,on_status_message: cef_display_handler_t_on_status_message,on_console_message: cef_display_handler_t_on_console_message,on_auto_resize: cef_display_handler_t_on_auto_resize,on_loading_progress_change: cef_display_handler_t_on_loading_progress_change,);
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
  let ret = CefDisplayHandler::from_cef(_self, true).get().on_console_message(crate::include::CefBrowser::from_cef_own(browser).unwrap(),level.into(),match &crate::include::internal::CefString::from_cef(message) { Some(ref x) => Some(x), None => None },match &crate::include::internal::CefString::from_cef(source) { Some(ref x) => Some(x), None => None },line,);
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
