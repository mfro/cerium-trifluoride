pub type CefAuthCallback = crate::include::refcounting::CefProxy<cef_sys::cef_auth_callback_t>;
#[allow(non_snake_case)]
impl CefAuthCallback {
  /// Continue the authentication request.
  pub fn cont(&mut self, username: Option<&crate::include::internal::CefString>, password: Option<&crate::include::internal::CefString>) -> () {
    unsafe {
      let ret = match self.raw.as_ref().cont {
        Some(f) => f(self.raw.as_ptr(),match username { Some(username) => username as *const _ as *const _, None => std::ptr::null_mut() },match password { Some(password) => password as *const _ as *const _, None => std::ptr::null_mut() },),
        None => panic!(),
      };
      ret
    }
  }
  /// Cancel the authentication request.
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
