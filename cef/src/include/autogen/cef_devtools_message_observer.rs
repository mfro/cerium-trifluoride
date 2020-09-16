/// Callback interface for CefBrowserHost::AddDevToolsMessageObserver. The
/// methods of this class will be called on the browser process UI thread.
#[allow(non_snake_case)]
pub trait DevToolsMessageObserver {
  fn on_dev_tools_agent_attached(&mut self, browser: crate::include::CefBrowser) -> () { Default::default() }
  fn on_dev_tools_agent_detached(&mut self, browser: crate::include::CefBrowser) -> () { Default::default() }
}
define_refcounted!(DevToolsMessageObserver, dev_tools_message_observer, on_dev_tools_agent_attached,on_dev_tools_agent_detached,);
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
