pub type CefURLRequest = crate::include::base::CefProxy<cef_sys::cef_urlrequest_t>;
#[allow(non_snake_case)]
impl CefURLRequest {
  /// Create a new URL request that is not associated with a specific browser or
  /// frame. Use CefFrame::CreateURLRequest instead if you want the request to
  /// have this association, in which case it may be handled differently (see
  /// documentation on that method). Requests may originate from the both browser
  /// process and the render process.
  /// 
  /// For requests originating from the browser process:
  /// - It may be intercepted by the client via CefResourceRequestHandler or
  /// CefSchemeHandlerFactory.
  /// - POST data may only contain only a single element of type PDE_TYPE_FILE
  /// or PDE_TYPE_BYTES.
  /// - If |request_context| is empty the global request context will be used.
  /// For requests originating from the render process:
  /// - It cannot be intercepted by the client so only http(s) and blob schemes
  /// are supported.
  /// - POST data may only contain a single element of type PDE_TYPE_BYTES.
  /// - The |request_context| parameter must be NULL.
  /// 
  /// The |request| object will be marked as read-only after calling this method.
  #[allow(non_snake_case)]
  pub fn create(request: crate::include::CefRequest, client: crate::include::CefURLRequestClient, request_context: Option<crate::include::CefRequestContext>, ) -> Option<crate::include::CefURLRequest> {
    unsafe {
      let ret = cef_sys::cef_urlrequest_create(crate::include::CefRequest::to_cef_own(request),crate::include::CefURLRequestClient::to_cef_own(client),request_context.map_or(std::ptr::null_mut(), |o| crate::include::CefRequestContext::to_cef_own(o)),);
      crate::include::CefURLRequest::from_cef_own(ret)
    }
  }
  /// Returns the request object used to create this URL request. The returned
  /// object is read-only and should not be modified.
  pub fn get_request(&mut self) -> Option<crate::include::CefRequest> {
    unsafe {
      let ret = match self.raw.as_ref().get_request {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefRequest::from_cef_own(ret)
    }
  }
  /// Returns the client.
  pub fn get_client(&mut self) -> Option<crate::include::CefURLRequestClient> {
    unsafe {
      let ret = match self.raw.as_ref().get_client {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefURLRequestClient::from_cef_own(ret)
    }
  }
  /// Returns the request status.
  pub fn get_request_status(&mut self) -> crate::include::internal::CefUrlrequestStatus {
    unsafe {
      let ret = match self.raw.as_ref().get_request_status {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret.into()
    }
  }
  /// Returns the request error if status is UR_CANCELED or UR_FAILED, or 0
  /// otherwise.
  pub fn get_request_error(&mut self) -> crate::include::internal::CefErrorcode {
    unsafe {
      let ret = match self.raw.as_ref().get_request_error {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret.into()
    }
  }
  /// Returns the response, or NULL if no response information is available.
  /// Response information will only be available after the upload has completed.
  /// The returned object is read-only and should not be modified.
  pub fn get_response(&mut self) -> Option<crate::include::CefResponse> {
    unsafe {
      let ret = match self.raw.as_ref().get_response {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefResponse::from_cef_own(ret)
    }
  }
  /// Returns true if the response body was served from the cache. This includes
  /// responses for which revalidation was required.
  pub fn response_was_cached(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().response_was_cached {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Cancel the request.
  pub fn cancel(&mut self) -> () {
    unsafe {
      let ret = match self.raw.as_ref().cancel {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
}
/// Interface that should be implemented by the CefURLRequest client. The
/// methods of this class will be called on the same thread that created the
/// request unless otherwise documented.
#[allow(non_snake_case)]
#[allow(unused_variables)]
pub trait URLRequestClient {
  /// Notifies the client that the request has completed. Use the
  /// CefURLRequest::GetRequestStatus method to determine if the request was
  /// successful or not.
  fn on_request_complete(&mut self, request: crate::include::CefURLRequest) -> () { Default::default() }
  /// Notifies the client of upload progress. |current| denotes the number of
  /// bytes sent so far and |total| is the total size of uploading data (or -1 if
  /// chunked upload is enabled). This method will only be called if the
  /// UR_FLAG_REPORT_UPLOAD_PROGRESS flag is set on the request.
  fn on_upload_progress(&mut self, request: crate::include::CefURLRequest, current: i64, total: i64) -> () { Default::default() }
  /// Notifies the client of download progress. |current| denotes the number of
  /// bytes received up to the call and |total| is the expected total size of the
  /// response (or -1 if not determined).
  fn on_download_progress(&mut self, request: crate::include::CefURLRequest, current: i64, total: i64) -> () { Default::default() }
  /// Called on the IO thread when the browser needs credentials from the user.
  /// |isProxy| indicates whether the host is a proxy server. |host| contains the
  /// hostname and |port| contains the port number. Return true to continue the
  /// request and call CefAuthCallback::Continue() when the authentication
  /// information is available. If the request has an associated browser/frame
  /// then returning false will result in a call to GetAuthCredentials on the
  /// CefRequestHandler associated with that browser, if any. Otherwise,
  /// returning false will cancel the request immediately. This method will only
  /// be called for requests initiated from the browser process.
  fn get_auth_credentials(&mut self, isProxy: bool, host: &crate::include::internal::CefString, port: i32, realm: Option<&crate::include::internal::CefString>, scheme: &crate::include::internal::CefString, callback: crate::include::CefAuthCallback) -> bool { Default::default() }
}
define_refcounted!(URLRequestClient, CefURLRequestClient, cef_urlrequest_client_t, on_request_complete: cef_urlrequest_client_t_on_request_complete,on_upload_progress: cef_urlrequest_client_t_on_upload_progress,on_download_progress: cef_urlrequest_client_t_on_download_progress,get_auth_credentials: cef_urlrequest_client_t_get_auth_credentials,);
#[allow(non_snake_case)]
unsafe extern "C" fn cef_urlrequest_client_t_on_request_complete(_self: *mut cef_sys::cef_urlrequest_client_t, request: *mut cef_sys::cef_urlrequest_t) -> () {
  let ret = CefURLRequestClient::from_cef(_self, true).get().on_request_complete(crate::include::CefURLRequest::from_cef_own(request).unwrap(),);
  ret
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_urlrequest_client_t_on_upload_progress(_self: *mut cef_sys::cef_urlrequest_client_t, request: *mut cef_sys::cef_urlrequest_t, current: i64, total: i64) -> () {
  let ret = CefURLRequestClient::from_cef(_self, true).get().on_upload_progress(crate::include::CefURLRequest::from_cef_own(request).unwrap(),current,total,);
  ret
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_urlrequest_client_t_on_download_progress(_self: *mut cef_sys::cef_urlrequest_client_t, request: *mut cef_sys::cef_urlrequest_t, current: i64, total: i64) -> () {
  let ret = CefURLRequestClient::from_cef(_self, true).get().on_download_progress(crate::include::CefURLRequest::from_cef_own(request).unwrap(),current,total,);
  ret
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_urlrequest_client_t_get_auth_credentials(_self: *mut cef_sys::cef_urlrequest_client_t, isProxy: i32, host: *const cef_sys::cef_string_t, port: i32, realm: *const cef_sys::cef_string_t, scheme: *const cef_sys::cef_string_t, callback: *mut cef_sys::cef_auth_callback_t) -> i32 {
  let ret = CefURLRequestClient::from_cef(_self, true).get().get_auth_credentials(if isProxy == 0 { false } else { true },&crate::include::internal::CefString::from_cef(host).unwrap(),port,match &crate::include::internal::CefString::from_cef(realm) { Some(ref x) => Some(x), None => None },&crate::include::internal::CefString::from_cef(scheme).unwrap(),crate::include::CefAuthCallback::from_cef_own(callback).unwrap(),);
  if ret { 1 } else { 0 }
}
