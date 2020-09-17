pub type CefResponse = crate::include::refcounting::CefProxy<cef_sys::cef_response_t>;
#[allow(non_snake_case)]
impl CefResponse {
  /// Create a new CefResponse object.
  #[allow(non_snake_case)]
  pub fn create() -> Option<crate::include::CefResponse> {
    unsafe {
      let ret = cef_sys::cef_response_create();
      crate::include::CefResponse::from_cef_own(ret)
    }
  }
  /// Returns true if this object is read-only.
  pub fn is_read_only(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_read_only {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Get the response error code. Returns ERR_NONE if there was no error.
  pub fn get_error(&mut self) -> crate::include::internal::CefErrorcode {
    unsafe {
      let ret = match self.raw.as_ref().get_error {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret.into()
    }
  }
  /// Set the response error code. This can be used by custom scheme handlers
  /// to return errors during initial request processing.
  pub fn set_error(&mut self, error: crate::include::internal::CefErrorcode) -> () {
    unsafe {
      let ret = match self.raw.as_ref().set_error {
        Some(f) => f(self.raw.as_ptr(),error.into(),),
        None => panic!(),
      };
      ret
    }
  }
  /// Get the response status code.
  pub fn get_status(&mut self) -> i32 {
    unsafe {
      let ret = match self.raw.as_ref().get_status {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  /// Set the response status code.
  pub fn set_status(&mut self, status: i32) -> () {
    unsafe {
      let ret = match self.raw.as_ref().set_status {
        Some(f) => f(self.raw.as_ptr(),status,),
        None => panic!(),
      };
      ret
    }
  }
  /// Get the response status text.
  pub fn get_status_text(&mut self) -> crate::include::internal::CefStringUserFree {
    unsafe {
      let ret = match self.raw.as_ref().get_status_text {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefStringUserFree::from_cef(ret).unwrap()
    }
  }
  /// Set the response status text.
  pub fn set_status_text(&mut self, statusText: Option<&crate::include::internal::CefString>) -> () {
    unsafe {
      let ret = match self.raw.as_ref().set_status_text {
        Some(f) => f(self.raw.as_ptr(),match statusText { Some(statusText) => statusText as *const _ as *const _, None => std::ptr::null_mut() },),
        None => panic!(),
      };
      ret
    }
  }
  /// Get the response mime type.
  pub fn get_mime_type(&mut self) -> crate::include::internal::CefStringUserFree {
    unsafe {
      let ret = match self.raw.as_ref().get_mime_type {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefStringUserFree::from_cef(ret).unwrap()
    }
  }
  /// Set the response mime type.
  pub fn set_mime_type(&mut self, mimeType: Option<&crate::include::internal::CefString>) -> () {
    unsafe {
      let ret = match self.raw.as_ref().set_mime_type {
        Some(f) => f(self.raw.as_ptr(),match mimeType { Some(mimeType) => mimeType as *const _ as *const _, None => std::ptr::null_mut() },),
        None => panic!(),
      };
      ret
    }
  }
  /// Get the response charset.
  pub fn get_charset(&mut self) -> crate::include::internal::CefStringUserFree {
    unsafe {
      let ret = match self.raw.as_ref().get_charset {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefStringUserFree::from_cef(ret).unwrap()
    }
  }
  /// Set the response charset.
  pub fn set_charset(&mut self, charset: Option<&crate::include::internal::CefString>) -> () {
    unsafe {
      let ret = match self.raw.as_ref().set_charset {
        Some(f) => f(self.raw.as_ptr(),match charset { Some(charset) => charset as *const _ as *const _, None => std::ptr::null_mut() },),
        None => panic!(),
      };
      ret
    }
  }
  /// Get the value for the specified response header field.
  pub fn get_header_by_name(&mut self, name: &crate::include::internal::CefString) -> crate::include::internal::CefStringUserFree {
    unsafe {
      let ret = match self.raw.as_ref().get_header_by_name {
        Some(f) => f(self.raw.as_ptr(),name as *const _ as *const _,),
        None => panic!(),
      };
      crate::include::internal::CefStringUserFree::from_cef(ret).unwrap()
    }
  }
  /// Set the header |name| to |value|. If |overwrite| is true any existing
  /// values will be replaced with the new value. If |overwrite| is false any
  /// existing values will not be overwritten.
  pub fn set_header_by_name(&mut self, name: &crate::include::internal::CefString, value: Option<&crate::include::internal::CefString>, overwrite: bool) -> () {
    unsafe {
      let ret = match self.raw.as_ref().set_header_by_name {
        Some(f) => f(self.raw.as_ptr(),name as *const _ as *const _,match value { Some(value) => value as *const _ as *const _, None => std::ptr::null_mut() },if overwrite { 1 } else { 0 },),
        None => panic!(),
      };
      ret
    }
  }
  /// Get the resolved URL after redirects or changed as a result of HSTS.
  pub fn get_url(&mut self) -> crate::include::internal::CefStringUserFree {
    unsafe {
      let ret = match self.raw.as_ref().get_url {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefStringUserFree::from_cef(ret).unwrap()
    }
  }
  /// Set the resolved URL after redirects or changed as a result of HSTS.
  pub fn set_url(&mut self, url: Option<&crate::include::internal::CefString>) -> () {
    unsafe {
      let ret = match self.raw.as_ref().set_url {
        Some(f) => f(self.raw.as_ptr(),match url { Some(url) => url as *const _ as *const _, None => std::ptr::null_mut() },),
        None => panic!(),
      };
      ret
    }
  }
}
