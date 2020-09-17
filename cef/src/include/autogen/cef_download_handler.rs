pub type CefBeforeDownloadCallback = crate::include::base::CefProxy<cef_sys::cef_before_download_callback_t>;
#[allow(non_snake_case)]
impl CefBeforeDownloadCallback {
  /// Call to continue the download. Set |download_path| to the full file path
  /// for the download including the file name or leave blank to use the
  /// suggested name and the default temp directory. Set |show_dialog| to true
  /// if you do wish to show the default "Save As" dialog.
  pub fn cont(&mut self, download_path: Option<&crate::include::internal::CefString>, show_dialog: bool) -> () {
    unsafe {
      let ret = match self.raw.as_ref().cont {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(download_path),if show_dialog { 1 } else { 0 },),
        None => panic!(),
      };
      ret
    }
  }
}
pub type CefDownloadItemCallback = crate::include::base::CefProxy<cef_sys::cef_download_item_callback_t>;
#[allow(non_snake_case)]
impl CefDownloadItemCallback {
  /// Call to cancel the download.
  pub fn cancel(&mut self) -> () {
    unsafe {
      let ret = match self.raw.as_ref().cancel {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  /// Call to pause the download.
  pub fn pause(&mut self) -> () {
    unsafe {
      let ret = match self.raw.as_ref().pause {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  /// Call to resume the download.
  pub fn resume(&mut self) -> () {
    unsafe {
      let ret = match self.raw.as_ref().resume {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
}
/// Class used to handle file downloads. The methods of this class will called
/// on the browser process UI thread.
#[allow(non_snake_case)]
#[allow(unused_variables)]
pub trait DownloadHandler {
  /// Called before a download begins. |suggested_name| is the suggested name for
  /// the download file. By default the download will be canceled. Execute
  /// |callback| either asynchronously or in this method to continue the download
  /// if desired. Do not keep a reference to |download_item| outside of this
  /// method.
  fn on_before_download(&mut self, browser: crate::include::CefBrowser, download_item: crate::include::CefDownloadItem, suggested_name: &crate::include::internal::CefString, callback: crate::include::CefBeforeDownloadCallback) -> () { Default::default() }
  /// Called when a download's status or progress information has been updated.
  /// This may be called multiple times before and after OnBeforeDownload().
  /// Execute |callback| either asynchronously or in this method to cancel the
  /// download if desired. Do not keep a reference to |download_item| outside of
  /// this method.
  fn on_download_updated(&mut self, browser: crate::include::CefBrowser, download_item: crate::include::CefDownloadItem, callback: crate::include::CefDownloadItemCallback) -> () { Default::default() }
}
define_refcounted!(DownloadHandler, CefDownloadHandler, cef_download_handler_t, on_before_download: cef_download_handler_t_on_before_download,on_download_updated: cef_download_handler_t_on_download_updated,);
#[allow(non_snake_case)]
unsafe extern "C" fn cef_download_handler_t_on_before_download(_self: *mut cef_sys::cef_download_handler_t, browser: *mut cef_sys::cef_browser_t, download_item: *mut cef_sys::cef_download_item_t, suggested_name: *const cef_sys::cef_string_t, callback: *mut cef_sys::cef_before_download_callback_t) -> () {
  let ret = CefDownloadHandler::from_cef(_self, true).get().on_before_download(crate::include::CefBrowser::from_cef_own(browser).unwrap(),crate::include::CefDownloadItem::from_cef_own(download_item).unwrap(),&crate::include::internal::CefString::from_cef(suggested_name).unwrap(),crate::include::CefBeforeDownloadCallback::from_cef_own(callback).unwrap(),);
  ret
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_download_handler_t_on_download_updated(_self: *mut cef_sys::cef_download_handler_t, browser: *mut cef_sys::cef_browser_t, download_item: *mut cef_sys::cef_download_item_t, callback: *mut cef_sys::cef_download_item_callback_t) -> () {
  let ret = CefDownloadHandler::from_cef(_self, true).get().on_download_updated(crate::include::CefBrowser::from_cef_own(browser).unwrap(),crate::include::CefDownloadItem::from_cef_own(download_item).unwrap(),crate::include::CefDownloadItemCallback::from_cef_own(callback).unwrap(),);
  ret
}
