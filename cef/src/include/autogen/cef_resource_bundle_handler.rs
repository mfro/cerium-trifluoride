/// Class used to implement a custom resource bundle interface. See CefSettings
/// for additional options related to resource bundle loading. The methods of
/// this class may be called on multiple threads.
#[allow(non_snake_case)]
#[allow(unused_variables)]
pub trait ResourceBundleHandler {
  /// Called to retrieve a localized translation for the specified |string_id|.
  /// To provide the translation set |string| to the translation string and
  /// return true. To use the default translation return false. Include
  /// cef_pack_strings.h for a listing of valid string ID values.
  fn get_localized_string(&mut self, string_id: i32, string: &mut crate::include::internal::CefString) -> bool { Default::default() }
}
define_refcounted!(ResourceBundleHandler, CefResourceBundleHandler, cef_resource_bundle_handler_t, get_localized_string: cef_resource_bundle_handler_t_get_localized_string,);
#[allow(non_snake_case)]
unsafe extern "C" fn cef_resource_bundle_handler_t_get_localized_string(_self: *mut cef_sys::cef_resource_bundle_handler_t, string_id: i32, string: *mut cef_sys::cef_string_t) -> i32 {
  let ret = CefResourceBundleHandler::from_cef(_self, true).get().get_localized_string(string_id,&mut *(string as *mut _),);
  if ret { 1 } else { 0 }
}
