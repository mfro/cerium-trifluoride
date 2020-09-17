/// Implement this interface to provide handler implementations.
#[allow(non_snake_case)]
#[allow(unused_variables)]
pub trait Client {
  /// Return the handler for audio rendering events.
  fn get_audio_handler(&mut self) -> Option<crate::include::CefAudioHandler> { Default::default() }
  /// Return the handler for context menus. If no handler is provided the default
  /// implementation will be used.
  fn get_context_menu_handler(&mut self) -> Option<crate::include::CefContextMenuHandler> { Default::default() }
  /// Return the handler for dialogs. If no handler is provided the default
  /// implementation will be used.
  fn get_dialog_handler(&mut self) -> Option<crate::include::CefDialogHandler> { Default::default() }
  /// Return the handler for browser display state events.
  fn get_display_handler(&mut self) -> Option<crate::include::CefDisplayHandler> { Default::default() }
  /// Return the handler for download events. If no handler is returned downloads
  /// will not be allowed.
  fn get_download_handler(&mut self) -> Option<crate::include::CefDownloadHandler> { Default::default() }
  /// Return the handler for drag events.
  fn get_drag_handler(&mut self) -> Option<crate::include::CefDragHandler> { Default::default() }
  /// Return the handler for find result events.
  fn get_find_handler(&mut self) -> Option<crate::include::CefFindHandler> { Default::default() }
  /// Return the handler for focus events.
  fn get_focus_handler(&mut self) -> Option<crate::include::CefFocusHandler> { Default::default() }
  /// Return the handler for JavaScript dialogs. If no handler is provided the
  /// default implementation will be used.
  fn get_jsdialog_handler(&mut self) -> Option<crate::include::CefJSDialogHandler> { Default::default() }
  /// Return the handler for keyboard events.
  fn get_keyboard_handler(&mut self) -> Option<crate::include::CefKeyboardHandler> { Default::default() }
  /// Return the handler for browser life span events.
  fn get_life_span_handler(&mut self) -> Option<crate::include::CefLifeSpanHandler> { Default::default() }
  /// Return the handler for browser load status events.
  fn get_load_handler(&mut self) -> Option<crate::include::CefLoadHandler> { Default::default() }
  /// Return the handler for off-screen rendering events.
  fn get_render_handler(&mut self) -> Option<crate::include::CefRenderHandler> { Default::default() }
  /// Return the handler for browser request events.
  fn get_request_handler(&mut self) -> Option<crate::include::CefRequestHandler> { Default::default() }
  /// Called when a new message is received from a different process. Return true
  /// if the message was handled or false otherwise. Do not keep a reference to
  /// or attempt to access the message outside of this callback.
  fn on_process_message_received(&mut self, browser: crate::include::CefBrowser, frame: crate::include::CefFrame, source_process: crate::include::internal::CefProcessId, message: crate::include::CefProcessMessage) -> bool { Default::default() }
}
define_refcounted!(Client, CefClient, cef_client_t, get_audio_handler: cef_client_t_get_audio_handler,get_context_menu_handler: cef_client_t_get_context_menu_handler,get_dialog_handler: cef_client_t_get_dialog_handler,get_display_handler: cef_client_t_get_display_handler,get_download_handler: cef_client_t_get_download_handler,get_drag_handler: cef_client_t_get_drag_handler,get_find_handler: cef_client_t_get_find_handler,get_focus_handler: cef_client_t_get_focus_handler,get_jsdialog_handler: cef_client_t_get_jsdialog_handler,get_keyboard_handler: cef_client_t_get_keyboard_handler,get_life_span_handler: cef_client_t_get_life_span_handler,get_load_handler: cef_client_t_get_load_handler,get_render_handler: cef_client_t_get_render_handler,get_request_handler: cef_client_t_get_request_handler,on_process_message_received: cef_client_t_on_process_message_received,);
#[allow(non_snake_case)]
unsafe extern "C" fn cef_client_t_get_audio_handler(_self: *mut cef_sys::cef_client_t) -> *mut cef_sys::cef_audio_handler_t {
  let ret = CefClient::from_cef(_self, true).get().get_audio_handler();
  ret.map_or(std::ptr::null_mut(), |o| crate::include::CefAudioHandler::to_cef_own(o))
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_client_t_get_context_menu_handler(_self: *mut cef_sys::cef_client_t) -> *mut cef_sys::cef_context_menu_handler_t {
  let ret = CefClient::from_cef(_self, true).get().get_context_menu_handler();
  ret.map_or(std::ptr::null_mut(), |o| crate::include::CefContextMenuHandler::to_cef_own(o))
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_client_t_get_dialog_handler(_self: *mut cef_sys::cef_client_t) -> *mut cef_sys::cef_dialog_handler_t {
  let ret = CefClient::from_cef(_self, true).get().get_dialog_handler();
  ret.map_or(std::ptr::null_mut(), |o| crate::include::CefDialogHandler::to_cef_own(o))
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_client_t_get_display_handler(_self: *mut cef_sys::cef_client_t) -> *mut cef_sys::cef_display_handler_t {
  let ret = CefClient::from_cef(_self, true).get().get_display_handler();
  ret.map_or(std::ptr::null_mut(), |o| crate::include::CefDisplayHandler::to_cef_own(o))
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_client_t_get_download_handler(_self: *mut cef_sys::cef_client_t) -> *mut cef_sys::cef_download_handler_t {
  let ret = CefClient::from_cef(_self, true).get().get_download_handler();
  ret.map_or(std::ptr::null_mut(), |o| crate::include::CefDownloadHandler::to_cef_own(o))
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_client_t_get_drag_handler(_self: *mut cef_sys::cef_client_t) -> *mut cef_sys::cef_drag_handler_t {
  let ret = CefClient::from_cef(_self, true).get().get_drag_handler();
  ret.map_or(std::ptr::null_mut(), |o| crate::include::CefDragHandler::to_cef_own(o))
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_client_t_get_find_handler(_self: *mut cef_sys::cef_client_t) -> *mut cef_sys::cef_find_handler_t {
  let ret = CefClient::from_cef(_self, true).get().get_find_handler();
  ret.map_or(std::ptr::null_mut(), |o| crate::include::CefFindHandler::to_cef_own(o))
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_client_t_get_focus_handler(_self: *mut cef_sys::cef_client_t) -> *mut cef_sys::cef_focus_handler_t {
  let ret = CefClient::from_cef(_self, true).get().get_focus_handler();
  ret.map_or(std::ptr::null_mut(), |o| crate::include::CefFocusHandler::to_cef_own(o))
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_client_t_get_jsdialog_handler(_self: *mut cef_sys::cef_client_t) -> *mut cef_sys::cef_jsdialog_handler_t {
  let ret = CefClient::from_cef(_self, true).get().get_jsdialog_handler();
  ret.map_or(std::ptr::null_mut(), |o| crate::include::CefJSDialogHandler::to_cef_own(o))
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_client_t_get_keyboard_handler(_self: *mut cef_sys::cef_client_t) -> *mut cef_sys::cef_keyboard_handler_t {
  let ret = CefClient::from_cef(_self, true).get().get_keyboard_handler();
  ret.map_or(std::ptr::null_mut(), |o| crate::include::CefKeyboardHandler::to_cef_own(o))
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_client_t_get_life_span_handler(_self: *mut cef_sys::cef_client_t) -> *mut cef_sys::cef_life_span_handler_t {
  let ret = CefClient::from_cef(_self, true).get().get_life_span_handler();
  ret.map_or(std::ptr::null_mut(), |o| crate::include::CefLifeSpanHandler::to_cef_own(o))
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_client_t_get_load_handler(_self: *mut cef_sys::cef_client_t) -> *mut cef_sys::cef_load_handler_t {
  let ret = CefClient::from_cef(_self, true).get().get_load_handler();
  ret.map_or(std::ptr::null_mut(), |o| crate::include::CefLoadHandler::to_cef_own(o))
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_client_t_get_render_handler(_self: *mut cef_sys::cef_client_t) -> *mut cef_sys::cef_render_handler_t {
  let ret = CefClient::from_cef(_self, true).get().get_render_handler();
  ret.map_or(std::ptr::null_mut(), |o| crate::include::CefRenderHandler::to_cef_own(o))
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_client_t_get_request_handler(_self: *mut cef_sys::cef_client_t) -> *mut cef_sys::cef_request_handler_t {
  let ret = CefClient::from_cef(_self, true).get().get_request_handler();
  ret.map_or(std::ptr::null_mut(), |o| crate::include::CefRequestHandler::to_cef_own(o))
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_client_t_on_process_message_received(_self: *mut cef_sys::cef_client_t, browser: *mut cef_sys::cef_browser_t, frame: *mut cef_sys::cef_frame_t, source_process: cef_sys::cef_process_id_t, message: *mut cef_sys::cef_process_message_t) -> i32 {
  let ret = CefClient::from_cef(_self, true).get().on_process_message_received(crate::include::CefBrowser::from_cef_own(browser).unwrap(),crate::include::CefFrame::from_cef_own(frame).unwrap(),source_process.into(),crate::include::CefProcessMessage::from_cef_own(message).unwrap(),);
  if ret { 1 } else { 0 }
}
