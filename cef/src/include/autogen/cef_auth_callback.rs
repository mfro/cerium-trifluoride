pub type CefAuthCallback = crate::include::base::CefProxy<cef_sys::cef_auth_callback_t>;
#[allow(non_snake_case)]
impl CefAuthCallback {
  pub fn cont(&mut self, username: &crate::include::internal::CefString, password: &crate::include::internal::CefString) -> () {
    unsafe {
      let ret = match self.raw.as_ref().cont {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(username),crate::include::internal::IntoCef::into_cef(password),),
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
