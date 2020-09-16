/// Implement this interface to provide handler implementations. Methods will be
/// called by the process and/or thread indicated.
#[allow(non_snake_case)]
pub trait App {
  fn on_before_command_line_processing(&mut self, process_type: Option<&crate::include::internal::CefString>, command_line: crate::include::CefCommandLine) -> () { Default::default() }
  fn on_register_custom_schemes(&mut self, registrar: crate::include::CefSchemeRegistrar) -> () { Default::default() }
  fn get_resource_bundle_handler(&mut self) -> Option<crate::include::CefResourceBundleHandler> { Default::default() }
  fn get_browser_process_handler(&mut self) -> Option<crate::include::CefBrowserProcessHandler> { Default::default() }
  fn get_render_process_handler(&mut self) -> Option<crate::include::CefRenderProcessHandler> { Default::default() }
}
define_refcounted!(App, app, on_before_command_line_processing,on_register_custom_schemes,get_resource_bundle_handler,get_browser_process_handler,get_render_process_handler,);
#[allow(non_snake_case)]
unsafe extern "C" fn cef_app_t_on_before_command_line_processing(_self: *mut cef_sys::cef_app_t, process_type: *const cef_sys::cef_string_t, command_line: *mut cef_sys::cef_command_line_t) -> () {
  let ret = CefApp::from_cef(_self, true).get().on_before_command_line_processing(match &crate::include::internal::CefString::from_cef(process_type) { Some(ref x) => Some(x), None => None },crate::include::CefCommandLine::from_cef_own(command_line).unwrap(),);
  ret
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_app_t_on_register_custom_schemes(_self: *mut cef_sys::cef_app_t, registrar: *mut cef_sys::cef_scheme_registrar_t) -> () {
  let ret = CefApp::from_cef(_self, true).get().on_register_custom_schemes(crate::include::CefSchemeRegistrar::from_cef_ref(registrar).unwrap(),);
  ret
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_app_t_get_resource_bundle_handler(_self: *mut cef_sys::cef_app_t) -> *mut cef_sys::cef_resource_bundle_handler_t {
  let ret = CefApp::from_cef(_self, true).get().get_resource_bundle_handler();
  ret.map_or(std::ptr::null_mut(), |o| crate::include::CefResourceBundleHandler::to_cef_own(o))
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_app_t_get_browser_process_handler(_self: *mut cef_sys::cef_app_t) -> *mut cef_sys::cef_browser_process_handler_t {
  let ret = CefApp::from_cef(_self, true).get().get_browser_process_handler();
  ret.map_or(std::ptr::null_mut(), |o| crate::include::CefBrowserProcessHandler::to_cef_own(o))
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_app_t_get_render_process_handler(_self: *mut cef_sys::cef_app_t) -> *mut cef_sys::cef_render_process_handler_t {
  let ret = CefApp::from_cef(_self, true).get().get_render_process_handler();
  ret.map_or(std::ptr::null_mut(), |o| crate::include::CefRenderProcessHandler::to_cef_own(o))
}
