pub type CefResourceBundle = crate::include::refcounting::CefProxy<cef_sys::cef_resource_bundle_t>;
#[allow(non_snake_case)]
impl CefResourceBundle {
  /// Returns the global resource bundle instance.
  #[allow(non_snake_case)]
  pub fn get_global() -> Option<crate::include::CefResourceBundle> {
    unsafe {
      let ret = cef_sys::cef_resource_bundle_get_global();
      crate::include::CefResourceBundle::from_cef_own(ret)
    }
  }
  /// Returns the localized string for the specified |string_id| or an empty
  /// string if the value is not found. Include cef_pack_strings.h for a listing
  /// of valid string ID values.
  pub fn get_localized_string(&mut self, string_id: i32) -> crate::include::internal::CefStringUserFree {
    unsafe {
      let ret = match self.raw.as_ref().get_localized_string {
        Some(f) => f(self.raw.as_ptr(),string_id,),
        None => panic!(),
      };
      crate::include::internal::CefStringUserFree::from_cef(ret).unwrap()
    }
  }
  /// Returns a CefBinaryValue containing the decompressed contents of the
  /// specified scale independent |resource_id| or NULL if not found. Include
  /// cef_pack_resources.h for a listing of valid resource ID values.
  pub fn get_data_resource(&mut self, resource_id: i32) -> Option<crate::include::CefBinaryValue> {
    unsafe {
      let ret = match self.raw.as_ref().get_data_resource {
        Some(f) => f(self.raw.as_ptr(),resource_id,),
        None => panic!(),
      };
      crate::include::CefBinaryValue::from_cef_own(ret)
    }
  }
  /// Returns a CefBinaryValue containing the decompressed contents of the
  /// specified |resource_id| nearest the scale factor |scale_factor| or NULL if
  /// not found. Use a |scale_factor| value of SCALE_FACTOR_NONE for scale
  /// independent resources or call GetDataResource instead.Include
  /// cef_pack_resources.h for a listing of valid resource ID values.
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
