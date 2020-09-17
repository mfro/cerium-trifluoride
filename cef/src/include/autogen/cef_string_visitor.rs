/// Implement this interface to receive string values asynchronously.
#[allow(non_snake_case)]
#[allow(unused_variables)]
pub trait StringVisitor {
  /// Method that will be executed.
  fn visit(&mut self, string: Option<&crate::include::internal::CefString>) -> () { Default::default() }
}
define_refcounted!(StringVisitor, CefStringVisitor, cef_string_visitor_t, visit: cef_string_visitor_t_visit,);
#[allow(non_snake_case)]
unsafe extern "C" fn cef_string_visitor_t_visit(_self: *mut cef_sys::cef_string_visitor_t, string: *const cef_sys::cef_string_t) -> () {
  let ret = CefStringVisitor::from_cef(_self, true).get().visit(match &crate::include::internal::CefString::from_cef(string) { Some(ref x) => Some(x), None => None },);
  ret
}
