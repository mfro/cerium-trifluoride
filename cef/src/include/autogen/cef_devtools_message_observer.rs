/// Callback interface for CefBrowserHost::AddDevToolsMessageObserver. The
/// methods of this class will be called on the browser process UI thread.
#[allow(non_snake_case)]
#[allow(unused_variables)]
pub trait DevToolsMessageObserver {
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
define_refcounted!(DevToolsMessageObserver, CefDevToolsMessageObserver, cef_dev_tools_message_observer_t, on_dev_tools_agent_attached: cef_dev_tools_message_observer_t_on_dev_tools_agent_attached,on_dev_tools_agent_detached: cef_dev_tools_message_observer_t_on_dev_tools_agent_detached,);
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
