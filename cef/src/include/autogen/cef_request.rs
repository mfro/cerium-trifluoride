pub type CefRequest = crate::include::base::CefProxy<cef_sys::cef_request_t>;
#[allow(non_snake_case)]
impl CefRequest {
  pub fn is_read_only(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_read_only {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
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
  pub fn set_url(&mut self, url: &crate::include::internal::CefString) -> () {
    unsafe {
      let ret = match self.raw.as_ref().set_url {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(url),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn get_method(&mut self) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_method {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  pub fn set_method(&mut self, method: &crate::include::internal::CefString) -> () {
    unsafe {
      let ret = match self.raw.as_ref().set_method {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(method),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn set_referrer(&mut self, referrer_url: Option<&crate::include::internal::CefString>, policy: crate::include::internal::CefReferrerPolicy) -> () {
    unsafe {
      let ret = match self.raw.as_ref().set_referrer {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(referrer_url),policy.into(),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn get_referrer_url(&mut self) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_referrer_url {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  pub fn get_referrer_policy(&mut self) -> crate::include::internal::CefReferrerPolicy {
    unsafe {
      let ret = match self.raw.as_ref().get_referrer_policy {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret.into()
    }
  }
  pub fn get_post_data(&mut self) -> Option<crate::include::CefPostData> {
    unsafe {
      let ret = match self.raw.as_ref().get_post_data {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefPostData::from_cef_own(ret)
    }
  }
  pub fn set_post_data(&mut self, postData: crate::include::CefPostData) -> () {
    unsafe {
      let ret = match self.raw.as_ref().set_post_data {
        Some(f) => f(self.raw.as_ptr(),crate::include::CefPostData::to_cef_own(postData),),
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
  pub fn get_flags(&mut self) -> i32 {
    unsafe {
      let ret = match self.raw.as_ref().get_flags {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn set_flags(&mut self, flags: i32) -> () {
    unsafe {
      let ret = match self.raw.as_ref().set_flags {
        Some(f) => f(self.raw.as_ptr(),flags,),
        None => panic!(),
      };
      ret
    }
  }
  pub fn get_first_party_for_cookies(&mut self) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_first_party_for_cookies {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  pub fn set_first_party_for_cookies(&mut self, url: Option<&crate::include::internal::CefString>) -> () {
    unsafe {
      let ret = match self.raw.as_ref().set_first_party_for_cookies {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(url),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn get_resource_type(&mut self) -> crate::include::internal::CefResourceType {
    unsafe {
      let ret = match self.raw.as_ref().get_resource_type {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret.into()
    }
  }
  pub fn get_transition_type(&mut self) -> crate::include::internal::CefTransitionType {
    unsafe {
      let ret = match self.raw.as_ref().get_transition_type {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret.into()
    }
  }
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
pub type CefPostData = crate::include::base::CefProxy<cef_sys::cef_post_data_t>;
#[allow(non_snake_case)]
impl CefPostData {
  pub fn is_read_only(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_read_only {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn has_excluded_elements(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().has_excluded_elements {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn get_element_count(&mut self) -> u64 {
    unsafe {
      let ret = match self.raw.as_ref().get_element_count {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn remove_element(&mut self, element: crate::include::CefPostDataElement) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().remove_element {
        Some(f) => f(self.raw.as_ptr(),crate::include::CefPostDataElement::to_cef_own(element),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn add_element(&mut self, element: crate::include::CefPostDataElement) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().add_element {
        Some(f) => f(self.raw.as_ptr(),crate::include::CefPostDataElement::to_cef_own(element),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
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
pub type CefPostDataElement = crate::include::base::CefProxy<cef_sys::cef_post_data_element_t>;
#[allow(non_snake_case)]
impl CefPostDataElement {
  pub fn is_read_only(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_read_only {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn set_to_empty(&mut self) -> () {
    unsafe {
      let ret = match self.raw.as_ref().set_to_empty {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn set_to_file(&mut self, fileName: &crate::include::internal::CefString) -> () {
    unsafe {
      let ret = match self.raw.as_ref().set_to_file {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(fileName),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn get_type(&mut self) -> crate::include::internal::CefPostdataelementType {
    unsafe {
      let ret = match self.raw.as_ref().get_type {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret.into()
    }
  }
  pub fn get_file(&mut self) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_file {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  pub fn get_bytes_count(&mut self) -> u64 {
    unsafe {
      let ret = match self.raw.as_ref().get_bytes_count {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn get_bytes(&mut self, size: u64, bytes: &mut std::os::raw::c_void) -> u64 {
    unsafe {
      let ret = match self.raw.as_ref().get_bytes {
        Some(f) => f(self.raw.as_ptr(),size,bytes,),
        None => panic!(),
      };
      ret
    }
  }
}
