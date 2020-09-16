/// Implement this interface to handle events related to browser requests. The
/// methods of this class will be called on the IO thread unless otherwise
/// indicated.
#[allow(non_snake_case)]
pub trait ResourceRequestHandler {
  fn get_cookie_access_filter(&mut self, browser: crate::include::CefBrowser, frame: crate::include::CefFrame, request: crate::include::CefRequest) -> Option<crate::include::CefCookieAccessFilter> { Default::default() }
  fn on_before_resource_load(&mut self, browser: crate::include::CefBrowser, frame: crate::include::CefFrame, request: crate::include::CefRequest, callback: crate::include::CefRequestCallback) -> crate::include::internal::CefReturnValue { Default::default() }
  fn get_resource_handler(&mut self, browser: crate::include::CefBrowser, frame: crate::include::CefFrame, request: crate::include::CefRequest) -> Option<crate::include::CefResourceHandler> { Default::default() }
  fn on_resource_redirect(&mut self, browser: crate::include::CefBrowser, frame: crate::include::CefFrame, request: crate::include::CefRequest, response: crate::include::CefResponse, new_url: &mut crate::include::internal::CefString) -> () { Default::default() }
  fn on_resource_response(&mut self, browser: crate::include::CefBrowser, frame: crate::include::CefFrame, request: crate::include::CefRequest, response: crate::include::CefResponse) -> bool { Default::default() }
  fn get_resource_response_filter(&mut self, browser: crate::include::CefBrowser, frame: crate::include::CefFrame, request: crate::include::CefRequest, response: crate::include::CefResponse) -> Option<crate::include::CefResponseFilter> { Default::default() }
  fn on_resource_load_complete(&mut self, browser: crate::include::CefBrowser, frame: crate::include::CefFrame, request: crate::include::CefRequest, response: crate::include::CefResponse, status: crate::include::internal::CefUrlrequestStatus, received_content_length: i64) -> () { Default::default() }
  fn on_protocol_execution(&mut self, browser: crate::include::CefBrowser, frame: crate::include::CefFrame, request: crate::include::CefRequest, allow_os_execution: &mut bool) -> () { Default::default() }
}
define_refcounted!(ResourceRequestHandler, resource_request_handler, get_cookie_access_filter,on_before_resource_load,get_resource_handler,on_resource_redirect,on_resource_response,get_resource_response_filter,on_resource_load_complete,on_protocol_execution,);
#[allow(non_snake_case)]
unsafe extern "C" fn cef_resource_request_handler_t_get_cookie_access_filter(_self: *mut cef_sys::cef_resource_request_handler_t, browser: *mut cef_sys::cef_browser_t, frame: *mut cef_sys::cef_frame_t, request: *mut cef_sys::cef_request_t) -> *mut cef_sys::cef_cookie_access_filter_t {
  let ret = CefResourceRequestHandler::from_cef(_self, true).get().get_cookie_access_filter(crate::include::CefBrowser::from_cef_own(browser).unwrap(),crate::include::CefFrame::from_cef_own(frame).unwrap(),crate::include::CefRequest::from_cef_own(request).unwrap(),);
  ret.map_or(std::ptr::null_mut(), |o| crate::include::CefCookieAccessFilter::to_cef_own(o))
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_resource_request_handler_t_on_before_resource_load(_self: *mut cef_sys::cef_resource_request_handler_t, browser: *mut cef_sys::cef_browser_t, frame: *mut cef_sys::cef_frame_t, request: *mut cef_sys::cef_request_t, callback: *mut cef_sys::cef_request_callback_t) -> cef_sys::cef_return_value_t {
  let ret = CefResourceRequestHandler::from_cef(_self, true).get().on_before_resource_load(crate::include::CefBrowser::from_cef_own(browser).unwrap(),crate::include::CefFrame::from_cef_own(frame).unwrap(),crate::include::CefRequest::from_cef_own(request).unwrap(),crate::include::CefRequestCallback::from_cef_own(callback).unwrap(),);
  ret.into()
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_resource_request_handler_t_get_resource_handler(_self: *mut cef_sys::cef_resource_request_handler_t, browser: *mut cef_sys::cef_browser_t, frame: *mut cef_sys::cef_frame_t, request: *mut cef_sys::cef_request_t) -> *mut cef_sys::cef_resource_handler_t {
  let ret = CefResourceRequestHandler::from_cef(_self, true).get().get_resource_handler(crate::include::CefBrowser::from_cef_own(browser).unwrap(),crate::include::CefFrame::from_cef_own(frame).unwrap(),crate::include::CefRequest::from_cef_own(request).unwrap(),);
  ret.map_or(std::ptr::null_mut(), |o| crate::include::CefResourceHandler::to_cef_own(o))
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_resource_request_handler_t_on_resource_redirect(_self: *mut cef_sys::cef_resource_request_handler_t, browser: *mut cef_sys::cef_browser_t, frame: *mut cef_sys::cef_frame_t, request: *mut cef_sys::cef_request_t, response: *mut cef_sys::cef_response_t, new_url: *mut cef_sys::cef_string_t) -> () {
  let ret = CefResourceRequestHandler::from_cef(_self, true).get().on_resource_redirect(crate::include::CefBrowser::from_cef_own(browser).unwrap(),crate::include::CefFrame::from_cef_own(frame).unwrap(),crate::include::CefRequest::from_cef_own(request).unwrap(),crate::include::CefResponse::from_cef_own(response).unwrap(),&mut crate::include::internal::CefString::from_cef(new_url).unwrap(),);
  ret
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_resource_request_handler_t_on_resource_response(_self: *mut cef_sys::cef_resource_request_handler_t, browser: *mut cef_sys::cef_browser_t, frame: *mut cef_sys::cef_frame_t, request: *mut cef_sys::cef_request_t, response: *mut cef_sys::cef_response_t) -> i32 {
  let ret = CefResourceRequestHandler::from_cef(_self, true).get().on_resource_response(crate::include::CefBrowser::from_cef_own(browser).unwrap(),crate::include::CefFrame::from_cef_own(frame).unwrap(),crate::include::CefRequest::from_cef_own(request).unwrap(),crate::include::CefResponse::from_cef_own(response).unwrap(),);
  if ret { 1 } else { 0 }
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_resource_request_handler_t_get_resource_response_filter(_self: *mut cef_sys::cef_resource_request_handler_t, browser: *mut cef_sys::cef_browser_t, frame: *mut cef_sys::cef_frame_t, request: *mut cef_sys::cef_request_t, response: *mut cef_sys::cef_response_t) -> *mut cef_sys::cef_response_filter_t {
  let ret = CefResourceRequestHandler::from_cef(_self, true).get().get_resource_response_filter(crate::include::CefBrowser::from_cef_own(browser).unwrap(),crate::include::CefFrame::from_cef_own(frame).unwrap(),crate::include::CefRequest::from_cef_own(request).unwrap(),crate::include::CefResponse::from_cef_own(response).unwrap(),);
  ret.map_or(std::ptr::null_mut(), |o| crate::include::CefResponseFilter::to_cef_own(o))
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_resource_request_handler_t_on_resource_load_complete(_self: *mut cef_sys::cef_resource_request_handler_t, browser: *mut cef_sys::cef_browser_t, frame: *mut cef_sys::cef_frame_t, request: *mut cef_sys::cef_request_t, response: *mut cef_sys::cef_response_t, status: cef_sys::cef_urlrequest_status_t, received_content_length: i64) -> () {
  let ret = CefResourceRequestHandler::from_cef(_self, true).get().on_resource_load_complete(crate::include::CefBrowser::from_cef_own(browser).unwrap(),crate::include::CefFrame::from_cef_own(frame).unwrap(),crate::include::CefRequest::from_cef_own(request).unwrap(),crate::include::CefResponse::from_cef_own(response).unwrap(),status.into(),received_content_length,);
  ret
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_resource_request_handler_t_on_protocol_execution(_self: *mut cef_sys::cef_resource_request_handler_t, browser: *mut cef_sys::cef_browser_t, frame: *mut cef_sys::cef_frame_t, request: *mut cef_sys::cef_request_t, allow_os_execution: *mut i32) -> () {
  let mut allow_os_execution__tmp = if *allow_os_execution == 0 { false } else { true };
  let ret = CefResourceRequestHandler::from_cef(_self, true).get().on_protocol_execution(crate::include::CefBrowser::from_cef_own(browser).unwrap(),crate::include::CefFrame::from_cef_own(frame).unwrap(),crate::include::CefRequest::from_cef_own(request).unwrap(),&mut allow_os_execution__tmp,);
  *allow_os_execution = if allow_os_execution__tmp { 1 } else { 0 };
  ret
}
/// Implement this interface to filter cookies that may be sent or received from
/// resource requests. The methods of this class will be called on the IO thread
/// unless otherwise indicated.
#[allow(non_snake_case)]
pub trait CookieAccessFilter {
  fn can_send_cookie(&mut self, browser: crate::include::CefBrowser, frame: crate::include::CefFrame, request: crate::include::CefRequest, cookie: &crate::include::internal::CefCookie) -> bool { Default::default() }
  fn can_save_cookie(&mut self, browser: crate::include::CefBrowser, frame: crate::include::CefFrame, request: crate::include::CefRequest, response: crate::include::CefResponse, cookie: &crate::include::internal::CefCookie) -> bool { Default::default() }
}
define_refcounted!(CookieAccessFilter, cookie_access_filter, can_send_cookie,can_save_cookie,);
#[allow(non_snake_case)]
unsafe extern "C" fn cef_cookie_access_filter_t_can_send_cookie(_self: *mut cef_sys::cef_cookie_access_filter_t, browser: *mut cef_sys::cef_browser_t, frame: *mut cef_sys::cef_frame_t, request: *mut cef_sys::cef_request_t, cookie: *const cef_sys::cef_cookie_t) -> i32 {
  let ret = CefCookieAccessFilter::from_cef(_self, true).get().can_send_cookie(crate::include::CefBrowser::from_cef_own(browser).unwrap(),crate::include::CefFrame::from_cef_own(frame).unwrap(),crate::include::CefRequest::from_cef_own(request).unwrap(),&*(cookie as *const _),);
  if ret { 1 } else { 0 }
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_cookie_access_filter_t_can_save_cookie(_self: *mut cef_sys::cef_cookie_access_filter_t, browser: *mut cef_sys::cef_browser_t, frame: *mut cef_sys::cef_frame_t, request: *mut cef_sys::cef_request_t, response: *mut cef_sys::cef_response_t, cookie: *const cef_sys::cef_cookie_t) -> i32 {
  let ret = CefCookieAccessFilter::from_cef(_self, true).get().can_save_cookie(crate::include::CefBrowser::from_cef_own(browser).unwrap(),crate::include::CefFrame::from_cef_own(frame).unwrap(),crate::include::CefRequest::from_cef_own(request).unwrap(),crate::include::CefResponse::from_cef_own(response).unwrap(),&*(cookie as *const _),);
  if ret { 1 } else { 0 }
}
