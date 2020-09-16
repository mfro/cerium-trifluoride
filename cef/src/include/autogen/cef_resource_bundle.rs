pub type CefResourceBundle = crate::include::base::CefProxy<cef_sys::cef_resource_bundle_t>;
#[allow(non_snake_case)]
impl CefResourceBundle {
  pub fn get_localized_string(&mut self, string_id: i32) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_localized_string {
        Some(f) => f(self.raw.as_ptr(),string_id,),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  pub fn get_data_resource(&mut self, resource_id: i32) -> Option<crate::include::CefBinaryValue> {
    unsafe {
      let ret = match self.raw.as_ref().get_data_resource {
        Some(f) => f(self.raw.as_ptr(),resource_id,),
        None => panic!(),
      };
      crate::include::CefBinaryValue::from_cef_own(ret)
    }
  }
  pub fn get_data_resource_for_scale(&mut self, resource_id: i32, scale_factor: crate::include::internal::CefScaleFactor) -> Option<crate::include::CefBinaryValue> {
    unsafe {
      let ret = match self.raw.as_ref().get_data_resource_for_scale {
        Some(f) => f(self.raw.as_ptr(),resource_id,scale_factor.into(),),
        None => panic!(),
      };
      crate::include::CefBinaryValue::from_cef_own(ret)
    }
  }
}
