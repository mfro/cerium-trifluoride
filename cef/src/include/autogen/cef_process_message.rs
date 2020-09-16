pub type CefProcessMessage = crate::include::base::CefProxy<cef_sys::cef_process_message_t>;
#[allow(non_snake_case)]
impl CefProcessMessage {
  pub fn is_valid(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_valid {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn is_read_only(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_read_only {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn copy(&mut self) -> Option<crate::include::CefProcessMessage> {
    unsafe {
      let ret = match self.raw.as_ref().copy {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefProcessMessage::from_cef_own(ret)
    }
  }
  pub fn get_name(&mut self) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_name {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  pub fn get_argument_list(&mut self) -> Option<crate::include::CefListValue> {
    unsafe {
      let ret = match self.raw.as_ref().get_argument_list {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefListValue::from_cef_own(ret)
    }
  }
}
