/// Implement this interface to handle events related to browser requests. The
/// methods of this class will be called on the IO thread unless otherwise
/// indicated.
#[allow(non_snake_case)]
#[allow(unused_variables)]
pub trait ResourceRequestHandler {
  /// Called on the IO thread before a resource request is loaded. The |browser|
  /// and |frame| values represent the source of the request, and may be NULL for
  /// requests originating from service workers or CefURLRequest. To optionally
  /// filter cookies for the request return a CefCookieAccessFilter object. The
  /// |request| object cannot not be modified in this callback.
  fn get_cookie_access_filter(&mut self, browser: Option<crate::include::CefBrowser>, frame: Option<crate::include::CefFrame>, request: crate::include::CefRequest) -> Option<crate::include::CefCookieAccessFilter> { Default::default() }
  /// Called on the IO thread before a resource request is loaded. The |browser|
  /// and |frame| values represent the source of the request, and may be NULL for
  /// requests originating from service workers or CefURLRequest. To redirect or
  /// change the resource load optionally modify |request|. Modification of the
  /// request URL will be treated as a redirect. Return RV_CONTINUE to continue
  /// the request immediately. Return RV_CONTINUE_ASYNC and call
  /// CefRequestCallback:: Continue() at a later time to continue or cancel the
  /// request asynchronously. Return RV_CANCEL to cancel the request immediately.
  /// 
  fn on_before_resource_load(&mut self, browser: Option<crate::include::CefBrowser>, frame: Option<crate::include::CefFrame>, request: crate::include::CefRequest, callback: crate::include::CefRequestCallback) -> crate::include::internal::CefReturnValue { Default::default() }
  /// Called on the IO thread before a resource is loaded. The |browser| and
  /// |frame| values represent the source of the request, and may be NULL for
  /// requests originating from service workers or CefURLRequest. To allow the
  /// resource to load using the default network loader return NULL. To specify a
  /// handler for the resource return a CefResourceHandler object. The |request|
  /// object cannot not be modified in this callback.
  fn get_resource_handler(&mut self, browser: Option<crate::include::CefBrowser>, frame: Option<crate::include::CefFrame>, request: crate::include::CefRequest) -> Option<crate::include::CefResourceHandler> { Default::default() }
  /// Called on the IO thread when a resource load is redirected. The |browser|
  /// and |frame| values represent the source of the request, and may be NULL for
  /// requests originating from service workers or CefURLRequest. The |request|
  /// parameter will contain the old URL and other request-related information.
  /// The |response| parameter will contain the response that resulted in the
  /// redirect. The |new_url| parameter will contain the new URL and can be
  /// changed if desired. The |request| and |response| objects cannot be modified
  /// in this callback.
  fn on_resource_redirect(&mut self, browser: Option<crate::include::CefBrowser>, frame: Option<crate::include::CefFrame>, request: crate::include::CefRequest, response: crate::include::CefResponse, new_url: &mut crate::include::internal::CefString) -> () { Default::default() }
  /// Called on the IO thread when a resource response is received. The |browser|
  /// and |frame| values represent the source of the request, and may be NULL for
  /// requests originating from service workers or CefURLRequest. To allow the
  /// resource load to proceed without modification return false. To redirect or
  /// retry the resource load optionally modify |request| and return true.
  /// Modification of the request URL will be treated as a redirect. Requests
  /// handled using the default network loader cannot be redirected in this
  /// callback. The |response| object cannot be modified in this callback.
  /// 
  /// WARNING: Redirecting using this method is deprecated. Use
  /// OnBeforeResourceLoad or GetResourceHandler to perform redirects.
  fn on_resource_response(&mut self, browser: Option<crate::include::CefBrowser>, frame: Option<crate::include::CefFrame>, request: crate::include::CefRequest, response: crate::include::CefResponse) -> bool { Default::default() }
  /// Called on the IO thread to optionally filter resource response content. The
  /// |browser| and |frame| values represent the source of the request, and may
  /// be NULL for requests originating from service workers or CefURLRequest.
  /// |request| and |response| represent the request and response respectively
  /// and cannot be modified in this callback.
  fn get_resource_response_filter(&mut self, browser: Option<crate::include::CefBrowser>, frame: Option<crate::include::CefFrame>, request: crate::include::CefRequest, response: crate::include::CefResponse) -> Option<crate::include::CefResponseFilter> { Default::default() }
  /// Called on the IO thread when a resource load has completed. The |browser|
  /// and |frame| values represent the source of the request, and may be NULL for
  /// requests originating from service workers or CefURLRequest. |request| and
  /// |response| represent the request and response respectively and cannot be
  /// modified in this callback. |status| indicates the load completion status.
  /// |received_content_length| is the number of response bytes actually read.
  /// This method will be called for all requests, including requests that are
  /// aborted due to CEF shutdown or destruction of the associated browser. In
  /// cases where the associated browser is destroyed this callback may arrive
  /// after the CefLifeSpanHandler::OnBeforeClose callback for that browser. The
  /// CefFrame::IsValid method can be used to test for this situation, and care
  /// should be taken not to call |browser| or |frame| methods that modify state
  /// (like LoadURL, SendProcessMessage, etc.) if the frame is invalid.
  fn on_resource_load_complete(&mut self, browser: Option<crate::include::CefBrowser>, frame: Option<crate::include::CefFrame>, request: crate::include::CefRequest, response: crate::include::CefResponse, status: crate::include::internal::CefUrlrequestStatus, received_content_length: i64) -> () { Default::default() }
  /// Called on the IO thread to handle requests for URLs with an unknown
  /// protocol component. The |browser| and |frame| values represent the source
  /// of the request, and may be NULL for requests originating from service
  /// workers or CefURLRequest. |request| cannot be modified in this callback.
  /// Set |allow_os_execution| to true to attempt execution via the registered OS
  /// protocol handler, if any.
  /// SECURITY WARNING: YOU SHOULD USE THIS METHOD TO ENFORCE RESTRICTIONS BASED
  /// ON SCHEME, HOST OR OTHER URL ANALYSIS BEFORE ALLOWING OS EXECUTION.
  fn on_protocol_execution(&mut self, browser: Option<crate::include::CefBrowser>, frame: Option<crate::include::CefFrame>, request: crate::include::CefRequest, allow_os_execution: &mut bool) -> () { Default::default() }
}
define_refcounted!(ResourceRequestHandler, CefResourceRequestHandler, cef_resource_request_handler_t, get_cookie_access_filter: cef_resource_request_handler_t_get_cookie_access_filter,on_before_resource_load: cef_resource_request_handler_t_on_before_resource_load,get_resource_handler: cef_resource_request_handler_t_get_resource_handler,on_resource_redirect: cef_resource_request_handler_t_on_resource_redirect,on_resource_response: cef_resource_request_handler_t_on_resource_response,get_resource_response_filter: cef_resource_request_handler_t_get_resource_response_filter,on_resource_load_complete: cef_resource_request_handler_t_on_resource_load_complete,on_protocol_execution: cef_resource_request_handler_t_on_protocol_execution,);
#[allow(non_snake_case)]
unsafe extern "C" fn cef_resource_request_handler_t_get_cookie_access_filter(_self: *mut cef_sys::cef_resource_request_handler_t, browser: *mut cef_sys::cef_browser_t, frame: *mut cef_sys::cef_frame_t, request: *mut cef_sys::cef_request_t) -> *mut cef_sys::cef_cookie_access_filter_t {
  let ret = CefResourceRequestHandler::from_cef(_self, true).get().get_cookie_access_filter(crate::include::CefBrowser::from_cef_own(browser),crate::include::CefFrame::from_cef_own(frame),crate::include::CefRequest::from_cef_own(request).unwrap(),);
  ret.map_or(std::ptr::null_mut(), |o| crate::include::CefCookieAccessFilter::to_cef_own(o))
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_resource_request_handler_t_on_before_resource_load(_self: *mut cef_sys::cef_resource_request_handler_t, browser: *mut cef_sys::cef_browser_t, frame: *mut cef_sys::cef_frame_t, request: *mut cef_sys::cef_request_t, callback: *mut cef_sys::cef_request_callback_t) -> cef_sys::cef_return_value_t {
  let ret = CefResourceRequestHandler::from_cef(_self, true).get().on_before_resource_load(crate::include::CefBrowser::from_cef_own(browser),crate::include::CefFrame::from_cef_own(frame),crate::include::CefRequest::from_cef_own(request).unwrap(),crate::include::CefRequestCallback::from_cef_own(callback).unwrap(),);
  ret.into()
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_resource_request_handler_t_get_resource_handler(_self: *mut cef_sys::cef_resource_request_handler_t, browser: *mut cef_sys::cef_browser_t, frame: *mut cef_sys::cef_frame_t, request: *mut cef_sys::cef_request_t) -> *mut cef_sys::cef_resource_handler_t {
  let ret = CefResourceRequestHandler::from_cef(_self, true).get().get_resource_handler(crate::include::CefBrowser::from_cef_own(browser),crate::include::CefFrame::from_cef_own(frame),crate::include::CefRequest::from_cef_own(request).unwrap(),);
  ret.map_or(std::ptr::null_mut(), |o| crate::include::CefResourceHandler::to_cef_own(o))
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_resource_request_handler_t_on_resource_redirect(_self: *mut cef_sys::cef_resource_request_handler_t, browser: *mut cef_sys::cef_browser_t, frame: *mut cef_sys::cef_frame_t, request: *mut cef_sys::cef_request_t, response: *mut cef_sys::cef_response_t, new_url: *mut cef_sys::cef_string_t) -> () {
  let ret = CefResourceRequestHandler::from_cef(_self, true).get().on_resource_redirect(crate::include::CefBrowser::from_cef_own(browser),crate::include::CefFrame::from_cef_own(frame),crate::include::CefRequest::from_cef_own(request).unwrap(),crate::include::CefResponse::from_cef_own(response).unwrap(),&mut *(new_url as *mut _),);
  ret
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_resource_request_handler_t_on_resource_response(_self: *mut cef_sys::cef_resource_request_handler_t, browser: *mut cef_sys::cef_browser_t, frame: *mut cef_sys::cef_frame_t, request: *mut cef_sys::cef_request_t, response: *mut cef_sys::cef_response_t) -> i32 {
  let ret = CefResourceRequestHandler::from_cef(_self, true).get().on_resource_response(crate::include::CefBrowser::from_cef_own(browser),crate::include::CefFrame::from_cef_own(frame),crate::include::CefRequest::from_cef_own(request).unwrap(),crate::include::CefResponse::from_cef_own(response).unwrap(),);
  if ret { 1 } else { 0 }
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_resource_request_handler_t_get_resource_response_filter(_self: *mut cef_sys::cef_resource_request_handler_t, browser: *mut cef_sys::cef_browser_t, frame: *mut cef_sys::cef_frame_t, request: *mut cef_sys::cef_request_t, response: *mut cef_sys::cef_response_t) -> *mut cef_sys::cef_response_filter_t {
  let ret = CefResourceRequestHandler::from_cef(_self, true).get().get_resource_response_filter(crate::include::CefBrowser::from_cef_own(browser),crate::include::CefFrame::from_cef_own(frame),crate::include::CefRequest::from_cef_own(request).unwrap(),crate::include::CefResponse::from_cef_own(response).unwrap(),);
  ret.map_or(std::ptr::null_mut(), |o| crate::include::CefResponseFilter::to_cef_own(o))
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_resource_request_handler_t_on_resource_load_complete(_self: *mut cef_sys::cef_resource_request_handler_t, browser: *mut cef_sys::cef_browser_t, frame: *mut cef_sys::cef_frame_t, request: *mut cef_sys::cef_request_t, response: *mut cef_sys::cef_response_t, status: cef_sys::cef_urlrequest_status_t, received_content_length: i64) -> () {
  let ret = CefResourceRequestHandler::from_cef(_self, true).get().on_resource_load_complete(crate::include::CefBrowser::from_cef_own(browser),crate::include::CefFrame::from_cef_own(frame),crate::include::CefRequest::from_cef_own(request).unwrap(),crate::include::CefResponse::from_cef_own(response).unwrap(),status.into(),received_content_length,);
  ret
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_resource_request_handler_t_on_protocol_execution(_self: *mut cef_sys::cef_resource_request_handler_t, browser: *mut cef_sys::cef_browser_t, frame: *mut cef_sys::cef_frame_t, request: *mut cef_sys::cef_request_t, allow_os_execution: *mut i32) -> () {
  let mut allow_os_execution__tmp = if *allow_os_execution == 0 { false } else { true };
  let ret = CefResourceRequestHandler::from_cef(_self, true).get().on_protocol_execution(crate::include::CefBrowser::from_cef_own(browser),crate::include::CefFrame::from_cef_own(frame),crate::include::CefRequest::from_cef_own(request).unwrap(),&mut allow_os_execution__tmp,);
  *allow_os_execution = if allow_os_execution__tmp { 1 } else { 0 };
  ret
}
/// Implement this interface to filter cookies that may be sent or received from
/// resource requests. The methods of this class will be called on the IO thread
/// unless otherwise indicated.
#[allow(non_snake_case)]
#[allow(unused_variables)]
pub trait CookieAccessFilter {
  /// Called on the IO thread before a resource request is sent. The |browser|
  /// and |frame| values represent the source of the request, and may be NULL for
  /// requests originating from service workers or CefURLRequest. |request|
  /// cannot be modified in this callback. Return true if the specified cookie
  /// can be sent with the request or false otherwise.
  fn can_send_cookie(&mut self, browser: Option<crate::include::CefBrowser>, frame: Option<crate::include::CefFrame>, request: crate::include::CefRequest, cookie: &crate::include::internal::CefCookie) -> bool { Default::default() }
  /// Called on the IO thread after a resource response is received. The
  /// |browser| and |frame| values represent the source of the request, and may
  /// be NULL for requests originating from service workers or CefURLRequest.
  /// |request| cannot be modified in this callback. Return true if the specified
  /// cookie returned with the response can be saved or false otherwise.
  fn can_save_cookie(&mut self, browser: Option<crate::include::CefBrowser>, frame: Option<crate::include::CefFrame>, request: crate::include::CefRequest, response: crate::include::CefResponse, cookie: &crate::include::internal::CefCookie) -> bool { Default::default() }
}
define_refcounted!(CookieAccessFilter, CefCookieAccessFilter, cef_cookie_access_filter_t, can_send_cookie: cef_cookie_access_filter_t_can_send_cookie,can_save_cookie: cef_cookie_access_filter_t_can_save_cookie,);
#[allow(non_snake_case)]
unsafe extern "C" fn cef_cookie_access_filter_t_can_send_cookie(_self: *mut cef_sys::cef_cookie_access_filter_t, browser: *mut cef_sys::cef_browser_t, frame: *mut cef_sys::cef_frame_t, request: *mut cef_sys::cef_request_t, cookie: *const cef_sys::cef_cookie_t) -> i32 {
  let ret = CefCookieAccessFilter::from_cef(_self, true).get().can_send_cookie(crate::include::CefBrowser::from_cef_own(browser),crate::include::CefFrame::from_cef_own(frame),crate::include::CefRequest::from_cef_own(request).unwrap(),&*(cookie as *const _),);
  if ret { 1 } else { 0 }
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_cookie_access_filter_t_can_save_cookie(_self: *mut cef_sys::cef_cookie_access_filter_t, browser: *mut cef_sys::cef_browser_t, frame: *mut cef_sys::cef_frame_t, request: *mut cef_sys::cef_request_t, response: *mut cef_sys::cef_response_t, cookie: *const cef_sys::cef_cookie_t) -> i32 {
  let ret = CefCookieAccessFilter::from_cef(_self, true).get().can_save_cookie(crate::include::CefBrowser::from_cef_own(browser),crate::include::CefFrame::from_cef_own(frame),crate::include::CefRequest::from_cef_own(request).unwrap(),crate::include::CefResponse::from_cef_own(response).unwrap(),&*(cookie as *const _),);
  if ret { 1 } else { 0 }
}
