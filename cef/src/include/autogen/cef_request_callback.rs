pub type CefRequestCallback = crate::include::refcounting::CefProxy<cef_sys::cef_request_callback_t>;
#[allow(non_snake_case)]
impl CefRequestCallback {
  /// Continue the url request. If |allow| is true the request will be continued.
  /// Otherwise, the request will be canceled.
  pub fn cont(&mut self, allow: bool) -> () {
    unsafe {
      let ret = match self.raw.as_ref().cont {
        Some(f) => f(self.raw.as_ptr(),if allow { 1 } else { 0 },),
        None => panic!(),
      };
      ret
    }
  }
  /// Cancel the url request.
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
