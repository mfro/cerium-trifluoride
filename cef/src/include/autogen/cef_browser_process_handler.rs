/// Class used to implement browser process callbacks. The methods of this class
/// will be called on the browser process main thread unless otherwise indicated.
#[allow(non_snake_case)]
#[allow(unused_variables)]
pub trait BrowserProcessHandler {
  /// Called on the browser process UI thread immediately after the CEF context
  /// has been initialized.
  fn on_context_initialized(&mut self) -> () { Default::default() }
  /// Called before a child process is launched. Will be called on the browser
  /// process UI thread when launching a render process and on the browser
  /// process IO thread when launching a GPU or plugin process. Provides an
  /// opportunity to modify the child process command line. Do not keep a
  /// reference to |command_line| outside of this method.
  fn on_before_child_process_launch(&mut self, command_line: crate::include::CefCommandLine) -> () { Default::default() }
  /// Return the handler for printing on Linux. If a print handler is not
  /// provided then printing will not be supported on the Linux platform.
  fn get_print_handler(&mut self) -> Option<crate::include::CefPrintHandler> { Default::default() }
  /// Called from any thread when work has been scheduled for the browser process
  /// main (UI) thread. This callback is used in combination with CefSettings.
  /// external_message_pump and CefDoMessageLoopWork() in cases where the CEF
  /// message loop must be integrated into an existing application message loop
  /// (see additional comments and warnings on CefDoMessageLoopWork). This
  /// callback should schedule a CefDoMessageLoopWork() call to happen on the
  /// main (UI) thread. |delay_ms| is the requested delay in milliseconds. If
  /// |delay_ms| is <= 0 then the call should happen reasonably soon. If
  /// |delay_ms| is > 0 then the call should be scheduled to happen after the
  /// specified delay and any currently pending scheduled call should be
  /// cancelled.
  fn on_schedule_message_pump_work(&mut self, delay_ms: i64) -> () { Default::default() }
}
define_refcounted!(BrowserProcessHandler, CefBrowserProcessHandler, cef_browser_process_handler_t, on_context_initialized: cef_browser_process_handler_t_on_context_initialized,on_before_child_process_launch: cef_browser_process_handler_t_on_before_child_process_launch,get_print_handler: cef_browser_process_handler_t_get_print_handler,on_schedule_message_pump_work: cef_browser_process_handler_t_on_schedule_message_pump_work,);
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
