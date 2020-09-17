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
  /// Called whenever draggable regions for the browser window change. These can
  /// be specified using the '-webkit-app-region: drag/no-drag' CSS-property. If
  /// draggable regions are never defined in a document this method will also
  /// never be called. If the last draggable region is removed from a document
  /// this method will be called with an empty vector.
  fn on_draggable_regions_changed(&mut self, browser: crate::include::CefBrowser, frame: crate::include::CefFrame, regions: &[crate::include::internal::CefDraggableRegion]) -> () { Default::default() }
}
define_refcounted!(DragHandler, CefDragHandler, cef_drag_handler_t, on_drag_enter: cef_drag_handler_t_on_drag_enter,on_draggable_regions_changed: cef_drag_handler_t_on_draggable_regions_changed,);
#[allow(non_snake_case)]
unsafe extern "C" fn cef_drag_handler_t_on_drag_enter(_self: *mut cef_sys::cef_drag_handler_t, browser: *mut cef_sys::cef_browser_t, dragData: *mut cef_sys::cef_drag_data_t, mask: cef_sys::cef_drag_operations_mask_t) -> i32 {
  let ret = CefDragHandler::from_cef(_self, true).get().on_drag_enter(crate::include::CefBrowser::from_cef_own(browser).unwrap(),crate::include::CefDragData::from_cef_own(dragData).unwrap(),mask.into(),);
  if ret { 1 } else { 0 }
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_drag_handler_t_on_draggable_regions_changed(_self: *mut cef_sys::cef_drag_handler_t, browser: *mut cef_sys::cef_browser_t, frame: *mut cef_sys::cef_frame_t, regions0: u64, regions1: *const cef_sys::cef_draggable_region_t) -> () {
  let ret = CefDragHandler::from_cef(_self, true).get().on_draggable_regions_changed(crate::include::CefBrowser::from_cef_own(browser).unwrap(),crate::include::CefFrame::from_cef_own(frame).unwrap(),std::slice::from_raw_parts(regions0 as *const _, regions1 as _),);
  ret
}
