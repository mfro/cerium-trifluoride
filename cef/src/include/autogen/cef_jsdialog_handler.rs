pub type CefJSDialogCallback = crate::include::base::CefProxy<cef_sys::cef_jsdialog_callback_t>;
#[allow(non_snake_case)]
impl CefJSDialogCallback {
  pub fn cont(&mut self, success: bool, user_input: Option<&crate::include::internal::CefString>) -> () {
    unsafe {
      let ret = match self.raw.as_ref().cont {
        Some(f) => f(self.raw.as_ptr(),if success { 1 } else { 0 },crate::include::internal::IntoCef::into_cef(user_input),),
        None => panic!(),
      };
      ret
    }
  }
}
/// Implement this interface to handle events related to JavaScript dialogs. The
/// methods of this class will be called on the UI thread.
#[allow(non_snake_case)]
pub trait JSDialogHandler {
  fn on_jsdialog(&mut self, browser: crate::include::CefBrowser, origin_url: &crate::include::internal::CefString, dialog_type: crate::include::internal::CefJsdialogType, message_text: &crate::include::internal::CefString, default_prompt_text: &crate::include::internal::CefString, callback: crate::include::CefJSDialogCallback, suppress_message: &mut bool) -> bool { Default::default() }
  fn on_before_unload_dialog(&mut self, browser: crate::include::CefBrowser, message_text: Option<&crate::include::internal::CefString>, is_reload: bool, callback: crate::include::CefJSDialogCallback) -> bool { Default::default() }
  fn on_reset_dialog_state(&mut self, browser: crate::include::CefBrowser) -> () { Default::default() }
  fn on_dialog_closed(&mut self, browser: crate::include::CefBrowser) -> () { Default::default() }
}
define_refcounted!(JSDialogHandler, jsdialog_handler, on_jsdialog,on_before_unload_dialog,on_reset_dialog_state,on_dialog_closed,);
#[allow(non_snake_case)]
unsafe extern "C" fn cef_jsdialog_handler_t_on_jsdialog(_self: *mut cef_sys::cef_jsdialog_handler_t, browser: *mut cef_sys::cef_browser_t, origin_url: *const cef_sys::cef_string_t, dialog_type: cef_sys::cef_jsdialog_type_t, message_text: *const cef_sys::cef_string_t, default_prompt_text: *const cef_sys::cef_string_t, callback: *mut cef_sys::cef_jsdialog_callback_t, suppress_message: *mut i32) -> i32 {
  let mut suppress_message__tmp = if *suppress_message == 0 { false } else { true };
  let ret = CefJSDialogHandler::from_cef(_self, true).get().on_jsdialog(crate::include::CefBrowser::from_cef_own(browser).unwrap(),&crate::include::internal::CefString::from_cef(origin_url).unwrap(),dialog_type.into(),&crate::include::internal::CefString::from_cef(message_text).unwrap(),&crate::include::internal::CefString::from_cef(default_prompt_text).unwrap(),crate::include::CefJSDialogCallback::from_cef_own(callback).unwrap(),&mut suppress_message__tmp,);
  *suppress_message = if suppress_message__tmp { 1 } else { 0 };
  if ret { 1 } else { 0 }
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_jsdialog_handler_t_on_before_unload_dialog(_self: *mut cef_sys::cef_jsdialog_handler_t, browser: *mut cef_sys::cef_browser_t, message_text: *const cef_sys::cef_string_t, is_reload: i32, callback: *mut cef_sys::cef_jsdialog_callback_t) -> i32 {
  let ret = CefJSDialogHandler::from_cef(_self, true).get().on_before_unload_dialog(crate::include::CefBrowser::from_cef_own(browser).unwrap(),match &crate::include::internal::CefString::from_cef(message_text) { Some(ref x) => Some(x), None => None },if is_reload == 0 { false } else { true },crate::include::CefJSDialogCallback::from_cef_own(callback).unwrap(),);
  if ret { 1 } else { 0 }
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_jsdialog_handler_t_on_reset_dialog_state(_self: *mut cef_sys::cef_jsdialog_handler_t, browser: *mut cef_sys::cef_browser_t) -> () {
  let ret = CefJSDialogHandler::from_cef(_self, true).get().on_reset_dialog_state(crate::include::CefBrowser::from_cef_own(browser).unwrap(),);
  ret
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_jsdialog_handler_t_on_dialog_closed(_self: *mut cef_sys::cef_jsdialog_handler_t, browser: *mut cef_sys::cef_browser_t) -> () {
  let ret = CefJSDialogHandler::from_cef(_self, true).get().on_dialog_closed(crate::include::CefBrowser::from_cef_own(browser).unwrap(),);
  ret
}
