/// Implement this interface to handle events related to focus. The methods of
/// this class will be called on the UI thread.
#[allow(non_snake_case)]
pub trait FocusHandler {
  fn on_take_focus(&mut self, browser: crate::include::CefBrowser, next: bool) -> () { Default::default() }
  fn on_set_focus(&mut self, browser: crate::include::CefBrowser, source: crate::include::internal::CefFocusSource) -> bool { Default::default() }
  fn on_got_focus(&mut self, browser: crate::include::CefBrowser) -> () { Default::default() }
}
define_refcounted!(FocusHandler, focus_handler, on_take_focus,on_set_focus,on_got_focus,);
#[allow(non_snake_case)]
unsafe extern "C" fn cef_focus_handler_t_on_take_focus(_self: *mut cef_sys::cef_focus_handler_t, browser: *mut cef_sys::cef_browser_t, next: i32) -> () {
  let ret = CefFocusHandler::from_cef(_self, true).get().on_take_focus(crate::include::CefBrowser::from_cef_own(browser).unwrap(),if next == 0 { false } else { true },);
  ret
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_focus_handler_t_on_set_focus(_self: *mut cef_sys::cef_focus_handler_t, browser: *mut cef_sys::cef_browser_t, source: cef_sys::cef_focus_source_t) -> i32 {
  let ret = CefFocusHandler::from_cef(_self, true).get().on_set_focus(crate::include::CefBrowser::from_cef_own(browser).unwrap(),source.into(),);
  if ret { 1 } else { 0 }
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_focus_handler_t_on_got_focus(_self: *mut cef_sys::cef_focus_handler_t, browser: *mut cef_sys::cef_browser_t) -> () {
  let ret = CefFocusHandler::from_cef(_self, true).get().on_got_focus(crate::include::CefBrowser::from_cef_own(browser).unwrap(),);
  ret
}
