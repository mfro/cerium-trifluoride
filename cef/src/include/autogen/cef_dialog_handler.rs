pub type CefFileDialogCallback = crate::include::base::CefProxy<cef_sys::cef_file_dialog_callback_t>;
#[allow(non_snake_case)]
impl CefFileDialogCallback {
  /// Continue the file selection. |selected_accept_filter| should be the 0-based
  /// index of the value selected from the accept filters array passed to
  /// CefDialogHandler::OnFileDialog. |file_paths| should be a single value or a
  /// list of values depending on the dialog mode. An empty |file_paths| value is
  /// treated the same as calling Cancel().
  pub fn cont(&mut self, selected_accept_filter: i32, file_paths: &crate::include::internal::CefStringList) -> () {
    unsafe {
      let ret = match self.raw.as_ref().cont {
        Some(f) => f(self.raw.as_ptr(),selected_accept_filter,file_paths.into(),),
        None => panic!(),
      };
      ret
    }
  }
  /// Cancel the file selection.
  pub fn cancel(&mut self) -> () {
    unsafe {
      let ret = match self.raw.as_ref().cancel {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
}
/// Implement this interface to handle dialog events. The methods of this class
/// will be called on the browser process UI thread.
#[allow(non_snake_case)]
#[allow(unused_variables)]
pub trait DialogHandler {
  /// Called to run a file chooser dialog. |mode| represents the type of dialog
  /// to display. |title| to the title to be used for the dialog and may be empty
  /// to show the default title ("Open" or "Save" depending on the mode).
  /// |default_file_path| is the path with optional directory and/or file name
  /// component that should be initially selected in the dialog. |accept_filters|
  /// are used to restrict the selectable file types and may any combination of
  /// (a) valid lower-cased MIME types (e.g. "text/*" or "image/*"),
  /// (b) individual file extensions (e.g. ".txt" or ".png"), or (c) combined
  /// description and file extension delimited using "|" and ";" (e.g.
  /// "Image Types|.png;.gif;.jpg"). |selected_accept_filter| is the 0-based
  /// index of the filter that should be selected by default. To display a custom
  /// dialog return true and execute |callback| either inline or at a later time.
  /// To display the default dialog return false.
  fn on_file_dialog(&mut self, browser: crate::include::CefBrowser, mode: crate::include::internal::CefFileDialogMode, title: Option<&crate::include::internal::CefString>, default_file_path: Option<&crate::include::internal::CefString>, accept_filters: &crate::include::internal::CefStringList, selected_accept_filter: i32, callback: crate::include::CefFileDialogCallback) -> bool { Default::default() }
}
define_refcounted!(DialogHandler, CefDialogHandler, cef_dialog_handler_t, on_file_dialog: cef_dialog_handler_t_on_file_dialog,);
#[allow(non_snake_case)]
unsafe extern "C" fn cef_dialog_handler_t_on_file_dialog(_self: *mut cef_sys::cef_dialog_handler_t, browser: *mut cef_sys::cef_browser_t, mode: cef_sys::cef_file_dialog_mode_t, title: *const cef_sys::cef_string_t, default_file_path: *const cef_sys::cef_string_t, accept_filters: cef_sys::cef_string_list_t, selected_accept_filter: i32, callback: *mut cef_sys::cef_file_dialog_callback_t) -> i32 {
  let ret = CefDialogHandler::from_cef(_self, true).get().on_file_dialog(crate::include::CefBrowser::from_cef_own(browser).unwrap(),mode.into(),match &crate::include::internal::CefString::from_cef(title) { Some(ref x) => Some(x), None => None },match &crate::include::internal::CefString::from_cef(default_file_path) { Some(ref x) => Some(x), None => None },&accept_filters.into(),selected_accept_filter,crate::include::CefFileDialogCallback::from_cef_own(callback).unwrap(),);
  if ret { 1 } else { 0 }
}
