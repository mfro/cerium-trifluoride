/// Implement this interface to handle events when window rendering is disabled.
/// The methods of this class will be called on the UI thread.
#[allow(non_snake_case)]
pub trait RenderHandler {
  fn get_accessibility_handler(&mut self) -> Option<crate::include::CefAccessibilityHandler> { Default::default() }
  fn get_root_screen_rect(&mut self, browser: crate::include::CefBrowser, rect: &mut crate::include::internal::CefRect) -> bool { Default::default() }
  fn get_view_rect(&mut self, browser: crate::include::CefBrowser, rect: &mut crate::include::internal::CefRect) -> () { Default::default() }
  fn get_screen_point(&mut self, browser: crate::include::CefBrowser, viewX: i32, viewY: i32, screenX: &mut i32, screenY: &mut i32) -> bool { Default::default() }
  fn get_screen_info(&mut self, browser: crate::include::CefBrowser, screen_info: &mut crate::include::internal::CefScreenInfo) -> bool { Default::default() }
  fn on_popup_show(&mut self, browser: crate::include::CefBrowser, show: bool) -> () { Default::default() }
  fn on_popup_size(&mut self, browser: crate::include::CefBrowser, rect: &crate::include::internal::CefRect) -> () { Default::default() }
  fn on_cursor_change(&mut self, browser: crate::include::CefBrowser, cursor: crate::include::internal::CefCursorHandle, type_: crate::include::internal::CefCursorType, custom_cursor_info: &crate::include::internal::CefCursorInfo) -> () { Default::default() }
  fn start_dragging(&mut self, browser: crate::include::CefBrowser, drag_data: crate::include::CefDragData, allowed_ops: crate::include::internal::CefDragOperationsMask, x: i32, y: i32) -> bool { Default::default() }
  fn update_drag_cursor(&mut self, browser: crate::include::CefBrowser, operation: crate::include::internal::CefDragOperationsMask) -> () { Default::default() }
  fn on_scroll_offset_changed(&mut self, browser: crate::include::CefBrowser, x: f64, y: f64) -> () { Default::default() }
  fn on_text_selection_changed(&mut self, browser: crate::include::CefBrowser, selected_text: &crate::include::internal::CefString, selected_range: &crate::include::internal::CefRange) -> () { Default::default() }
  fn on_virtual_keyboard_requested(&mut self, browser: crate::include::CefBrowser, input_mode: crate::include::internal::CefTextInputMode) -> () { Default::default() }
}
define_refcounted!(RenderHandler, render_handler, get_accessibility_handler,get_root_screen_rect,get_view_rect,get_screen_point,get_screen_info,on_popup_show,on_popup_size,on_cursor_change,start_dragging,update_drag_cursor,on_scroll_offset_changed,on_text_selection_changed,on_virtual_keyboard_requested,);
#[allow(non_snake_case)]
unsafe extern "C" fn cef_render_handler_t_get_accessibility_handler(_self: *mut cef_sys::cef_render_handler_t) -> *mut cef_sys::cef_accessibility_handler_t {
  let ret = CefRenderHandler::from_cef(_self, true).get().get_accessibility_handler();
  ret.map_or(std::ptr::null_mut(), |o| crate::include::CefAccessibilityHandler::to_cef_own(o))
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_render_handler_t_get_root_screen_rect(_self: *mut cef_sys::cef_render_handler_t, browser: *mut cef_sys::cef_browser_t, rect: *mut cef_sys::cef_rect_t) -> i32 {
  let ret = CefRenderHandler::from_cef(_self, true).get().get_root_screen_rect(crate::include::CefBrowser::from_cef_own(browser).unwrap(),&mut *(rect as *mut _),);
  if ret { 1 } else { 0 }
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_render_handler_t_get_view_rect(_self: *mut cef_sys::cef_render_handler_t, browser: *mut cef_sys::cef_browser_t, rect: *mut cef_sys::cef_rect_t) -> () {
  let ret = CefRenderHandler::from_cef(_self, true).get().get_view_rect(crate::include::CefBrowser::from_cef_own(browser).unwrap(),&mut *(rect as *mut _),);
  ret
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_render_handler_t_get_screen_point(_self: *mut cef_sys::cef_render_handler_t, browser: *mut cef_sys::cef_browser_t, viewX: i32, viewY: i32, screenX: *mut i32, screenY: *mut i32) -> i32 {
  let ret = CefRenderHandler::from_cef(_self, true).get().get_screen_point(crate::include::CefBrowser::from_cef_own(browser).unwrap(),viewX,viewY,&mut *screenX,&mut *screenY,);
  if ret { 1 } else { 0 }
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_render_handler_t_get_screen_info(_self: *mut cef_sys::cef_render_handler_t, browser: *mut cef_sys::cef_browser_t, screen_info: *mut cef_sys::cef_screen_info_t) -> i32 {
  let ret = CefRenderHandler::from_cef(_self, true).get().get_screen_info(crate::include::CefBrowser::from_cef_own(browser).unwrap(),&mut *(screen_info as *mut _),);
  if ret { 1 } else { 0 }
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_render_handler_t_on_popup_show(_self: *mut cef_sys::cef_render_handler_t, browser: *mut cef_sys::cef_browser_t, show: i32) -> () {
  let ret = CefRenderHandler::from_cef(_self, true).get().on_popup_show(crate::include::CefBrowser::from_cef_own(browser).unwrap(),if show == 0 { false } else { true },);
  ret
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_render_handler_t_on_popup_size(_self: *mut cef_sys::cef_render_handler_t, browser: *mut cef_sys::cef_browser_t, rect: *const cef_sys::cef_rect_t) -> () {
  let ret = CefRenderHandler::from_cef(_self, true).get().on_popup_size(crate::include::CefBrowser::from_cef_own(browser).unwrap(),&*(rect as *const _),);
  ret
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_render_handler_t_on_cursor_change(_self: *mut cef_sys::cef_render_handler_t, browser: *mut cef_sys::cef_browser_t, cursor: cef_sys::cef_cursor_handle_t, type_: cef_sys::cef_cursor_type_t, custom_cursor_info: *const cef_sys::cef_cursor_info_t) -> () {
  let ret = CefRenderHandler::from_cef(_self, true).get().on_cursor_change(crate::include::CefBrowser::from_cef_own(browser).unwrap(),cursor.into(),type_.into(),&*(custom_cursor_info as *const _),);
  ret
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_render_handler_t_start_dragging(_self: *mut cef_sys::cef_render_handler_t, browser: *mut cef_sys::cef_browser_t, drag_data: *mut cef_sys::cef_drag_data_t, allowed_ops: cef_sys::cef_drag_operations_mask_t, x: i32, y: i32) -> i32 {
  let ret = CefRenderHandler::from_cef(_self, true).get().start_dragging(crate::include::CefBrowser::from_cef_own(browser).unwrap(),crate::include::CefDragData::from_cef_own(drag_data).unwrap(),allowed_ops.into(),x,y,);
  if ret { 1 } else { 0 }
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_render_handler_t_update_drag_cursor(_self: *mut cef_sys::cef_render_handler_t, browser: *mut cef_sys::cef_browser_t, operation: cef_sys::cef_drag_operations_mask_t) -> () {
  let ret = CefRenderHandler::from_cef(_self, true).get().update_drag_cursor(crate::include::CefBrowser::from_cef_own(browser).unwrap(),operation.into(),);
  ret
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_render_handler_t_on_scroll_offset_changed(_self: *mut cef_sys::cef_render_handler_t, browser: *mut cef_sys::cef_browser_t, x: f64, y: f64) -> () {
  let ret = CefRenderHandler::from_cef(_self, true).get().on_scroll_offset_changed(crate::include::CefBrowser::from_cef_own(browser).unwrap(),x,y,);
  ret
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_render_handler_t_on_text_selection_changed(_self: *mut cef_sys::cef_render_handler_t, browser: *mut cef_sys::cef_browser_t, selected_text: *const cef_sys::cef_string_t, selected_range: *const cef_sys::cef_range_t) -> () {
  let ret = CefRenderHandler::from_cef(_self, true).get().on_text_selection_changed(crate::include::CefBrowser::from_cef_own(browser).unwrap(),&crate::include::internal::CefString::from_cef(selected_text).unwrap(),&*(selected_range as *const _),);
  ret
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_render_handler_t_on_virtual_keyboard_requested(_self: *mut cef_sys::cef_render_handler_t, browser: *mut cef_sys::cef_browser_t, input_mode: cef_sys::cef_text_input_mode_t) -> () {
  let ret = CefRenderHandler::from_cef(_self, true).get().on_virtual_keyboard_requested(crate::include::CefBrowser::from_cef_own(browser).unwrap(),input_mode.into(),);
  ret
}
