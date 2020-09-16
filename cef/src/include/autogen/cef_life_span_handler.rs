/// Implement this interface to handle events related to browser life span. The
/// methods of this class will be called on the UI thread unless otherwise
/// indicated.
#[allow(non_snake_case)]
pub trait LifeSpanHandler {
  fn on_before_popup(&mut self, browser: crate::include::CefBrowser, frame: crate::include::CefFrame, target_url: &crate::include::internal::CefString, target_frame_name: &crate::include::internal::CefString, target_disposition: crate::include::internal::CefWindowOpenDisposition, user_gesture: bool, popupFeatures: &crate::include::internal::CefPopupFeatures, windowInfo: &mut crate::include::internal::CefWindowInfo, client: &mut crate::include::CefClient, settings: &mut crate::include::internal::CefBrowserSettings, extra_info: &mut crate::include::CefDictionaryValue, no_javascript_access: &mut bool) -> bool { Default::default() }
  fn on_after_created(&mut self, browser: crate::include::CefBrowser) -> () { Default::default() }
  fn do_close(&mut self, browser: crate::include::CefBrowser) -> bool { Default::default() }
  fn on_before_close(&mut self, browser: crate::include::CefBrowser) -> () { Default::default() }
}
define_refcounted!(LifeSpanHandler, life_span_handler, on_before_popup,on_after_created,do_close,on_before_close,);
#[allow(non_snake_case)]
unsafe extern "C" fn cef_life_span_handler_t_on_before_popup(_self: *mut cef_sys::cef_life_span_handler_t, browser: *mut cef_sys::cef_browser_t, frame: *mut cef_sys::cef_frame_t, target_url: *const cef_sys::cef_string_t, target_frame_name: *const cef_sys::cef_string_t, target_disposition: cef_sys::cef_window_open_disposition_t, user_gesture: i32, popupFeatures: *const cef_sys::cef_popup_features_t, windowInfo: *mut cef_sys::cef_window_info_t, client: *mut *mut cef_sys::cef_client_t, settings: *mut cef_sys::cef_browser_settings_t, extra_info: *mut *mut cef_sys::cef_dictionary_value_t, no_javascript_access: *mut i32) -> i32 {
  let mut client__tmp = crate::include::CefClient::from_cef_own(*client).unwrap();
  let mut extra_info__tmp = crate::include::CefDictionaryValue::from_cef_own(*extra_info).unwrap();
  let mut no_javascript_access__tmp = if *no_javascript_access == 0 { false } else { true };
  let ret = CefLifeSpanHandler::from_cef(_self, true).get().on_before_popup(crate::include::CefBrowser::from_cef_own(browser).unwrap(),crate::include::CefFrame::from_cef_own(frame).unwrap(),&crate::include::internal::CefString::from_cef(target_url).unwrap(),&crate::include::internal::CefString::from_cef(target_frame_name).unwrap(),target_disposition.into(),if user_gesture == 0 { false } else { true },&*(popupFeatures as *const _),&mut *(windowInfo as *mut _),&mut client__tmp,&mut *(settings as *mut _),&mut extra_info__tmp,&mut no_javascript_access__tmp,);
  *client = crate::include::CefClient::to_cef_own(client__tmp);
  *extra_info = crate::include::CefDictionaryValue::to_cef_own(extra_info__tmp);
  *no_javascript_access = if no_javascript_access__tmp { 1 } else { 0 };
  if ret { 1 } else { 0 }
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_life_span_handler_t_on_after_created(_self: *mut cef_sys::cef_life_span_handler_t, browser: *mut cef_sys::cef_browser_t) -> () {
  let ret = CefLifeSpanHandler::from_cef(_self, true).get().on_after_created(crate::include::CefBrowser::from_cef_own(browser).unwrap(),);
  ret
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_life_span_handler_t_do_close(_self: *mut cef_sys::cef_life_span_handler_t, browser: *mut cef_sys::cef_browser_t) -> i32 {
  let ret = CefLifeSpanHandler::from_cef(_self, true).get().do_close(crate::include::CefBrowser::from_cef_own(browser).unwrap(),);
  if ret { 1 } else { 0 }
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_life_span_handler_t_on_before_close(_self: *mut cef_sys::cef_life_span_handler_t, browser: *mut cef_sys::cef_browser_t) -> () {
  let ret = CefLifeSpanHandler::from_cef(_self, true).get().on_before_close(crate::include::CefBrowser::from_cef_own(browser).unwrap(),);
  ret
}
