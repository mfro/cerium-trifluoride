/// Class used to implement a custom resource bundle interface. See CefSettings
/// for additional options related to resource bundle loading. The methods of
/// this class may be called on multiple threads.
#[allow(non_snake_case)]
pub trait ResourceBundleHandler {
  fn get_localized_string(&mut self, string_id: i32, string: &mut crate::include::internal::CefString) -> bool { Default::default() }
}
define_refcounted!(ResourceBundleHandler, resource_bundle_handler, get_localized_string,);
#[allow(non_snake_case)]
unsafe extern "C" fn cef_resource_bundle_handler_t_get_localized_string(_self: *mut cef_sys::cef_resource_bundle_handler_t, string_id: i32, string: *mut cef_sys::cef_string_t) -> i32 {
  let ret = CefResourceBundleHandler::from_cef(_self, true).get().get_localized_string(string_id,&mut crate::include::internal::CefString::from_cef(string).unwrap(),);
  if ret { 1 } else { 0 }
}
