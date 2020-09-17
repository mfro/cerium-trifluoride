/// Callback interface for CefBrowserHost::AddDevToolsMessageObserver. The
/// methods of this class will be called on the browser process UI thread.
#[allow(non_snake_case)]
#[allow(unused_variables)]
pub trait DevToolsMessageObserver {
  /// Method that will be called on receipt of a DevTools protocol message.
  /// |browser| is the originating browser instance. |message| is a UTF8-encoded
  /// JSON dictionary representing either a method result or an event. |message|
  /// is only valid for the scope of this callback and should be copied if
  /// necessary. Return true if the message was handled or false if the message
  /// should be further processed and passed to the OnDevToolsMethodResult or
  /// OnDevToolsEvent methods as appropriate.
  /// 
  /// Method result dictionaries include an "id" (int) value that identifies the
  /// orginating method call sent from CefBrowserHost::SendDevToolsMessage, and
  /// optionally either a "result" (dictionary) or "error" (dictionary) value.
  /// The "error" dictionary will contain "code" (int) and "message" (string)
  /// values. Event dictionaries include a "method" (string) value and optionally
  /// a "params" (dictionary) value. See the DevTools protocol documentation at
  /// https://chromedevtools.github.io/devtools-protocol/ for details of
  /// supported method calls and the expected "result" or "params" dictionary
  /// contents. JSON dictionaries can be parsed using the CefParseJSON function
  /// if desired, however be aware of performance considerations when parsing
  /// large messages (some of which may exceed 1MB in size).
  fn on_dev_tools_message(&mut self, browser: crate::include::CefBrowser, message: &[u8]) -> bool { Default::default() }
  /// Method that will be called after attempted execution of a DevTools protocol
  /// method. |browser| is the originating browser instance. |message_id| is the
  /// "id" value that identifies the originating method call message. If the
  /// method succeeded |success| will be true and |result| will be the
  /// UTF8-encoded JSON "result" dictionary value (which may be empty). If the
  /// method failed |success| will be false and |result| will be the UTF8-encoded
  /// JSON "error" dictionary value. |result| is only valid for the scope of this
  /// callback and should be copied if necessary. See the OnDevToolsMessage
  /// documentation for additional details on |result| contents.
  fn on_dev_tools_method_result(&mut self, browser: crate::include::CefBrowser, message_id: i32, success: bool, result: &[u8]) -> () { Default::default() }
  /// Method that will be called on receipt of a DevTools protocol event.
  /// |browser| is the originating browser instance. |method| is the "method"
  /// value. |params| is the UTF8-encoded JSON "params" dictionary value (which
  /// may be empty). |params| is only valid for the scope of this callback and
  /// should be copied if necessary. See the OnDevToolsMessage documentation for
  /// additional details on |params| contents.
  fn on_dev_tools_event(&mut self, browser: crate::include::CefBrowser, method: &crate::include::internal::CefString, params: &[u8]) -> () { Default::default() }
  /// Method that will be called when the DevTools agent has attached. |browser|
  /// is the originating browser instance. This will generally occur in response
  /// to the first message sent while the agent is detached.
  fn on_dev_tools_agent_attached(&mut self, browser: crate::include::CefBrowser) -> () { Default::default() }
  /// Method that will be called when the DevTools agent has detached. |browser|
  /// is the originating browser instance. Any method results that were pending
  /// before the agent became detached will not be delivered, and any active
  /// event subscriptions will be canceled.
  fn on_dev_tools_agent_detached(&mut self, browser: crate::include::CefBrowser) -> () { Default::default() }
}
define_refcounted!(DevToolsMessageObserver, CefDevToolsMessageObserver, cef_dev_tools_message_observer_t, on_dev_tools_message: cef_dev_tools_message_observer_t_on_dev_tools_message,on_dev_tools_method_result: cef_dev_tools_message_observer_t_on_dev_tools_method_result,on_dev_tools_event: cef_dev_tools_message_observer_t_on_dev_tools_event,on_dev_tools_agent_attached: cef_dev_tools_message_observer_t_on_dev_tools_agent_attached,on_dev_tools_agent_detached: cef_dev_tools_message_observer_t_on_dev_tools_agent_detached,);
#[allow(non_snake_case)]
unsafe extern "C" fn cef_dev_tools_message_observer_t_on_dev_tools_message(_self: *mut cef_sys::cef_dev_tools_message_observer_t, browser: *mut cef_sys::cef_browser_t, message0: *const std::os::raw::c_void, message1: u64) -> i32 {
  let ret = CefDevToolsMessageObserver::from_cef(_self, true).get().on_dev_tools_message(crate::include::CefBrowser::from_cef_own(browser).unwrap(),std::slice::from_raw_parts(message0 as *const _, message1 as _),);
  if ret { 1 } else { 0 }
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_dev_tools_message_observer_t_on_dev_tools_method_result(_self: *mut cef_sys::cef_dev_tools_message_observer_t, browser: *mut cef_sys::cef_browser_t, message_id: i32, success: i32, result0: *const std::os::raw::c_void, result1: u64) -> () {
  let ret = CefDevToolsMessageObserver::from_cef(_self, true).get().on_dev_tools_method_result(crate::include::CefBrowser::from_cef_own(browser).unwrap(),message_id,if success == 0 { false } else { true },std::slice::from_raw_parts(result0 as *const _, result1 as _),);
  ret
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_dev_tools_message_observer_t_on_dev_tools_event(_self: *mut cef_sys::cef_dev_tools_message_observer_t, browser: *mut cef_sys::cef_browser_t, method: *const cef_sys::cef_string_t, params0: *const std::os::raw::c_void, params1: u64) -> () {
  let ret = CefDevToolsMessageObserver::from_cef(_self, true).get().on_dev_tools_event(crate::include::CefBrowser::from_cef_own(browser).unwrap(),&*(method as *const _),std::slice::from_raw_parts(params0 as *const _, params1 as _),);
  ret
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_dev_tools_message_observer_t_on_dev_tools_agent_attached(_self: *mut cef_sys::cef_dev_tools_message_observer_t, browser: *mut cef_sys::cef_browser_t) -> () {
  let ret = CefDevToolsMessageObserver::from_cef(_self, true).get().on_dev_tools_agent_attached(crate::include::CefBrowser::from_cef_own(browser).unwrap(),);
  ret
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_dev_tools_message_observer_t_on_dev_tools_agent_detached(_self: *mut cef_sys::cef_dev_tools_message_observer_t, browser: *mut cef_sys::cef_browser_t) -> () {
  let ret = CefDevToolsMessageObserver::from_cef(_self, true).get().on_dev_tools_agent_detached(crate::include::CefBrowser::from_cef_own(browser).unwrap(),);
  ret
}
