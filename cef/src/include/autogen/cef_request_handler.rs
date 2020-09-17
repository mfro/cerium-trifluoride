pub type CefSelectClientCertificateCallback = crate::include::base::CefProxy<cef_sys::cef_select_client_certificate_callback_t>;
#[allow(non_snake_case)]
impl CefSelectClientCertificateCallback {
  /// Chooses the specified certificate for client certificate authentication.
  /// NULL value means that no client certificate should be used.
  pub fn select(&mut self, cert: Option<crate::include::CefX509Certificate>) -> () {
    unsafe {
      let ret = match self.raw.as_ref().select {
        Some(f) => f(self.raw.as_ptr(),cert.map_or(std::ptr::null_mut(), |o| crate::include::CefX509Certificate::to_cef_own(o)),),
        None => panic!(),
      };
      ret
    }
  }
}
/// Implement this interface to handle events related to browser requests. The
/// methods of this class will be called on the thread indicated.
#[allow(non_snake_case)]
#[allow(unused_variables)]
pub trait RequestHandler {
  /// Called on the UI thread before browser navigation. Return true to cancel
  /// the navigation or false to allow the navigation to proceed. The |request|
  /// object cannot be modified in this callback.
  /// CefLoadHandler::OnLoadingStateChange will be called twice in all cases.
  /// If the navigation is allowed CefLoadHandler::OnLoadStart and
  /// CefLoadHandler::OnLoadEnd will be called. If the navigation is canceled
  /// CefLoadHandler::OnLoadError will be called with an |errorCode| value of
  /// ERR_ABORTED. The |user_gesture| value will be true if the browser
  /// navigated via explicit user gesture (e.g. clicking a link) or false if it
  /// navigated automatically (e.g. via the DomContentLoaded event).
  fn on_before_browse(&mut self, browser: crate::include::CefBrowser, frame: crate::include::CefFrame, request: crate::include::CefRequest, user_gesture: bool, is_redirect: bool) -> bool { Default::default() }
  /// Called on the UI thread before OnBeforeBrowse in certain limited cases
  /// where navigating a new or different browser might be desirable. This
  /// includes user-initiated navigation that might open in a special way (e.g.
  /// links clicked via middle-click or ctrl + left-click) and certain types of
  /// cross-origin navigation initiated from the renderer process (e.g.
  /// navigating the top-level frame to/from a file URL). The |browser| and
  /// |frame| values represent the source of the navigation. The
  /// |target_disposition| value indicates where the user intended to navigate
  /// the browser based on standard Chromium behaviors (e.g. current tab,
  /// new tab, etc). The |user_gesture| value will be true if the browser
  /// navigated via explicit user gesture (e.g. clicking a link) or false if it
  /// navigated automatically (e.g. via the DomContentLoaded event). Return true
  /// to cancel the navigation or false to allow the navigation to proceed in the
  /// source browser's top-level frame.
  fn on_open_urlfrom_tab(&mut self, browser: crate::include::CefBrowser, frame: crate::include::CefFrame, target_url: &crate::include::internal::CefString, target_disposition: crate::include::internal::CefWindowOpenDisposition, user_gesture: bool) -> bool { Default::default() }
  /// Called on the browser process IO thread before a resource request is
  /// initiated. The |browser| and |frame| values represent the source of the
  /// request. |request| represents the request contents and cannot be modified
  /// in this callback. |is_navigation| will be true if the resource request is a
  /// navigation. |is_download| will be true if the resource request is a
  /// download. |request_initiator| is the origin (scheme + domain) of the page
  /// that initiated the request. Set |disable_default_handling| to true to
  /// disable default handling of the request, in which case it will need to be
  /// handled via CefResourceRequestHandler::GetResourceHandler or it will be
  /// canceled. To allow the resource load to proceed with default handling
  /// return NULL. To specify a handler for the resource return a
  /// CefResourceRequestHandler object. If this callback returns NULL the same
  /// method will be called on the associated CefRequestContextHandler, if any.
  fn get_resource_request_handler(&mut self, browser: crate::include::CefBrowser, frame: crate::include::CefFrame, request: crate::include::CefRequest, is_navigation: bool, is_download: bool, request_initiator: Option<&crate::include::internal::CefString>, disable_default_handling: &mut bool) -> Option<crate::include::CefResourceRequestHandler> { Default::default() }
  /// Called on the IO thread when the browser needs credentials from the user.
  /// |origin_url| is the origin making this authentication request. |isProxy|
  /// indicates whether the host is a proxy server. |host| contains the hostname
  /// and |port| contains the port number. |realm| is the realm of the challenge
  /// and may be empty. |scheme| is the authentication scheme used, such as
  /// "basic" or "digest", and will be empty if the source of the request is an
  /// FTP server. Return true to continue the request and call
  /// CefAuthCallback::Continue() either in this method or at a later time when
  /// the authentication information is available. Return false to cancel the
  /// request immediately.
  fn get_auth_credentials(&mut self, browser: crate::include::CefBrowser, origin_url: &crate::include::internal::CefString, isProxy: bool, host: &crate::include::internal::CefString, port: i32, realm: Option<&crate::include::internal::CefString>, scheme: Option<&crate::include::internal::CefString>, callback: crate::include::CefAuthCallback) -> bool { Default::default() }
  /// Called on the IO thread when JavaScript requests a specific storage quota
  /// size via the webkitStorageInfo.requestQuota function. |origin_url| is the
  /// origin of the page making the request. |new_size| is the requested quota
  /// size in bytes. Return true to continue the request and call
  /// CefRequestCallback::Continue() either in this method or at a later time to
  /// grant or deny the request. Return false to cancel the request immediately.
  fn on_quota_request(&mut self, browser: crate::include::CefBrowser, origin_url: &crate::include::internal::CefString, new_size: i64, callback: crate::include::CefRequestCallback) -> bool { Default::default() }
  /// Called on the UI thread to handle requests for URLs with an invalid
  /// SSL certificate. Return true and call CefRequestCallback::Continue() either
  /// in this method or at a later time to continue or cancel the request. Return
  /// false to cancel the request immediately. If
  /// CefSettings.ignore_certificate_errors is set all invalid certificates will
  /// be accepted without calling this method.
  fn on_certificate_error(&mut self, browser: crate::include::CefBrowser, cert_error: crate::include::internal::CefErrorcode, request_url: &crate::include::internal::CefString, ssl_info: crate::include::CefSSLInfo, callback: crate::include::CefRequestCallback) -> bool { Default::default() }
  /// Called on the browser process UI thread when a plugin has crashed.
  /// |plugin_path| is the path of the plugin that crashed.
  fn on_plugin_crashed(&mut self, browser: crate::include::CefBrowser, plugin_path: &crate::include::internal::CefString) -> () { Default::default() }
  /// Called on the browser process UI thread when the render view associated
  /// with |browser| is ready to receive/handle IPC messages in the render
  /// process.
  fn on_render_view_ready(&mut self, browser: crate::include::CefBrowser) -> () { Default::default() }
  /// Called on the browser process UI thread when the render process
  /// terminates unexpectedly. |status| indicates how the process
  /// terminated.
  fn on_render_process_terminated(&mut self, browser: crate::include::CefBrowser, status: crate::include::internal::CefTerminationStatus) -> () { Default::default() }
  /// Called on the browser process UI thread when the window.document object of
  /// the main frame has been created.
  fn on_document_available_in_main_frame(&mut self, browser: crate::include::CefBrowser) -> () { Default::default() }
}
define_refcounted!(RequestHandler, CefRequestHandler, cef_request_handler_t, on_before_browse: cef_request_handler_t_on_before_browse,on_open_urlfrom_tab: cef_request_handler_t_on_open_urlfrom_tab,get_resource_request_handler: cef_request_handler_t_get_resource_request_handler,get_auth_credentials: cef_request_handler_t_get_auth_credentials,on_quota_request: cef_request_handler_t_on_quota_request,on_certificate_error: cef_request_handler_t_on_certificate_error,on_plugin_crashed: cef_request_handler_t_on_plugin_crashed,on_render_view_ready: cef_request_handler_t_on_render_view_ready,on_render_process_terminated: cef_request_handler_t_on_render_process_terminated,on_document_available_in_main_frame: cef_request_handler_t_on_document_available_in_main_frame,);
#[allow(non_snake_case)]
unsafe extern "C" fn cef_request_handler_t_on_before_browse(_self: *mut cef_sys::cef_request_handler_t, browser: *mut cef_sys::cef_browser_t, frame: *mut cef_sys::cef_frame_t, request: *mut cef_sys::cef_request_t, user_gesture: i32, is_redirect: i32) -> i32 {
  let ret = CefRequestHandler::from_cef(_self, true).get().on_before_browse(crate::include::CefBrowser::from_cef_own(browser).unwrap(),crate::include::CefFrame::from_cef_own(frame).unwrap(),crate::include::CefRequest::from_cef_own(request).unwrap(),if user_gesture == 0 { false } else { true },if is_redirect == 0 { false } else { true },);
  if ret { 1 } else { 0 }
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_request_handler_t_on_open_urlfrom_tab(_self: *mut cef_sys::cef_request_handler_t, browser: *mut cef_sys::cef_browser_t, frame: *mut cef_sys::cef_frame_t, target_url: *const cef_sys::cef_string_t, target_disposition: cef_sys::cef_window_open_disposition_t, user_gesture: i32) -> i32 {
  let ret = CefRequestHandler::from_cef(_self, true).get().on_open_urlfrom_tab(crate::include::CefBrowser::from_cef_own(browser).unwrap(),crate::include::CefFrame::from_cef_own(frame).unwrap(),&crate::include::internal::CefString::from_cef(target_url).unwrap(),target_disposition.into(),if user_gesture == 0 { false } else { true },);
  if ret { 1 } else { 0 }
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_request_handler_t_get_resource_request_handler(_self: *mut cef_sys::cef_request_handler_t, browser: *mut cef_sys::cef_browser_t, frame: *mut cef_sys::cef_frame_t, request: *mut cef_sys::cef_request_t, is_navigation: i32, is_download: i32, request_initiator: *const cef_sys::cef_string_t, disable_default_handling: *mut i32) -> *mut cef_sys::cef_resource_request_handler_t {
  let mut disable_default_handling__tmp = if *disable_default_handling == 0 { false } else { true };
  let ret = CefRequestHandler::from_cef(_self, true).get().get_resource_request_handler(crate::include::CefBrowser::from_cef_own(browser).unwrap(),crate::include::CefFrame::from_cef_own(frame).unwrap(),crate::include::CefRequest::from_cef_own(request).unwrap(),if is_navigation == 0 { false } else { true },if is_download == 0 { false } else { true },match &crate::include::internal::CefString::from_cef(request_initiator) { Some(ref x) => Some(x), None => None },&mut disable_default_handling__tmp,);
  *disable_default_handling = if disable_default_handling__tmp { 1 } else { 0 };
  ret.map_or(std::ptr::null_mut(), |o| crate::include::CefResourceRequestHandler::to_cef_own(o))
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_request_handler_t_get_auth_credentials(_self: *mut cef_sys::cef_request_handler_t, browser: *mut cef_sys::cef_browser_t, origin_url: *const cef_sys::cef_string_t, isProxy: i32, host: *const cef_sys::cef_string_t, port: i32, realm: *const cef_sys::cef_string_t, scheme: *const cef_sys::cef_string_t, callback: *mut cef_sys::cef_auth_callback_t) -> i32 {
  let ret = CefRequestHandler::from_cef(_self, true).get().get_auth_credentials(crate::include::CefBrowser::from_cef_own(browser).unwrap(),&crate::include::internal::CefString::from_cef(origin_url).unwrap(),if isProxy == 0 { false } else { true },&crate::include::internal::CefString::from_cef(host).unwrap(),port,match &crate::include::internal::CefString::from_cef(realm) { Some(ref x) => Some(x), None => None },match &crate::include::internal::CefString::from_cef(scheme) { Some(ref x) => Some(x), None => None },crate::include::CefAuthCallback::from_cef_own(callback).unwrap(),);
  if ret { 1 } else { 0 }
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_request_handler_t_on_quota_request(_self: *mut cef_sys::cef_request_handler_t, browser: *mut cef_sys::cef_browser_t, origin_url: *const cef_sys::cef_string_t, new_size: i64, callback: *mut cef_sys::cef_request_callback_t) -> i32 {
  let ret = CefRequestHandler::from_cef(_self, true).get().on_quota_request(crate::include::CefBrowser::from_cef_own(browser).unwrap(),&crate::include::internal::CefString::from_cef(origin_url).unwrap(),new_size,crate::include::CefRequestCallback::from_cef_own(callback).unwrap(),);
  if ret { 1 } else { 0 }
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_request_handler_t_on_certificate_error(_self: *mut cef_sys::cef_request_handler_t, browser: *mut cef_sys::cef_browser_t, cert_error: cef_sys::cef_errorcode_t, request_url: *const cef_sys::cef_string_t, ssl_info: *mut cef_sys::cef_sslinfo_t, callback: *mut cef_sys::cef_request_callback_t) -> i32 {
  let ret = CefRequestHandler::from_cef(_self, true).get().on_certificate_error(crate::include::CefBrowser::from_cef_own(browser).unwrap(),cert_error.into(),&crate::include::internal::CefString::from_cef(request_url).unwrap(),crate::include::CefSSLInfo::from_cef_own(ssl_info).unwrap(),crate::include::CefRequestCallback::from_cef_own(callback).unwrap(),);
  if ret { 1 } else { 0 }
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_request_handler_t_on_plugin_crashed(_self: *mut cef_sys::cef_request_handler_t, browser: *mut cef_sys::cef_browser_t, plugin_path: *const cef_sys::cef_string_t) -> () {
  let ret = CefRequestHandler::from_cef(_self, true).get().on_plugin_crashed(crate::include::CefBrowser::from_cef_own(browser).unwrap(),&crate::include::internal::CefString::from_cef(plugin_path).unwrap(),);
  ret
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_request_handler_t_on_render_view_ready(_self: *mut cef_sys::cef_request_handler_t, browser: *mut cef_sys::cef_browser_t) -> () {
  let ret = CefRequestHandler::from_cef(_self, true).get().on_render_view_ready(crate::include::CefBrowser::from_cef_own(browser).unwrap(),);
  ret
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_request_handler_t_on_render_process_terminated(_self: *mut cef_sys::cef_request_handler_t, browser: *mut cef_sys::cef_browser_t, status: cef_sys::cef_termination_status_t) -> () {
  let ret = CefRequestHandler::from_cef(_self, true).get().on_render_process_terminated(crate::include::CefBrowser::from_cef_own(browser).unwrap(),status.into(),);
  ret
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_request_handler_t_on_document_available_in_main_frame(_self: *mut cef_sys::cef_request_handler_t, browser: *mut cef_sys::cef_browser_t) -> () {
  let ret = CefRequestHandler::from_cef(_self, true).get().on_document_available_in_main_frame(crate::include::CefBrowser::from_cef_own(browser).unwrap(),);
  ret
}
