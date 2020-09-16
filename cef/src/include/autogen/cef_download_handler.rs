pub type CefBeforeDownloadCallback = crate::include::base::CefProxy<cef_sys::cef_before_download_callback_t>;
#[allow(non_snake_case)]
impl CefBeforeDownloadCallback {
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
  pub fn cancel(&mut self) -> () {
    unsafe {
      let ret = match self.raw.as_ref().cancel {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn pause(&mut self) -> () {
    unsafe {
      let ret = match self.raw.as_ref().pause {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
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
pub trait DownloadHandler {
  fn on_before_download(&mut self, browser: crate::include::CefBrowser, download_item: crate::include::CefDownloadItem, suggested_name: &crate::include::internal::CefString, callback: crate::include::CefBeforeDownloadCallback) -> () { Default::default() }
  fn on_download_updated(&mut self, browser: crate::include::CefBrowser, download_item: crate::include::CefDownloadItem, callback: crate::include::CefDownloadItemCallback) -> () { Default::default() }
}
define_refcounted!(DownloadHandler, download_handler, on_before_download,on_download_updated,);
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
