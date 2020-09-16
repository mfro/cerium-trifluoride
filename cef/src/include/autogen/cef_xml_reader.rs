pub type CefXmlReader = crate::include::base::CefProxy<cef_sys::cef_xml_reader_t>;
#[allow(non_snake_case)]
impl CefXmlReader {
  pub fn move_to_next_node(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().move_to_next_node {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn close(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().close {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn has_error(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().has_error {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn get_error(&mut self) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_error {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  pub fn get_type(&mut self) -> crate::include::internal::CefXmlNodeType {
    unsafe {
      let ret = match self.raw.as_ref().get_type {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret.into()
    }
  }
  pub fn get_depth(&mut self) -> i32 {
    unsafe {
      let ret = match self.raw.as_ref().get_depth {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn get_local_name(&mut self) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_local_name {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  pub fn get_prefix(&mut self) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_prefix {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  pub fn get_qualified_name(&mut self) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_qualified_name {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  pub fn get_namespace_uri(&mut self) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_namespace_uri {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  pub fn get_base_uri(&mut self) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_base_uri {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  pub fn get_xml_lang(&mut self) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_xml_lang {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  pub fn is_empty_element(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_empty_element {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn has_value(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().has_value {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn get_value(&mut self) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_value {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  pub fn has_attributes(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().has_attributes {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn get_attribute_count(&mut self) -> u64 {
    unsafe {
      let ret = match self.raw.as_ref().get_attribute_count {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn get_attribute_byindex(&mut self, index: i32) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_attribute_byindex {
        Some(f) => f(self.raw.as_ptr(),index,),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  pub fn get_attribute_byqname(&mut self, qualifiedName: &crate::include::internal::CefString) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_attribute_byqname {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(qualifiedName),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  pub fn get_attribute_bylname(&mut self, localName: &crate::include::internal::CefString, namespaceURI: &crate::include::internal::CefString) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_attribute_bylname {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(localName),crate::include::internal::IntoCef::into_cef(namespaceURI),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  pub fn get_inner_xml(&mut self) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_inner_xml {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  pub fn get_outer_xml(&mut self) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_outer_xml {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  pub fn get_line_number(&mut self) -> i32 {
    unsafe {
      let ret = match self.raw.as_ref().get_line_number {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn move_to_attribute_byindex(&mut self, index: i32) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().move_to_attribute_byindex {
        Some(f) => f(self.raw.as_ptr(),index,),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn move_to_attribute_byqname(&mut self, qualifiedName: &crate::include::internal::CefString) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().move_to_attribute_byqname {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(qualifiedName),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn move_to_attribute_bylname(&mut self, localName: &crate::include::internal::CefString, namespaceURI: &crate::include::internal::CefString) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().move_to_attribute_bylname {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(localName),crate::include::internal::IntoCef::into_cef(namespaceURI),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn move_to_first_attribute(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().move_to_first_attribute {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn move_to_next_attribute(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().move_to_next_attribute {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn move_to_carrying_element(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().move_to_carrying_element {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
}
