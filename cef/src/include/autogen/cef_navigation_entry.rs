pub type CefNavigationEntry = crate::include::base::CefProxy<cef_sys::cef_navigation_entry_t>;
#[allow(non_snake_case)]
impl CefNavigationEntry {
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
  /// Returns the actual URL of the page. For some pages this may be data: URL or
  /// similar. Use GetDisplayURL() to return a display-friendly version.
  pub fn get_url(&mut self) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_url {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  /// Returns a display-friendly version of the URL.
  pub fn get_display_url(&mut self) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_display_url {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  /// Returns the original URL that was entered by the user before any redirects.
  pub fn get_original_url(&mut self) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_original_url {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  /// Returns the title set by the page. This value may be empty.
  pub fn get_title(&mut self) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_title {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  /// Returns the transition type which indicates what the user did to move to
  /// this page from the previous page.
  pub fn get_transition_type(&mut self) -> crate::include::internal::CefTransitionType {
    unsafe {
      let ret = match self.raw.as_ref().get_transition_type {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret.into()
    }
  }
  /// Returns true if this navigation includes post data.
  pub fn has_post_data(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().has_post_data {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns the time for the last known successful navigation completion. A
  /// navigation may be completed more than once if the page is reloaded. May be
  /// 0 if the navigation has not yet completed.
  pub fn get_completion_time(&mut self) -> crate::include::internal::CefTime {
    unsafe {
      let ret = match self.raw.as_ref().get_completion_time {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret.into()
    }
  }
  /// Returns the HTTP status code for the last known successful navigation
  /// response. May be 0 if the response has not yet been received or if the
  /// navigation has not yet completed.
  pub fn get_http_status_code(&mut self) -> i32 {
    unsafe {
      let ret = match self.raw.as_ref().get_http_status_code {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  /// Returns the SSL information for this navigation entry.
  pub fn get_sslstatus(&mut self) -> Option<crate::include::CefSSLStatus> {
    unsafe {
      let ret = match self.raw.as_ref().get_sslstatus {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefSSLStatus::from_cef_own(ret)
    }
  }
}
