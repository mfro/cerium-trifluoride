/// Implement this interface to handle events related to browser load status. The
/// methods of this class will be called on the browser process UI thread or
/// render process main thread (TID_RENDERER).
#[allow(non_snake_case)]
pub trait LoadHandler {
  fn on_loading_state_change(&mut self, browser: crate::include::CefBrowser, isLoading: bool, canGoBack: bool, canGoForward: bool) -> () { Default::default() }
  fn on_load_start(&mut self, browser: crate::include::CefBrowser, frame: crate::include::CefFrame, transition_type: crate::include::internal::CefTransitionType) -> () { Default::default() }
  fn on_load_end(&mut self, browser: crate::include::CefBrowser, frame: crate::include::CefFrame, httpStatusCode: i32) -> () { Default::default() }
  fn on_load_error(&mut self, browser: crate::include::CefBrowser, frame: crate::include::CefFrame, errorCode: crate::include::internal::CefErrorcode, errorText: Option<&crate::include::internal::CefString>, failedUrl: &crate::include::internal::CefString) -> () { Default::default() }
}
define_refcounted!(LoadHandler, load_handler, on_loading_state_change,on_load_start,on_load_end,on_load_error,);
#[allow(non_snake_case)]
unsafe extern "C" fn cef_load_handler_t_on_loading_state_change(_self: *mut cef_sys::cef_load_handler_t, browser: *mut cef_sys::cef_browser_t, isLoading: i32, canGoBack: i32, canGoForward: i32) -> () {
  let ret = CefLoadHandler::from_cef(_self, true).get().on_loading_state_change(crate::include::CefBrowser::from_cef_own(browser).unwrap(),if isLoading == 0 { false } else { true },if canGoBack == 0 { false } else { true },if canGoForward == 0 { false } else { true },);
  ret
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_load_handler_t_on_load_start(_self: *mut cef_sys::cef_load_handler_t, browser: *mut cef_sys::cef_browser_t, frame: *mut cef_sys::cef_frame_t, transition_type: cef_sys::cef_transition_type_t) -> () {
  let ret = CefLoadHandler::from_cef(_self, true).get().on_load_start(crate::include::CefBrowser::from_cef_own(browser).unwrap(),crate::include::CefFrame::from_cef_own(frame).unwrap(),transition_type.into(),);
  ret
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_load_handler_t_on_load_end(_self: *mut cef_sys::cef_load_handler_t, browser: *mut cef_sys::cef_browser_t, frame: *mut cef_sys::cef_frame_t, httpStatusCode: i32) -> () {
  let ret = CefLoadHandler::from_cef(_self, true).get().on_load_end(crate::include::CefBrowser::from_cef_own(browser).unwrap(),crate::include::CefFrame::from_cef_own(frame).unwrap(),httpStatusCode,);
  ret
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_load_handler_t_on_load_error(_self: *mut cef_sys::cef_load_handler_t, browser: *mut cef_sys::cef_browser_t, frame: *mut cef_sys::cef_frame_t, errorCode: cef_sys::cef_errorcode_t, errorText: *const cef_sys::cef_string_t, failedUrl: *const cef_sys::cef_string_t) -> () {
  let ret = CefLoadHandler::from_cef(_self, true).get().on_load_error(crate::include::CefBrowser::from_cef_own(browser).unwrap(),crate::include::CefFrame::from_cef_own(frame).unwrap(),errorCode.into(),match &crate::include::internal::CefString::from_cef(errorText) { Some(ref x) => Some(x), None => None },&crate::include::internal::CefString::from_cef(failedUrl).unwrap(),);
  ret
}
