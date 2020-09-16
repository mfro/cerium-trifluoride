/// Implement this interface to handle events related to keyboard input. The
/// methods of this class will be called on the UI thread.
#[allow(non_snake_case)]
pub trait KeyboardHandler {
  fn on_pre_key_event(&mut self, browser: crate::include::CefBrowser, event: &crate::include::internal::CefKeyEvent, os_event: crate::include::internal::CefEventHandle, is_keyboard_shortcut: &mut bool) -> bool { Default::default() }
  fn on_key_event(&mut self, browser: crate::include::CefBrowser, event: &crate::include::internal::CefKeyEvent, os_event: crate::include::internal::CefEventHandle) -> bool { Default::default() }
}
define_refcounted!(KeyboardHandler, keyboard_handler, on_pre_key_event,on_key_event,);
#[allow(non_snake_case)]
unsafe extern "C" fn cef_keyboard_handler_t_on_pre_key_event(_self: *mut cef_sys::cef_keyboard_handler_t, browser: *mut cef_sys::cef_browser_t, event: *const cef_sys::cef_key_event_t, os_event: cef_sys::cef_event_handle_t, is_keyboard_shortcut: *mut i32) -> i32 {
  let mut is_keyboard_shortcut__tmp = if *is_keyboard_shortcut == 0 { false } else { true };
  let ret = CefKeyboardHandler::from_cef(_self, true).get().on_pre_key_event(crate::include::CefBrowser::from_cef_own(browser).unwrap(),&*(event as *const _),os_event.into(),&mut is_keyboard_shortcut__tmp,);
  *is_keyboard_shortcut = if is_keyboard_shortcut__tmp { 1 } else { 0 };
  if ret { 1 } else { 0 }
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_keyboard_handler_t_on_key_event(_self: *mut cef_sys::cef_keyboard_handler_t, browser: *mut cef_sys::cef_browser_t, event: *const cef_sys::cef_key_event_t, os_event: cef_sys::cef_event_handle_t) -> i32 {
  let ret = CefKeyboardHandler::from_cef(_self, true).get().on_key_event(crate::include::CefBrowser::from_cef_own(browser).unwrap(),&*(event as *const _),os_event.into(),);
  if ret { 1 } else { 0 }
}
