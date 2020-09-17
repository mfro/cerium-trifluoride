/// Implement this interface to handle events when window rendering is disabled.
/// The methods of this class will be called on the UI thread.
#[allow(non_snake_case)]
#[allow(unused_variables)]
pub trait RenderHandler {
  /// Return the handler for accessibility notifications. If no handler is
  /// provided the default implementation will be used.
  fn get_accessibility_handler(&mut self) -> Option<crate::include::CefAccessibilityHandler> { Default::default() }
  /// Called to retrieve the root window rectangle in screen coordinates. Return
  /// true if the rectangle was provided. If this method returns false the
  /// rectangle from GetViewRect will be used.
  fn get_root_screen_rect(&mut self, browser: crate::include::CefBrowser, rect: &mut crate::include::internal::CefRect) -> bool { Default::default() }
  /// Called to retrieve the view rectangle which is relative to screen
  /// coordinates. This method must always provide a non-empty rectangle.
  fn get_view_rect(&mut self, browser: crate::include::CefBrowser, rect: &mut crate::include::internal::CefRect) -> () { Default::default() }
  /// Called to retrieve the translation from view coordinates to actual screen
  /// coordinates. Return true if the screen coordinates were provided.
  fn get_screen_point(&mut self, browser: crate::include::CefBrowser, viewX: i32, viewY: i32, screenX: &mut i32, screenY: &mut i32) -> bool { Default::default() }
  /// Called to allow the client to fill in the CefScreenInfo object with
  /// appropriate values. Return true if the |screen_info| structure has been
  /// modified.
  /// 
  /// If the screen info rectangle is left empty the rectangle from GetViewRect
  /// will be used. If the rectangle is still empty or invalid popups may not be
  /// drawn correctly.
  fn get_screen_info(&mut self, browser: crate::include::CefBrowser, screen_info: &mut crate::include::internal::CefScreenInfo) -> bool { Default::default() }
  /// Called when the browser wants to show or hide the popup widget. The popup
  /// should be shown if |show| is true and hidden if |show| is false.
  fn on_popup_show(&mut self, browser: crate::include::CefBrowser, show: bool) -> () { Default::default() }
  /// Called when the browser wants to move or resize the popup widget. |rect|
  /// contains the new location and size in view coordinates.
  fn on_popup_size(&mut self, browser: crate::include::CefBrowser, rect: &crate::include::internal::CefRect) -> () { Default::default() }
  /// Called when the browser's cursor has changed. If |type| is CT_CUSTOM then
  /// |custom_cursor_info| will be populated with the custom cursor information.
  fn on_cursor_change(&mut self, browser: crate::include::CefBrowser, cursor: crate::include::internal::CefCursorHandle, type_: crate::include::internal::CefCursorType, custom_cursor_info: &crate::include::internal::CefCursorInfo) -> () { Default::default() }
  /// Called when the user starts dragging content in the web view. Contextual
  /// information about the dragged content is supplied by |drag_data|.
  /// (|x|, |y|) is the drag start location in screen coordinates.
  /// OS APIs that run a system message loop may be used within the
  /// StartDragging call.
  /// 
  /// Return false to abort the drag operation. Don't call any of
  /// CefBrowserHost::DragSource*Ended* methods after returning false.
  /// 
  /// Return true to handle the drag operation. Call
  /// CefBrowserHost::DragSourceEndedAt and DragSourceSystemDragEnded either
  /// synchronously or asynchronously to inform the web view that the drag
  /// operation has ended.
  fn start_dragging(&mut self, browser: crate::include::CefBrowser, drag_data: crate::include::CefDragData, allowed_ops: crate::include::internal::CefDragOperationsMask, x: i32, y: i32) -> bool { Default::default() }
  /// Called when the web view wants to update the mouse cursor during a
  /// drag & drop operation. |operation| describes the allowed operation
  /// (none, move, copy, link).
  fn update_drag_cursor(&mut self, browser: crate::include::CefBrowser, operation: crate::include::internal::CefDragOperationsMask) -> () { Default::default() }
  /// Called when the scroll offset has changed.
  fn on_scroll_offset_changed(&mut self, browser: crate::include::CefBrowser, x: f64, y: f64) -> () { Default::default() }
  /// Called when text selection has changed for the specified |browser|.
  /// |selected_text| is the currently selected text and |selected_range| is
  /// the character range.
  fn on_text_selection_changed(&mut self, browser: crate::include::CefBrowser, selected_text: Option<&crate::include::internal::CefString>, selected_range: Option<&crate::include::internal::CefRange>) -> () { Default::default() }
  /// Called when an on-screen keyboard should be shown or hidden for the
  /// specified |browser|. |input_mode| specifies what kind of keyboard
  /// should be opened. If |input_mode| is CEF_TEXT_INPUT_MODE_NONE, any
  /// existing keyboard for this browser should be hidden.
  fn on_virtual_keyboard_requested(&mut self, browser: crate::include::CefBrowser, input_mode: crate::include::internal::CefTextInputMode) -> () { Default::default() }
}
define_refcounted!(RenderHandler, CefRenderHandler, cef_render_handler_t, get_accessibility_handler: cef_render_handler_t_get_accessibility_handler,get_root_screen_rect: cef_render_handler_t_get_root_screen_rect,get_view_rect: cef_render_handler_t_get_view_rect,get_screen_point: cef_render_handler_t_get_screen_point,get_screen_info: cef_render_handler_t_get_screen_info,on_popup_show: cef_render_handler_t_on_popup_show,on_popup_size: cef_render_handler_t_on_popup_size,on_cursor_change: cef_render_handler_t_on_cursor_change,start_dragging: cef_render_handler_t_start_dragging,update_drag_cursor: cef_render_handler_t_update_drag_cursor,on_scroll_offset_changed: cef_render_handler_t_on_scroll_offset_changed,on_text_selection_changed: cef_render_handler_t_on_text_selection_changed,on_virtual_keyboard_requested: cef_render_handler_t_on_virtual_keyboard_requested,);
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
  let ret = CefRenderHandler::from_cef(_self, true).get().on_text_selection_changed(crate::include::CefBrowser::from_cef_own(browser).unwrap(),match &crate::include::internal::CefString::from_cef(selected_text) { Some(ref x) => Some(x), None => None },if selected_range.is_null() { None } else { Some(&*(selected_range as *const _)) },);
  ret
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_render_handler_t_on_virtual_keyboard_requested(_self: *mut cef_sys::cef_render_handler_t, browser: *mut cef_sys::cef_browser_t, input_mode: cef_sys::cef_text_input_mode_t) -> () {
  let ret = CefRenderHandler::from_cef(_self, true).get().on_virtual_keyboard_requested(crate::include::CefBrowser::from_cef_own(browser).unwrap(),input_mode.into(),);
  ret
}
