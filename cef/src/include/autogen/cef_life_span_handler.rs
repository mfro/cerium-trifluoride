/// Implement this interface to handle events related to browser life span. The
/// methods of this class will be called on the UI thread unless otherwise
/// indicated.
#[allow(non_snake_case)]
#[allow(unused_variables)]
pub trait LifeSpanHandler {
  /// Called on the UI thread before a new popup browser is created. The
  /// |browser| and |frame| values represent the source of the popup request. The
  /// |target_url| and |target_frame_name| values indicate where the popup
  /// browser should navigate and may be empty if not specified with the request.
  /// The |target_disposition| value indicates where the user intended to open
  /// the popup (e.g. current tab, new tab, etc). The |user_gesture| value will
  /// be true if the popup was opened via explicit user gesture (e.g. clicking a
  /// link) or false if the popup opened automatically (e.g. via the
  /// DomContentLoaded event). The |popupFeatures| structure contains additional
  /// information about the requested popup window. To allow creation of the
  /// popup browser optionally modify |windowInfo|, |client|, |settings| and
  /// |no_javascript_access| and return false. To cancel creation of the popup
  /// browser return true. The |client| and |settings| values will default to the
  /// source browser's values. If the |no_javascript_access| value is set to
  /// false the new browser will not be scriptable and may not be hosted in the
  /// same renderer process as the source browser. Any modifications to
  /// |windowInfo| will be ignored if the parent browser is wrapped in a
  /// CefBrowserView. Popup browser creation will be canceled if the parent
  /// browser is destroyed before the popup browser creation completes (indicated
  /// by a call to OnAfterCreated for the popup browser). The |extra_info|
  /// parameter provides an opportunity to specify extra information specific
  /// to the created popup browser that will be passed to
  /// CefRenderProcessHandler::OnBrowserCreated() in the render process.
  fn on_before_popup(&mut self, browser: crate::include::CefBrowser, frame: crate::include::CefFrame, target_url: Option<&crate::include::internal::CefString>, target_frame_name: Option<&crate::include::internal::CefString>, target_disposition: crate::include::internal::CefWindowOpenDisposition, user_gesture: bool, popupFeatures: &crate::include::internal::CefPopupFeatures, windowInfo: &mut crate::include::internal::CefWindowInfo, client: &mut crate::include::CefClient, settings: &mut crate::include::internal::CefBrowserSettings, extra_info: &mut crate::include::CefDictionaryValue, no_javascript_access: &mut bool) -> bool { Default::default() }
  /// Called after a new browser is created. This callback will be the first
  /// notification that references |browser|.
  fn on_after_created(&mut self, browser: crate::include::CefBrowser) -> () { Default::default() }
  /// Called when a browser has recieved a request to close. This may result
  /// directly from a call to CefBrowserHost::*CloseBrowser() or indirectly if
  /// the browser is parented to a top-level window created by CEF and the user
  /// attempts to close that window (by clicking the 'X', for example). The
  /// DoClose() method will be called after the JavaScript 'onunload' event has
  /// been fired.
  /// 
  /// An application should handle top-level owner window close notifications by
  /// calling CefBrowserHost::TryCloseBrowser() or
  /// CefBrowserHost::CloseBrowser(false) instead of allowing the window to close
  /// immediately (see the examples below). This gives CEF an opportunity to
  /// process the 'onbeforeunload' event and optionally cancel the close before
  /// DoClose() is called.
  /// 
  /// When windowed rendering is enabled CEF will internally create a window or
  /// view to host the browser. In that case returning false from DoClose() will
  /// send the standard close notification to the browser's top-level owner
  /// window (e.g. WM_CLOSE on Windows, performClose: on OS X, "delete_event" on
  /// Linux or CefWindowDelegate::CanClose() callback from Views). If the
  /// browser's host window/view has already been destroyed (via view hierarchy
  /// tear-down, for example) then DoClose() will not be called for that browser
  /// since is no longer possible to cancel the close.
  /// 
  /// When windowed rendering is disabled returning false from DoClose() will
  /// cause the browser object to be destroyed immediately.
  /// 
  /// If the browser's top-level owner window requires a non-standard close
  /// notification then send that notification from DoClose() and return true.
  /// 
  /// The CefLifeSpanHandler::OnBeforeClose() method will be called after
  /// DoClose() (if DoClose() is called) and immediately before the browser
  /// object is destroyed. The application should only exit after OnBeforeClose()
  /// has been called for all existing browsers.
  /// 
  /// The below examples describe what should happen during window close when the
  /// browser is parented to an application-provided top-level window.
  /// 
  /// Example 1: Using CefBrowserHost::TryCloseBrowser(). This is recommended for
  /// clients using standard close handling and windows created on the browser
  /// process UI thread.
  /// 1.  User clicks the window close button which sends a close notification to
  /// the application's top-level window.
  /// 2.  Application's top-level window receives the close notification and
  /// calls TryCloseBrowser() (which internally calls CloseBrowser(false)).
  /// TryCloseBrowser() returns false so the client cancels the window close.
  /// 3.  JavaScript 'onbeforeunload' handler executes and shows the close
  /// confirmation dialog (which can be overridden via
  /// CefJSDialogHandler::OnBeforeUnloadDialog()).
  /// 4.  User approves the close.
  /// 5.  JavaScript 'onunload' handler executes.
  /// 6.  CEF sends a close notification to the application's top-level window
  /// (because DoClose() returned false by default).
  /// 7.  Application's top-level window receives the close notification and
  /// calls TryCloseBrowser(). TryCloseBrowser() returns true so the client
  /// allows the window close.
  /// 8.  Application's top-level window is destroyed.
  /// 9.  Application's OnBeforeClose() handler is called and the browser object
  /// is destroyed.
  /// 10. Application exits by calling CefQuitMessageLoop() if no other browsers
  /// exist.
  /// 
  /// Example 2: Using CefBrowserHost::CloseBrowser(false) and implementing the
  /// DoClose() callback. This is recommended for clients using non-standard
  /// close handling or windows that were not created on the browser process UI
  /// thread.
  /// 1.  User clicks the window close button which sends a close notification to
  /// the application's top-level window.
  /// 2.  Application's top-level window receives the close notification and:
  /// A. Calls CefBrowserHost::CloseBrowser(false).
  /// B. Cancels the window close.
  /// 3.  JavaScript 'onbeforeunload' handler executes and shows the close
  /// confirmation dialog (which can be overridden via
  /// CefJSDialogHandler::OnBeforeUnloadDialog()).
  /// 4.  User approves the close.
  /// 5.  JavaScript 'onunload' handler executes.
  /// 6.  Application's DoClose() handler is called. Application will:
  /// A. Set a flag to indicate that the next close attempt will be allowed.
  /// B. Return false.
  /// 7.  CEF sends an close notification to the application's top-level window.
  /// 8.  Application's top-level window receives the close notification and
  /// allows the window to close based on the flag from #6B.
  /// 9.  Application's top-level window is destroyed.
  /// 10. Application's OnBeforeClose() handler is called and the browser object
  /// is destroyed.
  /// 11. Application exits by calling CefQuitMessageLoop() if no other browsers
  /// exist.
  fn do_close(&mut self, browser: crate::include::CefBrowser) -> bool { Default::default() }
  /// Called just before a browser is destroyed. Release all references to the
  /// browser object and do not attempt to execute any methods on the browser
  /// object (other than GetIdentifier or IsSame) after this callback returns.
  /// This callback will be the last notification that references |browser| on
  /// the UI thread. Any in-progress network requests associated with |browser|
  /// will be aborted when the browser is destroyed, and
  /// CefResourceRequestHandler callbacks related to those requests may still
  /// arrive on the IO thread after this method is called. See DoClose()
  /// documentation for additional usage information.
  fn on_before_close(&mut self, browser: crate::include::CefBrowser) -> () { Default::default() }
}
define_refcounted!(LifeSpanHandler, CefLifeSpanHandler, cef_life_span_handler_t, on_before_popup: cef_life_span_handler_t_on_before_popup,on_after_created: cef_life_span_handler_t_on_after_created,do_close: cef_life_span_handler_t_do_close,on_before_close: cef_life_span_handler_t_on_before_close,);
#[allow(non_snake_case)]
unsafe extern "C" fn cef_life_span_handler_t_on_before_popup(_self: *mut cef_sys::cef_life_span_handler_t, browser: *mut cef_sys::cef_browser_t, frame: *mut cef_sys::cef_frame_t, target_url: *const cef_sys::cef_string_t, target_frame_name: *const cef_sys::cef_string_t, target_disposition: cef_sys::cef_window_open_disposition_t, user_gesture: i32, popupFeatures: *const cef_sys::cef_popup_features_t, windowInfo: *mut cef_sys::cef_window_info_t, client: *mut *mut cef_sys::cef_client_t, settings: *mut cef_sys::cef_browser_settings_t, extra_info: *mut *mut cef_sys::cef_dictionary_value_t, no_javascript_access: *mut i32) -> i32 {
  let mut client__tmp = crate::include::CefClient::from_cef_own(*client).unwrap();
  let mut extra_info__tmp = crate::include::CefDictionaryValue::from_cef_own(*extra_info).unwrap();
  let mut no_javascript_access__tmp = if *no_javascript_access == 0 { false } else { true };
  let ret = CefLifeSpanHandler::from_cef(_self, true).get().on_before_popup(crate::include::CefBrowser::from_cef_own(browser).unwrap(),crate::include::CefFrame::from_cef_own(frame).unwrap(),match &crate::include::internal::CefString::from_cef(target_url) { Some(ref x) => Some(x), None => None },match &crate::include::internal::CefString::from_cef(target_frame_name) { Some(ref x) => Some(x), None => None },target_disposition.into(),if user_gesture == 0 { false } else { true },&*(popupFeatures as *const _),&mut *(windowInfo as *mut _),&mut client__tmp,&mut *(settings as *mut _),&mut extra_info__tmp,&mut no_javascript_access__tmp,);
  *client = crate::include::CefClient::to_cef_own(client__tmp);
  *extra_info = crate::include::CefDictionaryValue::to_cef_own(extra_info__tmp);
  *no_javascript_access = if no_javascript_access__tmp { 1 } else { 0 };
  if ret { 1 } else { 0 }
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_life_span_handler_t_on_after_created(_self: *mut cef_sys::cef_life_span_handler_t, browser: *mut cef_sys::cef_browser_t) -> () {
  let ret = CefLifeSpanHandler::from_cef(_self, true).get().on_after_created(crate::include::CefBrowser::from_cef_own(browser).unwrap(),);
  ret
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_life_span_handler_t_do_close(_self: *mut cef_sys::cef_life_span_handler_t, browser: *mut cef_sys::cef_browser_t) -> i32 {
  let ret = CefLifeSpanHandler::from_cef(_self, true).get().do_close(crate::include::CefBrowser::from_cef_own(browser).unwrap(),);
  if ret { 1 } else { 0 }
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_life_span_handler_t_on_before_close(_self: *mut cef_sys::cef_life_span_handler_t, browser: *mut cef_sys::cef_browser_t) -> () {
  let ret = CefLifeSpanHandler::from_cef(_self, true).get().on_before_close(crate::include::CefBrowser::from_cef_own(browser).unwrap(),);
  ret
}