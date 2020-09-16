pub type CefExtension = crate::include::base::CefProxy<cef_sys::cef_extension_t>;
#[allow(non_snake_case)]
impl CefExtension {
  pub fn get_identifier(&mut self) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_identifier {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  pub fn get_path(&mut self) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_path {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  pub fn get_manifest(&mut self) -> Option<crate::include::CefDictionaryValue> {
    unsafe {
      let ret = match self.raw.as_ref().get_manifest {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefDictionaryValue::from_cef_own(ret)
    }
  }
  pub fn is_same(&mut self, that: crate::include::CefExtension) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_same {
        Some(f) => f(self.raw.as_ptr(),crate::include::CefExtension::to_cef_own(that),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn get_handler(&mut self) -> Option<crate::include::CefExtensionHandler> {
    unsafe {
      let ret = match self.raw.as_ref().get_handler {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefExtensionHandler::from_cef_own(ret)
    }
  }
  pub fn get_loader_context(&mut self) -> Option<crate::include::CefRequestContext> {
    unsafe {
      let ret = match self.raw.as_ref().get_loader_context {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefRequestContext::from_cef_own(ret)
    }
  }
  pub fn is_loaded(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_loaded {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn unload(&mut self) -> () {
    unsafe {
      let ret = match self.raw.as_ref().unload {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
}
