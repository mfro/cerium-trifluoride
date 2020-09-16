/// Implement this interface to provide handler implementations. The handler
/// instance will not be released until all objects related to the context have
/// been destroyed.
#[allow(non_snake_case)]
pub trait RequestContextHandler {
  fn on_request_context_initialized(&mut self, request_context: crate::include::CefRequestContext) -> () { Default::default() }
  fn on_before_plugin_load(&mut self, mime_type: &crate::include::internal::CefString, plugin_url: &crate::include::internal::CefString, is_main_frame: bool, top_origin_url: &crate::include::internal::CefString, plugin_info: crate::include::CefWebPluginInfo, plugin_policy: &mut crate::include::internal::CefPluginPolicy) -> bool { Default::default() }
  fn get_resource_request_handler(&mut self, browser: crate::include::CefBrowser, frame: crate::include::CefFrame, request: crate::include::CefRequest, is_navigation: bool, is_download: bool, request_initiator: &crate::include::internal::CefString, disable_default_handling: &mut bool) -> Option<crate::include::CefResourceRequestHandler> { Default::default() }
}
define_refcounted!(RequestContextHandler, request_context_handler, on_request_context_initialized,on_before_plugin_load,get_resource_request_handler,);
#[allow(non_snake_case)]
unsafe extern "C" fn cef_request_context_handler_t_on_request_context_initialized(_self: *mut cef_sys::cef_request_context_handler_t, request_context: *mut cef_sys::cef_request_context_t) -> () {
  let ret = CefRequestContextHandler::from_cef(_self, true).get().on_request_context_initialized(crate::include::CefRequestContext::from_cef_own(request_context).unwrap(),);
  ret
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_request_context_handler_t_on_before_plugin_load(_self: *mut cef_sys::cef_request_context_handler_t, mime_type: *const cef_sys::cef_string_t, plugin_url: *const cef_sys::cef_string_t, is_main_frame: i32, top_origin_url: *const cef_sys::cef_string_t, plugin_info: *mut cef_sys::cef_web_plugin_info_t, plugin_policy: *mut cef_sys::cef_plugin_policy_t) -> i32 {
  let ret = CefRequestContextHandler::from_cef(_self, true).get().on_before_plugin_load(&crate::include::internal::CefString::from_cef(mime_type).unwrap(),&crate::include::internal::CefString::from_cef(plugin_url).unwrap(),if is_main_frame == 0 { false } else { true },&crate::include::internal::CefString::from_cef(top_origin_url).unwrap(),crate::include::CefWebPluginInfo::from_cef_own(plugin_info).unwrap(),&mut *(plugin_policy as *mut _),);
  if ret { 1 } else { 0 }
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_request_context_handler_t_get_resource_request_handler(_self: *mut cef_sys::cef_request_context_handler_t, browser: *mut cef_sys::cef_browser_t, frame: *mut cef_sys::cef_frame_t, request: *mut cef_sys::cef_request_t, is_navigation: i32, is_download: i32, request_initiator: *const cef_sys::cef_string_t, disable_default_handling: *mut i32) -> *mut cef_sys::cef_resource_request_handler_t {
  let mut disable_default_handling__tmp = if *disable_default_handling == 0 { false } else { true };
  let ret = CefRequestContextHandler::from_cef(_self, true).get().get_resource_request_handler(crate::include::CefBrowser::from_cef_own(browser).unwrap(),crate::include::CefFrame::from_cef_own(frame).unwrap(),crate::include::CefRequest::from_cef_own(request).unwrap(),if is_navigation == 0 { false } else { true },if is_download == 0 { false } else { true },&crate::include::internal::CefString::from_cef(request_initiator).unwrap(),&mut disable_default_handling__tmp,);
  *disable_default_handling = if disable_default_handling__tmp { 1 } else { 0 };
  ret.map_or(std::ptr::null_mut(), |o| crate::include::CefResourceRequestHandler::to_cef_own(o))
}
