pub type CefJSDialogCallback = crate::include::refcounting::CefProxy<cef_sys::cef_jsdialog_callback_t>;
#[allow(non_snake_case)]
impl CefJSDialogCallback {
  /// Continue the JS dialog request. Set |success| to true if the OK button was
  /// pressed. The |user_input| value should be specified for prompt dialogs.
  pub fn cont(&mut self, success: bool, user_input: Option<&crate::include::internal::CefString>) -> () {
    unsafe {
      let ret = match self.raw.as_ref().cont {
        Some(f) => f(self.raw.as_ptr(),if success { 1 } else { 0 },match user_input { Some(user_input) => user_input as *const _ as *const _, None => std::ptr::null_mut() },),
        None => panic!(),
      };
      ret
    }
  }
}
/// Implement this interface to handle events related to JavaScript dialogs. The
/// methods of this class will be called on the UI thread.
#[allow(non_snake_case)]
#[allow(unused_variables)]
pub trait JSDialogHandler {
  /// Called to run a JavaScript dialog. If |origin_url| is non-empty it can be
  /// passed to the CefFormatUrlForSecurityDisplay function to retrieve a secure
  /// and user-friendly display string. The |default_prompt_text| value will be
  /// specified for prompt dialogs only. Set |suppress_message| to true and
  /// return false to suppress the message (suppressing messages is preferable to
  /// immediately executing the callback as this is used to detect presumably
  /// malicious behavior like spamming alert messages in onbeforeunload). Set
  /// |suppress_message| to false and return false to use the default
  /// implementation (the default implementation will show one modal dialog at a
  /// time and suppress any additional dialog requests until the displayed dialog
  /// is dismissed). Return true if the application will use a custom dialog or
  /// if the callback has been executed immediately. Custom dialogs may be either
  /// modal or modeless. If a custom dialog is used the application must execute
  /// |callback| once the custom dialog is dismissed.
  fn on_jsdialog(&mut self, browser: crate::include::CefBrowser, origin_url: Option<&crate::include::internal::CefString>, dialog_type: crate::include::internal::CefJsdialogType, message_text: Option<&crate::include::internal::CefString>, default_prompt_text: Option<&crate::include::internal::CefString>, callback: crate::include::CefJSDialogCallback, suppress_message: &mut bool) -> bool { Default::default() }
  /// Called to run a dialog asking the user if they want to leave a page. Return
  /// false to use the default dialog implementation. Return true if the
  /// application will use a custom dialog or if the callback has been executed
  /// immediately. Custom dialogs may be either modal or modeless. If a custom
  /// dialog is used the application must execute |callback| once the custom
  /// dialog is dismissed.
  fn on_before_unload_dialog(&mut self, browser: crate::include::CefBrowser, message_text: Option<&crate::include::internal::CefString>, is_reload: bool, callback: crate::include::CefJSDialogCallback) -> bool { Default::default() }
  /// Called to cancel any pending dialogs and reset any saved dialog state. Will
  /// be called due to events like page navigation irregardless of whether any
  /// dialogs are currently pending.
  fn on_reset_dialog_state(&mut self, browser: crate::include::CefBrowser) -> () { Default::default() }
  /// Called when the default implementation dialog is closed.
  fn on_dialog_closed(&mut self, browser: crate::include::CefBrowser) -> () { Default::default() }
}
define_refcounted!(JSDialogHandler, CefJSDialogHandler, cef_jsdialog_handler_t, on_jsdialog: cef_jsdialog_handler_t_on_jsdialog,on_before_unload_dialog: cef_jsdialog_handler_t_on_before_unload_dialog,on_reset_dialog_state: cef_jsdialog_handler_t_on_reset_dialog_state,on_dialog_closed: cef_jsdialog_handler_t_on_dialog_closed,);
#[allow(non_snake_case)]
unsafe extern "C" fn cef_jsdialog_handler_t_on_jsdialog(_self: *mut cef_sys::cef_jsdialog_handler_t, browser: *mut cef_sys::cef_browser_t, origin_url: *const cef_sys::cef_string_t, dialog_type: cef_sys::cef_jsdialog_type_t, message_text: *const cef_sys::cef_string_t, default_prompt_text: *const cef_sys::cef_string_t, callback: *mut cef_sys::cef_jsdialog_callback_t, suppress_message: *mut i32) -> i32 {
  let mut suppress_message__tmp = if *suppress_message == 0 { false } else { true };
  let ret = CefJSDialogHandler::from_cef(_self, true).get().on_jsdialog(crate::include::CefBrowser::from_cef_own(browser).unwrap(),if origin_url.is_null() { None } else { Some(&*(origin_url as *const _)) },dialog_type.into(),if message_text.is_null() { None } else { Some(&*(message_text as *const _)) },if default_prompt_text.is_null() { None } else { Some(&*(default_prompt_text as *const _)) },crate::include::CefJSDialogCallback::from_cef_own(callback).unwrap(),&mut suppress_message__tmp,);
  *suppress_message = if suppress_message__tmp { 1 } else { 0 };
  if ret { 1 } else { 0 }
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_jsdialog_handler_t_on_before_unload_dialog(_self: *mut cef_sys::cef_jsdialog_handler_t, browser: *mut cef_sys::cef_browser_t, message_text: *const cef_sys::cef_string_t, is_reload: i32, callback: *mut cef_sys::cef_jsdialog_callback_t) -> i32 {
  let ret = CefJSDialogHandler::from_cef(_self, true).get().on_before_unload_dialog(crate::include::CefBrowser::from_cef_own(browser).unwrap(),if message_text.is_null() { None } else { Some(&*(message_text as *const _)) },if is_reload == 0 { false } else { true },crate::include::CefJSDialogCallback::from_cef_own(callback).unwrap(),);
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
