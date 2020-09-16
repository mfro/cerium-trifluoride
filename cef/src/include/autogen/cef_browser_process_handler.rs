/// Class used to implement browser process callbacks. The methods of this class
/// will be called on the browser process main thread unless otherwise indicated.
#[allow(non_snake_case)]
pub trait BrowserProcessHandler {
  fn on_context_initialized(&mut self) -> () { Default::default() }
  fn on_before_child_process_launch(&mut self, command_line: crate::include::CefCommandLine) -> () { Default::default() }
  fn get_print_handler(&mut self) -> Option<crate::include::CefPrintHandler> { Default::default() }
  fn on_schedule_message_pump_work(&mut self, delay_ms: i64) -> () { Default::default() }
}
define_refcounted!(BrowserProcessHandler, browser_process_handler, on_context_initialized,on_before_child_process_launch,get_print_handler,on_schedule_message_pump_work,);
#[allow(non_snake_case)]
unsafe extern "C" fn cef_browser_process_handler_t_on_context_initialized(_self: *mut cef_sys::cef_browser_process_handler_t) -> () {
  let ret = CefBrowserProcessHandler::from_cef(_self, true).get().on_context_initialized();
  ret
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_browser_process_handler_t_on_before_child_process_launch(_self: *mut cef_sys::cef_browser_process_handler_t, command_line: *mut cef_sys::cef_command_line_t) -> () {
  let ret = CefBrowserProcessHandler::from_cef(_self, true).get().on_before_child_process_launch(crate::include::CefCommandLine::from_cef_own(command_line).unwrap(),);
  ret
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_browser_process_handler_t_get_print_handler(_self: *mut cef_sys::cef_browser_process_handler_t) -> *mut cef_sys::cef_print_handler_t {
  let ret = CefBrowserProcessHandler::from_cef(_self, true).get().get_print_handler();
  ret.map_or(std::ptr::null_mut(), |o| crate::include::CefPrintHandler::to_cef_own(o))
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_browser_process_handler_t_on_schedule_message_pump_work(_self: *mut cef_sys::cef_browser_process_handler_t, delay_ms: i64) -> () {
  let ret = CefBrowserProcessHandler::from_cef(_self, true).get().on_schedule_message_pump_work(delay_ms,);
  ret
}
