/// Implement this interface to handle events related to find results. The
/// methods of this class will be called on the UI thread.
#[allow(non_snake_case)]
pub trait FindHandler {
  fn on_find_result(&mut self, browser: crate::include::CefBrowser, identifier: i32, count: i32, selectionRect: &crate::include::internal::CefRect, activeMatchOrdinal: i32, finalUpdate: bool) -> () { Default::default() }
}
define_refcounted!(FindHandler, find_handler, on_find_result,);
#[allow(non_snake_case)]
unsafe extern "C" fn cef_find_handler_t_on_find_result(_self: *mut cef_sys::cef_find_handler_t, browser: *mut cef_sys::cef_browser_t, identifier: i32, count: i32, selectionRect: *const cef_sys::cef_rect_t, activeMatchOrdinal: i32, finalUpdate: i32) -> () {
  let ret = CefFindHandler::from_cef(_self, true).get().on_find_result(crate::include::CefBrowser::from_cef_own(browser).unwrap(),identifier,count,&*(selectionRect as *const _),activeMatchOrdinal,if finalUpdate == 0 { false } else { true },);
  ret
}
