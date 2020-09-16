pub type CefResourceSkipCallback = crate::include::base::CefProxy<cef_sys::cef_resource_skip_callback_t>;
#[allow(non_snake_case)]
impl CefResourceSkipCallback {
  pub fn cont(&mut self, bytes_skipped: i64) -> () {
    unsafe {
      let ret = match self.raw.as_ref().cont {
        Some(f) => f(self.raw.as_ptr(),bytes_skipped,),
        None => panic!(),
      };
      ret
    }
  }
}
pub type CefResourceReadCallback = crate::include::base::CefProxy<cef_sys::cef_resource_read_callback_t>;
#[allow(non_snake_case)]
impl CefResourceReadCallback {
  pub fn cont(&mut self, bytes_read: i32) -> () {
    unsafe {
      let ret = match self.raw.as_ref().cont {
        Some(f) => f(self.raw.as_ptr(),bytes_read,),
        None => panic!(),
      };
      ret
    }
  }
}
/// Class used to implement a custom request handler interface. The methods of
/// this class will be called on the IO thread unless otherwise indicated.
#[allow(non_snake_case)]
pub trait ResourceHandler {
  fn open(&mut self, request: crate::include::CefRequest, handle_request: &mut bool, callback: crate::include::CefCallback) -> bool { Default::default() }
  fn process_request(&mut self, request: crate::include::CefRequest, callback: crate::include::CefCallback) -> bool { Default::default() }
  fn get_response_headers(&mut self, response: crate::include::CefResponse, response_length: &mut i64, redirectUrl: &mut crate::include::internal::CefString) -> () { Default::default() }
  fn skip(&mut self, bytes_to_skip: i64, bytes_skipped: &mut i64, callback: crate::include::CefResourceSkipCallback) -> bool { Default::default() }
  fn read(&mut self, data_out: &mut std::os::raw::c_void, bytes_to_read: i32, bytes_read: &mut i32, callback: crate::include::CefResourceReadCallback) -> bool { Default::default() }
  fn read_response(&mut self, data_out: &mut std::os::raw::c_void, bytes_to_read: i32, bytes_read: &mut i32, callback: crate::include::CefCallback) -> bool { Default::default() }
  fn cancel(&mut self) -> () { Default::default() }
}
define_refcounted!(ResourceHandler, resource_handler, open,process_request,get_response_headers,skip,read,read_response,cancel,);
#[allow(non_snake_case)]
unsafe extern "C" fn cef_resource_handler_t_open(_self: *mut cef_sys::cef_resource_handler_t, request: *mut cef_sys::cef_request_t, handle_request: *mut i32, callback: *mut cef_sys::cef_callback_t) -> i32 {
  let mut handle_request__tmp = if *handle_request == 0 { false } else { true };
  let ret = CefResourceHandler::from_cef(_self, true).get().open(crate::include::CefRequest::from_cef_own(request).unwrap(),&mut handle_request__tmp,crate::include::CefCallback::from_cef_own(callback).unwrap(),);
  *handle_request = if handle_request__tmp { 1 } else { 0 };
  if ret { 1 } else { 0 }
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_resource_handler_t_process_request(_self: *mut cef_sys::cef_resource_handler_t, request: *mut cef_sys::cef_request_t, callback: *mut cef_sys::cef_callback_t) -> i32 {
  let ret = CefResourceHandler::from_cef(_self, true).get().process_request(crate::include::CefRequest::from_cef_own(request).unwrap(),crate::include::CefCallback::from_cef_own(callback).unwrap(),);
  if ret { 1 } else { 0 }
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_resource_handler_t_get_response_headers(_self: *mut cef_sys::cef_resource_handler_t, response: *mut cef_sys::cef_response_t, response_length: *mut i64, redirectUrl: *mut cef_sys::cef_string_t) -> () {
  let ret = CefResourceHandler::from_cef(_self, true).get().get_response_headers(crate::include::CefResponse::from_cef_own(response).unwrap(),&mut *response_length,&mut crate::include::internal::CefString::from_cef(redirectUrl).unwrap(),);
  ret
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_resource_handler_t_skip(_self: *mut cef_sys::cef_resource_handler_t, bytes_to_skip: i64, bytes_skipped: *mut i64, callback: *mut cef_sys::cef_resource_skip_callback_t) -> i32 {
  let ret = CefResourceHandler::from_cef(_self, true).get().skip(bytes_to_skip,&mut *bytes_skipped,crate::include::CefResourceSkipCallback::from_cef_own(callback).unwrap(),);
  if ret { 1 } else { 0 }
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_resource_handler_t_read(_self: *mut cef_sys::cef_resource_handler_t, data_out: *mut std::os::raw::c_void, bytes_to_read: i32, bytes_read: *mut i32, callback: *mut cef_sys::cef_resource_read_callback_t) -> i32 {
  let ret = CefResourceHandler::from_cef(_self, true).get().read(&mut *data_out,bytes_to_read,&mut *bytes_read,crate::include::CefResourceReadCallback::from_cef_own(callback).unwrap(),);
  if ret { 1 } else { 0 }
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_resource_handler_t_read_response(_self: *mut cef_sys::cef_resource_handler_t, data_out: *mut std::os::raw::c_void, bytes_to_read: i32, bytes_read: *mut i32, callback: *mut cef_sys::cef_callback_t) -> i32 {
  let ret = CefResourceHandler::from_cef(_self, true).get().read_response(&mut *data_out,bytes_to_read,&mut *bytes_read,crate::include::CefCallback::from_cef_own(callback).unwrap(),);
  if ret { 1 } else { 0 }
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_resource_handler_t_cancel(_self: *mut cef_sys::cef_resource_handler_t) -> () {
  let ret = CefResourceHandler::from_cef(_self, true).get().cancel();
  ret
}
