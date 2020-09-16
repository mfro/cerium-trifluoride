pub type CefURLRequest = crate::include::base::CefProxy<cef_sys::cef_urlrequest_t>;
#[allow(non_snake_case)]
impl CefURLRequest {
  pub fn get_request(&mut self) -> Option<crate::include::CefRequest> {
    unsafe {
      let ret = match self.raw.as_ref().get_request {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefRequest::from_cef_own(ret)
    }
  }
  pub fn get_client(&mut self) -> Option<crate::include::CefURLRequestClient> {
    unsafe {
      let ret = match self.raw.as_ref().get_client {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefURLRequestClient::from_cef_own(ret)
    }
  }
  pub fn get_request_status(&mut self) -> crate::include::internal::CefUrlrequestStatus {
    unsafe {
      let ret = match self.raw.as_ref().get_request_status {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret.into()
    }
  }
  pub fn get_request_error(&mut self) -> crate::include::internal::CefErrorcode {
    unsafe {
      let ret = match self.raw.as_ref().get_request_error {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret.into()
    }
  }
  pub fn get_response(&mut self) -> Option<crate::include::CefResponse> {
    unsafe {
      let ret = match self.raw.as_ref().get_response {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefResponse::from_cef_own(ret)
    }
  }
  pub fn response_was_cached(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().response_was_cached {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
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
pub trait URLRequestClient {
  fn on_request_complete(&mut self, request: crate::include::CefURLRequest) -> () { Default::default() }
  fn on_upload_progress(&mut self, request: crate::include::CefURLRequest, current: i64, total: i64) -> () { Default::default() }
  fn on_download_progress(&mut self, request: crate::include::CefURLRequest, current: i64, total: i64) -> () { Default::default() }
  fn get_auth_credentials(&mut self, isProxy: bool, host: &crate::include::internal::CefString, port: i32, realm: Option<&crate::include::internal::CefString>, scheme: &crate::include::internal::CefString, callback: crate::include::CefAuthCallback) -> bool { Default::default() }
}
define_refcounted!(URLRequestClient, urlrequest_client, on_request_complete,on_upload_progress,on_download_progress,get_auth_credentials,);
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
