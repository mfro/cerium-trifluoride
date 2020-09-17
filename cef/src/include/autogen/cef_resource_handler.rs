pub type CefResourceSkipCallback = crate::include::base::CefProxy<cef_sys::cef_resource_skip_callback_t>;
#[allow(non_snake_case)]
impl CefResourceSkipCallback {
  /// Callback for asynchronous continuation of Skip(). If |bytes_skipped| > 0
  /// then either Skip() will be called again until the requested number of
  /// bytes have been skipped or the request will proceed. If |bytes_skipped|
  /// <= 0 the request will fail with ERR_REQUEST_RANGE_NOT_SATISFIABLE.
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
  /// Callback for asynchronous continuation of Read(). If |bytes_read| == 0
  /// the response will be considered complete. If |bytes_read| > 0 then Read()
  /// will be called again until the request is complete (based on either the
  /// result or the expected content length). If |bytes_read| < 0 then the
  /// request will fail and the |bytes_read| value will be treated as the error
  /// code.
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
#[allow(unused_variables)]
pub trait ResourceHandler {
  /// Open the response stream. To handle the request immediately set
  /// |handle_request| to true and return true. To decide at a later time set
  /// |handle_request| to false, return true, and execute |callback| to continue
  /// or cancel the request. To cancel the request immediately set
  /// |handle_request| to true and return false. This method will be called in
  /// sequence but not from a dedicated thread. For backwards compatibility set
  /// |handle_request| to false and return false and the ProcessRequest method
  /// will be called.
  fn open(&mut self, request: crate::include::CefRequest, handle_request: &mut bool, callback: crate::include::CefCallback) -> bool { Default::default() }
  /// Begin processing the request. To handle the request return true and call
  /// CefCallback::Continue() once the response header information is available
  /// (CefCallback::Continue() can also be called from inside this method if
  /// header information is available immediately). To cancel the request return
  /// false.
  /// 
  /// WARNING: This method is deprecated. Use Open instead.
  fn process_request(&mut self, request: crate::include::CefRequest, callback: crate::include::CefCallback) -> bool { Default::default() }
  /// Retrieve response header information. If the response length is not known
  /// set |response_length| to -1 and ReadResponse() will be called until it
  /// returns false. If the response length is known set |response_length|
  /// to a positive value and ReadResponse() will be called until it returns
  /// false or the specified number of bytes have been read. Use the |response|
  /// object to set the mime type, http status code and other optional header
  /// values. To redirect the request to a new URL set |redirectUrl| to the new
  /// URL. |redirectUrl| can be either a relative or fully qualified URL.
  /// It is also possible to set |response| to a redirect http status code
  /// and pass the new URL via a Location header. Likewise with |redirectUrl| it
  /// is valid to set a relative or fully qualified URL as the Location header
  /// value. If an error occured while setting up the request you can call
  /// SetError() on |response| to indicate the error condition.
  fn get_response_headers(&mut self, response: crate::include::CefResponse, response_length: &mut i64, redirectUrl: &mut crate::include::internal::CefString) -> () { Default::default() }
  /// Skip response data when requested by a Range header. Skip over and discard
  /// |bytes_to_skip| bytes of response data. If data is available immediately
  /// set |bytes_skipped| to the number of bytes skipped and return true. To
  /// read the data at a later time set |bytes_skipped| to 0, return true and
  /// execute |callback| when the data is available. To indicate failure set
  /// |bytes_skipped| to < 0 (e.g. -2 for ERR_FAILED) and return false. This
  /// method will be called in sequence but not from a dedicated thread.
  fn skip(&mut self, bytes_to_skip: i64, bytes_skipped: &mut i64, callback: crate::include::CefResourceSkipCallback) -> bool { Default::default() }
  /// Read response data. If data is available immediately copy up to
  /// |bytes_to_read| bytes into |data_out|, set |bytes_read| to the number of
  /// bytes copied, and return true. To read the data at a later time keep a
  /// pointer to |data_out|, set |bytes_read| to 0, return true and execute
  /// |callback| when the data is available (|data_out| will remain valid until
  /// the callback is executed). To indicate response completion set |bytes_read|
  /// to 0 and return false. To indicate failure set |bytes_read| to < 0 (e.g. -2
  /// for ERR_FAILED) and return false. This method will be called in sequence
  /// but not from a dedicated thread. For backwards compatibility set
  /// |bytes_read| to -1 and return false and the ReadResponse method will be
  /// called.
  fn read(&mut self, data_out: &mut std::os::raw::c_void, bytes_to_read: i32, bytes_read: &mut i32, callback: crate::include::CefResourceReadCallback) -> bool { Default::default() }
  /// Read response data. If data is available immediately copy up to
  /// |bytes_to_read| bytes into |data_out|, set |bytes_read| to the number of
  /// bytes copied, and return true. To read the data at a later time set
  /// |bytes_read| to 0, return true and call CefCallback::Continue() when the
  /// data is available. To indicate response completion return false.
  /// 
  /// WARNING: This method is deprecated. Use Skip and Read instead.
  fn read_response(&mut self, data_out: &mut std::os::raw::c_void, bytes_to_read: i32, bytes_read: &mut i32, callback: crate::include::CefCallback) -> bool { Default::default() }
  /// Request processing has been canceled.
  fn cancel(&mut self) -> () { Default::default() }
}
define_refcounted!(ResourceHandler, CefResourceHandler, cef_resource_handler_t, open: cef_resource_handler_t_open,process_request: cef_resource_handler_t_process_request,get_response_headers: cef_resource_handler_t_get_response_headers,skip: cef_resource_handler_t_skip,read: cef_resource_handler_t_read,read_response: cef_resource_handler_t_read_response,cancel: cef_resource_handler_t_cancel,);
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
