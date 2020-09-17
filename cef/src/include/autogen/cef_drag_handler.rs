/// Implement this interface to handle events related to dragging. The methods of
/// this class will be called on the UI thread.
#[allow(non_snake_case)]
#[allow(unused_variables)]
pub trait DragHandler {
  /// Called when an external drag event enters the browser window. |dragData|
  /// contains the drag event data and |mask| represents the type of drag
  /// operation. Return false for default drag handling behavior or true to
  /// cancel the drag event.
  fn on_drag_enter(&mut self, browser: crate::include::CefBrowser, dragData: crate::include::CefDragData, mask: crate::include::internal::CefDragOperationsMask) -> bool { Default::default() }
}
define_refcounted!(DragHandler, CefDragHandler, cef_drag_handler_t, on_drag_enter: cef_drag_handler_t_on_drag_enter,);
#[allow(non_snake_case)]
unsafe extern "C" fn cef_drag_handler_t_on_drag_enter(_self: *mut cef_sys::cef_drag_handler_t, browser: *mut cef_sys::cef_browser_t, dragData: *mut cef_sys::cef_drag_data_t, mask: cef_sys::cef_drag_operations_mask_t) -> i32 {
  let ret = CefDragHandler::from_cef(_self, true).get().on_drag_enter(crate::include::CefBrowser::from_cef_own(browser).unwrap(),crate::include::CefDragData::from_cef_own(dragData).unwrap(),mask.into(),);
  if ret { 1 } else { 0 }
}
