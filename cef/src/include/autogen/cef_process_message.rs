pub type CefProcessMessage = crate::include::base::CefProxy<cef_sys::cef_process_message_t>;
#[allow(non_snake_case)]
impl CefProcessMessage {
  /// Create a new CefProcessMessage object with the specified name.
  #[allow(non_snake_case)]
  pub fn create(name: &crate::include::internal::CefString, ) -> Option<crate::include::CefProcessMessage> {
    unsafe {
      let ret = cef_sys::cef_process_message_create(crate::include::internal::IntoCef::into_cef(name),);
      crate::include::CefProcessMessage::from_cef_own(ret)
    }
  }
  /// Returns true if this object is valid. Do not call any other methods if this
  /// function returns false.
  pub fn is_valid(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_valid {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns true if the values of this object are read-only. Some APIs may
  /// expose read-only objects.
  pub fn is_read_only(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_read_only {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns a writable copy of this object.
  pub fn copy(&mut self) -> Option<crate::include::CefProcessMessage> {
    unsafe {
      let ret = match self.raw.as_ref().copy {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefProcessMessage::from_cef_own(ret)
    }
  }
  /// Returns the message name.
  pub fn get_name(&mut self) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_name {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  /// Returns the list of arguments.
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
