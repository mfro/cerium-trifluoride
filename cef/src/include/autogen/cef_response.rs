pub type CefResponse = crate::include::base::CefProxy<cef_sys::cef_response_t>;
#[allow(non_snake_case)]
impl CefResponse {
  pub fn is_read_only(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_read_only {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn get_error(&mut self) -> crate::include::internal::CefErrorcode {
    unsafe {
      let ret = match self.raw.as_ref().get_error {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret.into()
    }
  }
  pub fn set_error(&mut self, error: crate::include::internal::CefErrorcode) -> () {
    unsafe {
      let ret = match self.raw.as_ref().set_error {
        Some(f) => f(self.raw.as_ptr(),error.into(),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn get_status(&mut self) -> i32 {
    unsafe {
      let ret = match self.raw.as_ref().get_status {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn set_status(&mut self, status: i32) -> () {
    unsafe {
      let ret = match self.raw.as_ref().set_status {
        Some(f) => f(self.raw.as_ptr(),status,),
        None => panic!(),
      };
      ret
    }
  }
  pub fn get_status_text(&mut self) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_status_text {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  pub fn set_status_text(&mut self, statusText: Option<&crate::include::internal::CefString>) -> () {
    unsafe {
      let ret = match self.raw.as_ref().set_status_text {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(statusText),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn get_mime_type(&mut self) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_mime_type {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  pub fn set_mime_type(&mut self, mimeType: Option<&crate::include::internal::CefString>) -> () {
    unsafe {
      let ret = match self.raw.as_ref().set_mime_type {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(mimeType),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn get_charset(&mut self) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_charset {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  pub fn set_charset(&mut self, charset: Option<&crate::include::internal::CefString>) -> () {
    unsafe {
      let ret = match self.raw.as_ref().set_charset {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(charset),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn get_header_by_name(&mut self, name: &crate::include::internal::CefString) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_header_by_name {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(name),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  pub fn set_header_by_name(&mut self, name: &crate::include::internal::CefString, value: Option<&crate::include::internal::CefString>, overwrite: bool) -> () {
    unsafe {
      let ret = match self.raw.as_ref().set_header_by_name {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(name),crate::include::internal::IntoCef::into_cef(value),if overwrite { 1 } else { 0 },),
        None => panic!(),
      };
      ret
    }
  }
  pub fn get_url(&mut self) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_url {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  pub fn set_url(&mut self, url: Option<&crate::include::internal::CefString>) -> () {
    unsafe {
      let ret = match self.raw.as_ref().set_url {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(url),),
        None => panic!(),
      };
      ret
    }
  }
}
