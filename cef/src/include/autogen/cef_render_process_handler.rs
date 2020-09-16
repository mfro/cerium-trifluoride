/// Class used to implement render process callbacks. The methods of this class
/// will be called on the render process main thread (TID_RENDERER) unless
/// otherwise indicated.
#[allow(non_snake_case)]
pub trait RenderProcessHandler {
  fn on_web_kit_initialized(&mut self) -> () { Default::default() }
  fn on_browser_created(&mut self, browser: crate::include::CefBrowser, extra_info: crate::include::CefDictionaryValue) -> () { Default::default() }
  fn on_browser_destroyed(&mut self, browser: crate::include::CefBrowser) -> () { Default::default() }
  fn get_load_handler(&mut self) -> Option<crate::include::CefLoadHandler> { Default::default() }
  fn on_context_created(&mut self, browser: crate::include::CefBrowser, frame: crate::include::CefFrame, context: crate::include::CefV8Context) -> () { Default::default() }
  fn on_context_released(&mut self, browser: crate::include::CefBrowser, frame: crate::include::CefFrame, context: crate::include::CefV8Context) -> () { Default::default() }
  fn on_uncaught_exception(&mut self, browser: crate::include::CefBrowser, frame: crate::include::CefFrame, context: crate::include::CefV8Context, exception: crate::include::CefV8Exception, stackTrace: crate::include::CefV8StackTrace) -> () { Default::default() }
  fn on_focused_node_changed(&mut self, browser: crate::include::CefBrowser, frame: crate::include::CefFrame, node: crate::include::CefDOMNode) -> () { Default::default() }
  fn on_process_message_received(&mut self, browser: crate::include::CefBrowser, frame: crate::include::CefFrame, source_process: crate::include::internal::CefProcessId, message: crate::include::CefProcessMessage) -> bool { Default::default() }
}
define_refcounted!(RenderProcessHandler, render_process_handler, on_web_kit_initialized,on_browser_created,on_browser_destroyed,get_load_handler,on_context_created,on_context_released,on_uncaught_exception,on_focused_node_changed,on_process_message_received,);
#[allow(non_snake_case)]
unsafe extern "C" fn cef_render_process_handler_t_on_web_kit_initialized(_self: *mut cef_sys::cef_render_process_handler_t) -> () {
  let ret = CefRenderProcessHandler::from_cef(_self, true).get().on_web_kit_initialized();
  ret
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_render_process_handler_t_on_browser_created(_self: *mut cef_sys::cef_render_process_handler_t, browser: *mut cef_sys::cef_browser_t, extra_info: *mut cef_sys::cef_dictionary_value_t) -> () {
  let ret = CefRenderProcessHandler::from_cef(_self, true).get().on_browser_created(crate::include::CefBrowser::from_cef_own(browser).unwrap(),crate::include::CefDictionaryValue::from_cef_own(extra_info).unwrap(),);
  ret
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_render_process_handler_t_on_browser_destroyed(_self: *mut cef_sys::cef_render_process_handler_t, browser: *mut cef_sys::cef_browser_t) -> () {
  let ret = CefRenderProcessHandler::from_cef(_self, true).get().on_browser_destroyed(crate::include::CefBrowser::from_cef_own(browser).unwrap(),);
  ret
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_render_process_handler_t_get_load_handler(_self: *mut cef_sys::cef_render_process_handler_t) -> *mut cef_sys::cef_load_handler_t {
  let ret = CefRenderProcessHandler::from_cef(_self, true).get().get_load_handler();
  ret.map_or(std::ptr::null_mut(), |o| crate::include::CefLoadHandler::to_cef_own(o))
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_render_process_handler_t_on_context_created(_self: *mut cef_sys::cef_render_process_handler_t, browser: *mut cef_sys::cef_browser_t, frame: *mut cef_sys::cef_frame_t, context: *mut cef_sys::cef_v8context_t) -> () {
  let ret = CefRenderProcessHandler::from_cef(_self, true).get().on_context_created(crate::include::CefBrowser::from_cef_own(browser).unwrap(),crate::include::CefFrame::from_cef_own(frame).unwrap(),crate::include::CefV8Context::from_cef_own(context).unwrap(),);
  ret
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_render_process_handler_t_on_context_released(_self: *mut cef_sys::cef_render_process_handler_t, browser: *mut cef_sys::cef_browser_t, frame: *mut cef_sys::cef_frame_t, context: *mut cef_sys::cef_v8context_t) -> () {
  let ret = CefRenderProcessHandler::from_cef(_self, true).get().on_context_released(crate::include::CefBrowser::from_cef_own(browser).unwrap(),crate::include::CefFrame::from_cef_own(frame).unwrap(),crate::include::CefV8Context::from_cef_own(context).unwrap(),);
  ret
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_render_process_handler_t_on_uncaught_exception(_self: *mut cef_sys::cef_render_process_handler_t, browser: *mut cef_sys::cef_browser_t, frame: *mut cef_sys::cef_frame_t, context: *mut cef_sys::cef_v8context_t, exception: *mut cef_sys::cef_v8exception_t, stackTrace: *mut cef_sys::cef_v8stack_trace_t) -> () {
  let ret = CefRenderProcessHandler::from_cef(_self, true).get().on_uncaught_exception(crate::include::CefBrowser::from_cef_own(browser).unwrap(),crate::include::CefFrame::from_cef_own(frame).unwrap(),crate::include::CefV8Context::from_cef_own(context).unwrap(),crate::include::CefV8Exception::from_cef_own(exception).unwrap(),crate::include::CefV8StackTrace::from_cef_own(stackTrace).unwrap(),);
  ret
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_render_process_handler_t_on_focused_node_changed(_self: *mut cef_sys::cef_render_process_handler_t, browser: *mut cef_sys::cef_browser_t, frame: *mut cef_sys::cef_frame_t, node: *mut cef_sys::cef_domnode_t) -> () {
  let ret = CefRenderProcessHandler::from_cef(_self, true).get().on_focused_node_changed(crate::include::CefBrowser::from_cef_own(browser).unwrap(),crate::include::CefFrame::from_cef_own(frame).unwrap(),crate::include::CefDOMNode::from_cef_own(node).unwrap(),);
  ret
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_render_process_handler_t_on_process_message_received(_self: *mut cef_sys::cef_render_process_handler_t, browser: *mut cef_sys::cef_browser_t, frame: *mut cef_sys::cef_frame_t, source_process: cef_sys::cef_process_id_t, message: *mut cef_sys::cef_process_message_t) -> i32 {
  let ret = CefRenderProcessHandler::from_cef(_self, true).get().on_process_message_received(crate::include::CefBrowser::from_cef_own(browser).unwrap(),crate::include::CefFrame::from_cef_own(frame).unwrap(),source_process.into(),crate::include::CefProcessMessage::from_cef_own(message).unwrap(),);
  if ret { 1 } else { 0 }
}
