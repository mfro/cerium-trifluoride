pub type CefRequestCallback = crate::include::base::CefProxy<cef_sys::cef_request_callback_t>;
#[allow(non_snake_case)]
impl CefRequestCallback {
  pub fn cont(&mut self, allow: bool) -> () {
    unsafe {
      let ret = match self.raw.as_ref().cont {
        Some(f) => f(self.raw.as_ptr(),if allow { 1 } else { 0 },),
        None => panic!(),
      };
      ret
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
