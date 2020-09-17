/// Implement this interface to receive accessibility notification when
/// accessibility events have been registered. The methods of this class will
/// be called on the UI thread.
#[allow(non_snake_case)]
#[allow(unused_variables)]
pub trait AccessibilityHandler {
  /// Called after renderer process sends accessibility tree changes to the
  /// browser process.
  fn on_accessibility_tree_change(&mut self, value: crate::include::CefValue) -> () { Default::default() }
  /// Called after renderer process sends accessibility location changes to the
  /// browser process.
  fn on_accessibility_location_change(&mut self, value: crate::include::CefValue) -> () { Default::default() }
}
define_refcounted!(AccessibilityHandler, CefAccessibilityHandler, cef_accessibility_handler_t, on_accessibility_tree_change: cef_accessibility_handler_t_on_accessibility_tree_change,on_accessibility_location_change: cef_accessibility_handler_t_on_accessibility_location_change,);
#[allow(non_snake_case)]
unsafe extern "C" fn cef_accessibility_handler_t_on_accessibility_tree_change(_self: *mut cef_sys::cef_accessibility_handler_t, value: *mut cef_sys::cef_value_t) -> () {
  let ret = CefAccessibilityHandler::from_cef(_self, true).get().on_accessibility_tree_change(crate::include::CefValue::from_cef_own(value).unwrap(),);
  ret
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_accessibility_handler_t_on_accessibility_location_change(_self: *mut cef_sys::cef_accessibility_handler_t, value: *mut cef_sys::cef_value_t) -> () {
  let ret = CefAccessibilityHandler::from_cef(_self, true).get().on_accessibility_location_change(crate::include::CefValue::from_cef_own(value).unwrap(),);
  ret
}
