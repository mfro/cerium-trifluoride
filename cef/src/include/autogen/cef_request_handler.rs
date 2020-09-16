pub type CefSelectClientCertificateCallback = crate::include::base::CefProxy<cef_sys::cef_select_client_certificate_callback_t>;
#[allow(non_snake_case)]
impl CefSelectClientCertificateCallback {
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
pub trait RequestHandler {
  fn on_before_browse(&mut self, browser: crate::include::CefBrowser, frame: crate::include::CefFrame, request: crate::include::CefRequest, user_gesture: bool, is_redirect: bool) -> bool { Default::default() }
  fn on_open_urlfrom_tab(&mut self, browser: crate::include::CefBrowser, frame: crate::include::CefFrame, target_url: &crate::include::internal::CefString, target_disposition: crate::include::internal::CefWindowOpenDisposition, user_gesture: bool) -> bool { Default::default() }
  fn get_resource_request_handler(&mut self, browser: crate::include::CefBrowser, frame: crate::include::CefFrame, request: crate::include::CefRequest, is_navigation: bool, is_download: bool, request_initiator: Option<&crate::include::internal::CefString>, disable_default_handling: &mut bool) -> Option<crate::include::CefResourceRequestHandler> { Default::default() }
  fn get_auth_credentials(&mut self, browser: crate::include::CefBrowser, origin_url: &crate::include::internal::CefString, isProxy: bool, host: &crate::include::internal::CefString, port: i32, realm: &crate::include::internal::CefString, scheme: &crate::include::internal::CefString, callback: crate::include::CefAuthCallback) -> bool { Default::default() }
  fn on_quota_request(&mut self, browser: crate::include::CefBrowser, origin_url: &crate::include::internal::CefString, new_size: i64, callback: crate::include::CefRequestCallback) -> bool { Default::default() }
  fn on_certificate_error(&mut self, browser: crate::include::CefBrowser, cert_error: crate::include::internal::CefErrorcode, request_url: &crate::include::internal::CefString, ssl_info: crate::include::CefSSLInfo, callback: crate::include::CefRequestCallback) -> bool { Default::default() }
  fn on_plugin_crashed(&mut self, browser: crate::include::CefBrowser, plugin_path: &crate::include::internal::CefString) -> () { Default::default() }
  fn on_render_view_ready(&mut self, browser: crate::include::CefBrowser) -> () { Default::default() }
  fn on_render_process_terminated(&mut self, browser: crate::include::CefBrowser, status: crate::include::internal::CefTerminationStatus) -> () { Default::default() }
  fn on_document_available_in_main_frame(&mut self, browser: crate::include::CefBrowser) -> () { Default::default() }
}
define_refcounted!(RequestHandler, request_handler, on_before_browse,on_open_urlfrom_tab,get_resource_request_handler,get_auth_credentials,on_quota_request,on_certificate_error,on_plugin_crashed,on_render_view_ready,on_render_process_terminated,on_document_available_in_main_frame,);
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
  let ret = CefRequestHandler::from_cef(_self, true).get().get_auth_credentials(crate::include::CefBrowser::from_cef_own(browser).unwrap(),&crate::include::internal::CefString::from_cef(origin_url).unwrap(),if isProxy == 0 { false } else { true },&crate::include::internal::CefString::from_cef(host).unwrap(),port,&crate::include::internal::CefString::from_cef(realm).unwrap(),&crate::include::internal::CefString::from_cef(scheme).unwrap(),crate::include::CefAuthCallback::from_cef_own(callback).unwrap(),);
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
