pub type CefRequest = crate::include::refcounting::CefProxy<cef_sys::cef_request_t>;
#[allow(non_snake_case)]
impl CefRequest {
  /// Create a new CefRequest object.
  #[allow(non_snake_case)]
  pub fn create() -> Option<crate::include::CefRequest> {
    unsafe {
      let ret = cef_sys::cef_request_create();
      crate::include::CefRequest::from_cef_own(ret)
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
  /// Get the fully qualified URL.
  pub fn get_url(&mut self) -> crate::include::internal::CefStringUserFree {
    unsafe {
      let ret = match self.raw.as_ref().get_url {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefStringUserFree::from_cef(ret).unwrap()
    }
  }
  /// Set the fully qualified URL.
  pub fn set_url(&mut self, url: &crate::include::internal::CefString) -> () {
    unsafe {
      let ret = match self.raw.as_ref().set_url {
        Some(f) => f(self.raw.as_ptr(),url as *const _ as *const _,),
        None => panic!(),
      };
      ret
    }
  }
  /// Get the request method type. The value will default to POST if post data
  /// is provided and GET otherwise.
  pub fn get_method(&mut self) -> crate::include::internal::CefStringUserFree {
    unsafe {
      let ret = match self.raw.as_ref().get_method {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefStringUserFree::from_cef(ret).unwrap()
    }
  }
  /// Set the request method type.
  pub fn set_method(&mut self, method: &crate::include::internal::CefString) -> () {
    unsafe {
      let ret = match self.raw.as_ref().set_method {
        Some(f) => f(self.raw.as_ptr(),method as *const _ as *const _,),
        None => panic!(),
      };
      ret
    }
  }
  /// Set the referrer URL and policy. If non-empty the referrer URL must be
  /// fully qualified with an HTTP or HTTPS scheme component. Any username,
  /// password or ref component will be removed.
  pub fn set_referrer(&mut self, referrer_url: Option<&crate::include::internal::CefString>, policy: crate::include::internal::CefReferrerPolicy) -> () {
    unsafe {
      let ret = match self.raw.as_ref().set_referrer {
        Some(f) => f(self.raw.as_ptr(),match referrer_url { Some(referrer_url) => referrer_url as *const _ as *const _, None => std::ptr::null_mut() },policy.into(),),
        None => panic!(),
      };
      ret
    }
  }
  /// Get the referrer URL.
  pub fn get_referrer_url(&mut self) -> crate::include::internal::CefStringUserFree {
    unsafe {
      let ret = match self.raw.as_ref().get_referrer_url {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefStringUserFree::from_cef(ret).unwrap()
    }
  }
  /// Get the referrer policy.
  pub fn get_referrer_policy(&mut self) -> crate::include::internal::CefReferrerPolicy {
    unsafe {
      let ret = match self.raw.as_ref().get_referrer_policy {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret.into()
    }
  }
  /// Get the post data.
  pub fn get_post_data(&mut self) -> Option<crate::include::CefPostData> {
    unsafe {
      let ret = match self.raw.as_ref().get_post_data {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefPostData::from_cef_own(ret)
    }
  }
  /// Set the post data.
  pub fn set_post_data(&mut self, postData: crate::include::CefPostData) -> () {
    unsafe {
      let ret = match self.raw.as_ref().set_post_data {
        Some(f) => f(self.raw.as_ptr(),crate::include::CefPostData::to_cef_own(postData),),
        None => panic!(),
      };
      ret
    }
  }
  /// Returns the first header value for |name| or an empty string if not found.
  /// Will not return the Referer value if any. Use GetHeaderMap instead if
  /// |name| might have multiple values.
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
  /// existing values will not be overwritten. The Referer value cannot be set
  /// using this method.
  pub fn set_header_by_name(&mut self, name: &crate::include::internal::CefString, value: Option<&crate::include::internal::CefString>, overwrite: bool) -> () {
    unsafe {
      let ret = match self.raw.as_ref().set_header_by_name {
        Some(f) => f(self.raw.as_ptr(),name as *const _ as *const _,match value { Some(value) => value as *const _ as *const _, None => std::ptr::null_mut() },if overwrite { 1 } else { 0 },),
        None => panic!(),
      };
      ret
    }
  }
  /// Get the flags used in combination with CefURLRequest. See
  /// cef_urlrequest_flags_t for supported values.
  pub fn get_flags(&mut self) -> i32 {
    unsafe {
      let ret = match self.raw.as_ref().get_flags {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  /// Set the flags used in combination with CefURLRequest.  See
  /// cef_urlrequest_flags_t for supported values.
  pub fn set_flags(&mut self, flags: i32) -> () {
    unsafe {
      let ret = match self.raw.as_ref().set_flags {
        Some(f) => f(self.raw.as_ptr(),flags,),
        None => panic!(),
      };
      ret
    }
  }
  /// Get the URL to the first party for cookies used in combination with
  /// CefURLRequest.
  pub fn get_first_party_for_cookies(&mut self) -> crate::include::internal::CefStringUserFree {
    unsafe {
      let ret = match self.raw.as_ref().get_first_party_for_cookies {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefStringUserFree::from_cef(ret).unwrap()
    }
  }
  /// Set the URL to the first party for cookies used in combination with
  /// CefURLRequest.
  pub fn set_first_party_for_cookies(&mut self, url: Option<&crate::include::internal::CefString>) -> () {
    unsafe {
      let ret = match self.raw.as_ref().set_first_party_for_cookies {
        Some(f) => f(self.raw.as_ptr(),match url { Some(url) => url as *const _ as *const _, None => std::ptr::null_mut() },),
        None => panic!(),
      };
      ret
    }
  }
  /// Get the resource type for this request. Only available in the browser
  /// process.
  pub fn get_resource_type(&mut self) -> crate::include::internal::CefResourceType {
    unsafe {
      let ret = match self.raw.as_ref().get_resource_type {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret.into()
    }
  }
  /// Get the transition type for this request. Only available in the browser
  /// process and only applies to requests that represent a main frame or
  /// sub-frame navigation.
  pub fn get_transition_type(&mut self) -> crate::include::internal::CefTransitionType {
    unsafe {
      let ret = match self.raw.as_ref().get_transition_type {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret.into()
    }
  }
  /// Returns the globally unique identifier for this request or 0 if not
  /// specified. Can be used by CefResourceRequestHandler implementations in the
  /// browser process to track a single request across multiple callbacks.
  pub fn get_identifier(&mut self) -> u64 {
    unsafe {
      let ret = match self.raw.as_ref().get_identifier {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
}
pub type CefPostData = crate::include::refcounting::CefProxy<cef_sys::cef_post_data_t>;
#[allow(non_snake_case)]
impl CefPostData {
  /// Create a new CefPostData object.
  #[allow(non_snake_case)]
  pub fn create() -> Option<crate::include::CefPostData> {
    unsafe {
      let ret = cef_sys::cef_post_data_create();
      crate::include::CefPostData::from_cef_own(ret)
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
  /// Returns true if the underlying POST data includes elements that are not
  /// represented by this CefPostData object (for example, multi-part file upload
  /// data). Modifying CefPostData objects with excluded elements may result in
  /// the request failing.
  pub fn has_excluded_elements(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().has_excluded_elements {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns the number of existing post data elements.
  pub fn get_element_count(&mut self) -> u64 {
    unsafe {
      let ret = match self.raw.as_ref().get_element_count {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  /// Remove the specified post data element.  Returns true if the removal
  /// succeeds.
  pub fn remove_element(&mut self, element: crate::include::CefPostDataElement) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().remove_element {
        Some(f) => f(self.raw.as_ptr(),crate::include::CefPostDataElement::to_cef_own(element),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Add the specified post data element.  Returns true if the add succeeds.
  pub fn add_element(&mut self, element: crate::include::CefPostDataElement) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().add_element {
        Some(f) => f(self.raw.as_ptr(),crate::include::CefPostDataElement::to_cef_own(element),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Remove all existing post data elements.
  pub fn remove_elements(&mut self) -> () {
    unsafe {
      let ret = match self.raw.as_ref().remove_elements {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
}
pub type CefPostDataElement = crate::include::refcounting::CefProxy<cef_sys::cef_post_data_element_t>;
#[allow(non_snake_case)]
impl CefPostDataElement {
  /// Create a new CefPostDataElement object.
  #[allow(non_snake_case)]
  pub fn create() -> Option<crate::include::CefPostDataElement> {
    unsafe {
      let ret = cef_sys::cef_post_data_element_create();
      crate::include::CefPostDataElement::from_cef_own(ret)
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
  /// Remove all contents from the post data element.
  pub fn set_to_empty(&mut self) -> () {
    unsafe {
      let ret = match self.raw.as_ref().set_to_empty {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  /// The post data element will represent a file.
  pub fn set_to_file(&mut self, fileName: &crate::include::internal::CefString) -> () {
    unsafe {
      let ret = match self.raw.as_ref().set_to_file {
        Some(f) => f(self.raw.as_ptr(),fileName as *const _ as *const _,),
        None => panic!(),
      };
      ret
    }
  }
  /// The post data element will represent bytes.  The bytes passed
  /// in will be copied.
  pub fn set_to_bytes(&mut self, size: &[u8]) -> () {
    unsafe {
      let ret = match self.raw.as_ref().set_to_bytes {
        Some(f) => f(self.raw.as_ptr(),size.len() as _,size.as_ptr() as *const _,),
        None => panic!(),
      };
      ret
    }
  }
  /// Return the type of this post data element.
  pub fn get_type(&mut self) -> crate::include::internal::CefPostdataelementType {
    unsafe {
      let ret = match self.raw.as_ref().get_type {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret.into()
    }
  }
  /// Return the file name.
  pub fn get_file(&mut self) -> crate::include::internal::CefStringUserFree {
    unsafe {
      let ret = match self.raw.as_ref().get_file {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefStringUserFree::from_cef(ret).unwrap()
    }
  }
  /// Return the number of bytes.
  pub fn get_bytes_count(&mut self) -> u64 {
    unsafe {
      let ret = match self.raw.as_ref().get_bytes_count {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  /// Read up to |size| bytes into |bytes| and return the number of bytes
  /// actually read.
  pub fn get_bytes(&mut self, size: &mut [u8]) -> u64 {
    unsafe {
      let ret = match self.raw.as_ref().get_bytes {
        Some(f) => f(self.raw.as_ptr(),size.len() as _,size.as_mut_ptr() as *mut _,),
        None => panic!(),
      };
      ret
    }
  }
}
