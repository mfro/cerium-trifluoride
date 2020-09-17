/// Implement this interface to handle events related to browser load status. The
/// methods of this class will be called on the browser process UI thread or
/// render process main thread (TID_RENDERER).
#[allow(non_snake_case)]
#[allow(unused_variables)]
pub trait LoadHandler {
  /// Called when the loading state has changed. This callback will be executed
  /// twice -- once when loading is initiated either programmatically or by user
  /// action, and once when loading is terminated due to completion, cancellation
  /// of failure. It will be called before any calls to OnLoadStart and after all
  /// calls to OnLoadError and/or OnLoadEnd.
  fn on_loading_state_change(&mut self, browser: crate::include::CefBrowser, isLoading: bool, canGoBack: bool, canGoForward: bool) -> () { Default::default() }
  /// Called after a navigation has been committed and before the browser begins
  /// loading contents in the frame. The |frame| value will never be empty --
  /// call the IsMain() method to check if this frame is the main frame.
  /// |transition_type| provides information about the source of the navigation
  /// and an accurate value is only available in the browser process. Multiple
  /// frames may be loading at the same time. Sub-frames may start or continue
  /// loading after the main frame load has ended. This method will not be called
  /// for same page navigations (fragments, history state, etc.) or for
  /// navigations that fail or are canceled before commit. For notification of
  /// overall browser load status use OnLoadingStateChange instead.
  fn on_load_start(&mut self, browser: crate::include::CefBrowser, frame: crate::include::CefFrame, transition_type: crate::include::internal::CefTransitionType) -> () { Default::default() }
  /// Called when the browser is done loading a frame. The |frame| value will
  /// never be empty -- call the IsMain() method to check if this frame is the
  /// main frame. Multiple frames may be loading at the same time. Sub-frames may
  /// start or continue loading after the main frame load has ended. This method
  /// will not be called for same page navigations (fragments, history state,
  /// etc.) or for navigations that fail or are canceled before commit. For
  /// notification of overall browser load status use OnLoadingStateChange
  /// instead.
  fn on_load_end(&mut self, browser: crate::include::CefBrowser, frame: crate::include::CefFrame, httpStatusCode: i32) -> () { Default::default() }
  /// Called when a navigation fails or is canceled. This method may be called
  /// by itself if before commit or in combination with OnLoadStart/OnLoadEnd if
  /// after commit. |errorCode| is the error code number, |errorText| is the
  /// error text and |failedUrl| is the URL that failed to load.
  /// See net\base\net_error_list.h for complete descriptions of the error codes.
  fn on_load_error(&mut self, browser: crate::include::CefBrowser, frame: crate::include::CefFrame, errorCode: crate::include::internal::CefErrorcode, errorText: Option<&crate::include::internal::CefString>, failedUrl: &crate::include::internal::CefString) -> () { Default::default() }
}
define_refcounted!(LoadHandler, CefLoadHandler, cef_load_handler_t, on_loading_state_change: cef_load_handler_t_on_loading_state_change,on_load_start: cef_load_handler_t_on_load_start,on_load_end: cef_load_handler_t_on_load_end,on_load_error: cef_load_handler_t_on_load_error,);
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
