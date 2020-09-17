/// Implement this interface to handle events related to find results. The
/// methods of this class will be called on the UI thread.
#[allow(non_snake_case)]
#[allow(unused_variables)]
pub trait FindHandler {
  /// Called to report find results returned by CefBrowserHost::Find().
  /// |identifer| is the identifier passed to Find(), |count| is the number of
  /// matches currently identified, |selectionRect| is the location of where the
  /// match was found (in window coordinates), |activeMatchOrdinal| is the
  /// current position in the search results, and |finalUpdate| is true if this
  /// is the last find notification.
  fn on_find_result(&mut self, browser: crate::include::CefBrowser, identifier: i32, count: i32, selectionRect: &crate::include::internal::CefRect, activeMatchOrdinal: i32, finalUpdate: bool) -> () { Default::default() }
}
define_refcounted!(FindHandler, CefFindHandler, cef_find_handler_t, on_find_result: cef_find_handler_t_on_find_result,);
#[allow(non_snake_case)]
unsafe extern "C" fn cef_find_handler_t_on_find_result(_self: *mut cef_sys::cef_find_handler_t, browser: *mut cef_sys::cef_browser_t, identifier: i32, count: i32, selectionRect: *const cef_sys::cef_rect_t, activeMatchOrdinal: i32, finalUpdate: i32) -> () {
  let ret = CefFindHandler::from_cef(_self, true).get().on_find_result(crate::include::CefBrowser::from_cef_own(browser).unwrap(),identifier,count,&*(selectionRect as *const _),activeMatchOrdinal,if finalUpdate == 0 { false } else { true },);
  ret
}
