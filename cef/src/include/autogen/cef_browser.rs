pub type CefBrowser = crate::include::base::CefProxy<cef_sys::cef_browser_t>;
#[allow(non_snake_case)]
impl CefBrowser {
  /// Returns the browser host object. This method can only be called in the
  /// browser process.
  pub fn get_host(&mut self) -> Option<crate::include::CefBrowserHost> {
    unsafe {
      let ret = match self.raw.as_ref().get_host {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefBrowserHost::from_cef_own(ret)
    }
  }
  /// Returns true if the browser can navigate backwards.
  pub fn can_go_back(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().can_go_back {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Navigate backwards.
  pub fn go_back(&mut self) -> () {
    unsafe {
      let ret = match self.raw.as_ref().go_back {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  /// Returns true if the browser can navigate forwards.
  pub fn can_go_forward(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().can_go_forward {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Navigate forwards.
  pub fn go_forward(&mut self) -> () {
    unsafe {
      let ret = match self.raw.as_ref().go_forward {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  /// Returns true if the browser is currently loading.
  pub fn is_loading(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_loading {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Reload the current page.
  pub fn reload(&mut self) -> () {
    unsafe {
      let ret = match self.raw.as_ref().reload {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  /// Reload the current page ignoring any cached data.
  pub fn reload_ignore_cache(&mut self) -> () {
    unsafe {
      let ret = match self.raw.as_ref().reload_ignore_cache {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  /// Stop loading the page.
  pub fn stop_load(&mut self) -> () {
    unsafe {
      let ret = match self.raw.as_ref().stop_load {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  /// Returns the globally unique identifier for this browser. This value is also
  /// used as the tabId for extension APIs.
  pub fn get_identifier(&mut self) -> i32 {
    unsafe {
      let ret = match self.raw.as_ref().get_identifier {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  /// Returns true if this object is pointing to the same handle as |that|
  /// object.
  pub fn is_same(&mut self, that: crate::include::CefBrowser) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_same {
        Some(f) => f(self.raw.as_ptr(),crate::include::CefBrowser::to_cef_own(that),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns true if the window is a popup window.
  pub fn is_popup(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_popup {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns true if a document has been loaded in the browser.
  pub fn has_document(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().has_document {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns the main (top-level) frame for the browser window.
  pub fn get_main_frame(&mut self) -> Option<crate::include::CefFrame> {
    unsafe {
      let ret = match self.raw.as_ref().get_main_frame {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefFrame::from_cef_own(ret)
    }
  }
  /// Returns the focused frame for the browser window.
  pub fn get_focused_frame(&mut self) -> Option<crate::include::CefFrame> {
    unsafe {
      let ret = match self.raw.as_ref().get_focused_frame {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefFrame::from_cef_own(ret)
    }
  }
  /// Returns the frame with the specified identifier, or NULL if not found.
  pub fn get_frame_byident(&mut self, identifier: i64) -> Option<crate::include::CefFrame> {
    unsafe {
      let ret = match self.raw.as_ref().get_frame_byident {
        Some(f) => f(self.raw.as_ptr(),identifier,),
        None => panic!(),
      };
      crate::include::CefFrame::from_cef_own(ret)
    }
  }
  /// Returns the frame with the specified name, or NULL if not found.
  pub fn get_frame(&mut self, name: Option<&crate::include::internal::CefString>) -> Option<crate::include::CefFrame> {
    unsafe {
      let ret = match self.raw.as_ref().get_frame {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(name),),
        None => panic!(),
      };
      crate::include::CefFrame::from_cef_own(ret)
    }
  }
  /// Returns the number of frames that currently exist.
  pub fn get_frame_count(&mut self) -> u64 {
    unsafe {
      let ret = match self.raw.as_ref().get_frame_count {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
}
/// Callback interface for CefBrowserHost::RunFileDialog. The methods of this
/// class will be called on the browser process UI thread.
#[allow(non_snake_case)]
#[allow(unused_variables)]
pub trait RunFileDialogCallback {
  /// Called asynchronously after the file dialog is dismissed.
  /// |selected_accept_filter| is the 0-based index of the value selected from
  /// the accept filters array passed to CefBrowserHost::RunFileDialog.
  /// |file_paths| will be a single value or a list of values depending on the
  /// dialog mode. If the selection was cancelled |file_paths| will be empty.
  fn on_file_dialog_dismissed(&mut self, selected_accept_filter: i32, file_paths: &crate::include::internal::CefStringList) -> () { Default::default() }
}
define_refcounted!(RunFileDialogCallback, CefRunFileDialogCallback, cef_run_file_dialog_callback_t, on_file_dialog_dismissed: cef_run_file_dialog_callback_t_on_file_dialog_dismissed,);
#[allow(non_snake_case)]
unsafe extern "C" fn cef_run_file_dialog_callback_t_on_file_dialog_dismissed(_self: *mut cef_sys::cef_run_file_dialog_callback_t, selected_accept_filter: i32, file_paths: cef_sys::cef_string_list_t) -> () {
  let ret = CefRunFileDialogCallback::from_cef(_self, true).get().on_file_dialog_dismissed(selected_accept_filter,&file_paths.into(),);
  ret
}
/// Callback interface for CefBrowserHost::GetNavigationEntries. The methods of
/// this class will be called on the browser process UI thread.
#[allow(non_snake_case)]
#[allow(unused_variables)]
pub trait NavigationEntryVisitor {
  /// Method that will be executed. Do not keep a reference to |entry| outside of
  /// this callback. Return true to continue visiting entries or false to stop.
  /// |current| is true if this entry is the currently loaded navigation entry.
  /// |index| is the 0-based index of this entry and |total| is the total number
  /// of entries.
  fn visit(&mut self, entry: crate::include::CefNavigationEntry, current: bool, index: i32, total: i32) -> bool { Default::default() }
}
define_refcounted!(NavigationEntryVisitor, CefNavigationEntryVisitor, cef_navigation_entry_visitor_t, visit: cef_navigation_entry_visitor_t_visit,);
#[allow(non_snake_case)]
unsafe extern "C" fn cef_navigation_entry_visitor_t_visit(_self: *mut cef_sys::cef_navigation_entry_visitor_t, entry: *mut cef_sys::cef_navigation_entry_t, current: i32, index: i32, total: i32) -> i32 {
  let ret = CefNavigationEntryVisitor::from_cef(_self, true).get().visit(crate::include::CefNavigationEntry::from_cef_own(entry).unwrap(),if current == 0 { false } else { true },index,total,);
  if ret { 1 } else { 0 }
}
/// Callback interface for CefBrowserHost::PrintToPDF. The methods of this class
/// will be called on the browser process UI thread.
#[allow(non_snake_case)]
#[allow(unused_variables)]
pub trait PdfPrintCallback {
  /// Method that will be executed when the PDF printing has completed. |path|
  /// is the output path. |ok| will be true if the printing completed
  /// successfully or false otherwise.
  fn on_pdf_print_finished(&mut self, path: &crate::include::internal::CefString, ok: bool) -> () { Default::default() }
}
define_refcounted!(PdfPrintCallback, CefPdfPrintCallback, cef_pdf_print_callback_t, on_pdf_print_finished: cef_pdf_print_callback_t_on_pdf_print_finished,);
#[allow(non_snake_case)]
unsafe extern "C" fn cef_pdf_print_callback_t_on_pdf_print_finished(_self: *mut cef_sys::cef_pdf_print_callback_t, path: *const cef_sys::cef_string_t, ok: i32) -> () {
  let ret = CefPdfPrintCallback::from_cef(_self, true).get().on_pdf_print_finished(&crate::include::internal::CefString::from_cef(path).unwrap(),if ok == 0 { false } else { true },);
  ret
}
/// Callback interface for CefBrowserHost::DownloadImage. The methods of this
/// class will be called on the browser process UI thread.
#[allow(non_snake_case)]
#[allow(unused_variables)]
pub trait DownloadImageCallback {
  /// Method that will be executed when the image download has completed.
  /// |image_url| is the URL that was downloaded and |http_status_code| is the
  /// resulting HTTP status code. |image| is the resulting image, possibly at
  /// multiple scale factors, or empty if the download failed.
  fn on_download_image_finished(&mut self, image_url: &crate::include::internal::CefString, http_status_code: i32, image: Option<crate::include::CefImage>) -> () { Default::default() }
}
define_refcounted!(DownloadImageCallback, CefDownloadImageCallback, cef_download_image_callback_t, on_download_image_finished: cef_download_image_callback_t_on_download_image_finished,);
#[allow(non_snake_case)]
unsafe extern "C" fn cef_download_image_callback_t_on_download_image_finished(_self: *mut cef_sys::cef_download_image_callback_t, image_url: *const cef_sys::cef_string_t, http_status_code: i32, image: *mut cef_sys::cef_image_t) -> () {
  let ret = CefDownloadImageCallback::from_cef(_self, true).get().on_download_image_finished(&crate::include::internal::CefString::from_cef(image_url).unwrap(),http_status_code,crate::include::CefImage::from_cef_own(image),);
  ret
}
pub type CefBrowserHost = crate::include::base::CefProxy<cef_sys::cef_browser_host_t>;
#[allow(non_snake_case)]
impl CefBrowserHost {
  /// Create a new browser window using the window parameters specified by
  /// |windowInfo|. All values will be copied internally and the actual window
  /// will be created on the UI thread. If |request_context| is empty the
  /// global request context will be used. This method can be called on any
  /// browser process thread and will not block. The optional |extra_info|
  /// parameter provides an opportunity to specify extra information specific
  /// to the created browser that will be passed to
  /// CefRenderProcessHandler::OnBrowserCreated() in the render process.
  #[allow(non_snake_case)]
  pub fn create_browser(windowInfo: &crate::include::internal::CefWindowInfo, client: Option<crate::include::CefClient>, url: Option<&crate::include::internal::CefString>, settings: &crate::include::internal::CefBrowserSettings, extra_info: Option<crate::include::CefDictionaryValue>, request_context: Option<crate::include::CefRequestContext>, ) -> bool {
    unsafe {
      let ret = cef_sys::cef_browser_host_create_browser(windowInfo as *const _ as *const _,client.map_or(std::ptr::null_mut(), |o| crate::include::CefClient::to_cef_own(o)),crate::include::internal::IntoCef::into_cef(url),settings as *const _ as *const _,extra_info.map_or(std::ptr::null_mut(), |o| crate::include::CefDictionaryValue::to_cef_own(o)),request_context.map_or(std::ptr::null_mut(), |o| crate::include::CefRequestContext::to_cef_own(o)),);
      if ret == 0 { false } else { true }
    }
  }
  /// Create a new browser window using the window parameters specified by
  /// |windowInfo|. If |request_context| is empty the global request context
  /// will be used. This method can only be called on the browser process UI
  /// thread. The optional |extra_info| parameter provides an opportunity to
  /// specify extra information specific to the created browser that will be
  /// passed to CefRenderProcessHandler::OnBrowserCreated() in the render
  /// process.
  #[allow(non_snake_case)]
  pub fn create_browser_sync(windowInfo: &crate::include::internal::CefWindowInfo, client: Option<crate::include::CefClient>, url: Option<&crate::include::internal::CefString>, settings: &crate::include::internal::CefBrowserSettings, extra_info: Option<crate::include::CefDictionaryValue>, request_context: Option<crate::include::CefRequestContext>, ) -> Option<crate::include::CefBrowser> {
    unsafe {
      let ret = cef_sys::cef_browser_host_create_browser_sync(windowInfo as *const _ as *const _,client.map_or(std::ptr::null_mut(), |o| crate::include::CefClient::to_cef_own(o)),crate::include::internal::IntoCef::into_cef(url),settings as *const _ as *const _,extra_info.map_or(std::ptr::null_mut(), |o| crate::include::CefDictionaryValue::to_cef_own(o)),request_context.map_or(std::ptr::null_mut(), |o| crate::include::CefRequestContext::to_cef_own(o)),);
      crate::include::CefBrowser::from_cef_own(ret)
    }
  }
  /// Returns the hosted browser object.
  pub fn get_browser(&mut self) -> Option<crate::include::CefBrowser> {
    unsafe {
      let ret = match self.raw.as_ref().get_browser {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefBrowser::from_cef_own(ret)
    }
  }
  /// Request that the browser close. The JavaScript 'onbeforeunload' event will
  /// be fired. If |force_close| is false the event handler, if any, will be
  /// allowed to prompt the user and the user can optionally cancel the close.
  /// If |force_close| is true the prompt will not be displayed and the close
  /// will proceed. Results in a call to CefLifeSpanHandler::DoClose() if the
  /// event handler allows the close or if |force_close| is true. See
  /// CefLifeSpanHandler::DoClose() documentation for additional usage
  /// information.
  pub fn close_browser(&mut self, force_close: bool) -> () {
    unsafe {
      let ret = match self.raw.as_ref().close_browser {
        Some(f) => f(self.raw.as_ptr(),if force_close { 1 } else { 0 },),
        None => panic!(),
      };
      ret
    }
  }
  /// Helper for closing a browser. Call this method from the top-level window
  /// close handler. Internally this calls CloseBrowser(false) if the close has
  /// not yet been initiated. This method returns false while the close is
  /// pending and true after the close has completed. See CloseBrowser() and
  /// CefLifeSpanHandler::DoClose() documentation for additional usage
  /// information. This method must be called on the browser process UI thread.
  pub fn try_close_browser(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().try_close_browser {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Set whether the browser is focused.
  pub fn set_focus(&mut self, focus: bool) -> () {
    unsafe {
      let ret = match self.raw.as_ref().set_focus {
        Some(f) => f(self.raw.as_ptr(),if focus { 1 } else { 0 },),
        None => panic!(),
      };
      ret
    }
  }
  /// Retrieve the window handle for this browser. If this browser is wrapped in
  /// a CefBrowserView this method should be called on the browser process UI
  /// thread and it will return the handle for the top-level native window.
  pub fn get_window_handle(&mut self) -> crate::include::internal::CefWindowHandle {
    unsafe {
      let ret = match self.raw.as_ref().get_window_handle {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret.into()
    }
  }
  /// Retrieve the window handle of the browser that opened this browser. Will
  /// return NULL for non-popup windows or if this browser is wrapped in a
  /// CefBrowserView. This method can be used in combination with custom handling
  /// of modal windows.
  pub fn get_opener_window_handle(&mut self) -> crate::include::internal::CefWindowHandle {
    unsafe {
      let ret = match self.raw.as_ref().get_opener_window_handle {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret.into()
    }
  }
  /// Returns true if this browser is wrapped in a CefBrowserView.
  pub fn has_view(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().has_view {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns the client for this browser.
  pub fn get_client(&mut self) -> Option<crate::include::CefClient> {
    unsafe {
      let ret = match self.raw.as_ref().get_client {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefClient::from_cef_own(ret)
    }
  }
  /// Returns the request context for this browser.
  pub fn get_request_context(&mut self) -> Option<crate::include::CefRequestContext> {
    unsafe {
      let ret = match self.raw.as_ref().get_request_context {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefRequestContext::from_cef_own(ret)
    }
  }
  /// Get the current zoom level. The default zoom level is 0.0. This method can
  /// only be called on the UI thread.
  pub fn get_zoom_level(&mut self) -> f64 {
    unsafe {
      let ret = match self.raw.as_ref().get_zoom_level {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  /// Change the zoom level to the specified value. Specify 0.0 to reset the
  /// zoom level. If called on the UI thread the change will be applied
  /// immediately. Otherwise, the change will be applied asynchronously on the
  /// UI thread.
  pub fn set_zoom_level(&mut self, zoomLevel: f64) -> () {
    unsafe {
      let ret = match self.raw.as_ref().set_zoom_level {
        Some(f) => f(self.raw.as_ptr(),zoomLevel,),
        None => panic!(),
      };
      ret
    }
  }
  /// Call to run a file chooser dialog. Only a single file chooser dialog may be
  /// pending at any given time. |mode| represents the type of dialog to display.
  /// |title| to the title to be used for the dialog and may be empty to show the
  /// default title ("Open" or "Save" depending on the mode). |default_file_path|
  /// is the path with optional directory and/or file name component that will be
  /// initially selected in the dialog. |accept_filters| are used to restrict the
  /// selectable file types and may any combination of (a) valid lower-cased MIME
  /// types (e.g. "text/*" or "image/*"), (b) individual file extensions (e.g.
  /// ".txt" or ".png"), or (c) combined description and file extension delimited
  /// using "|" and ";" (e.g. "Image Types|.png;.gif;.jpg").
  /// |selected_accept_filter| is the 0-based index of the filter that will be
  /// selected by default. |callback| will be executed after the dialog is
  /// dismissed or immediately if another dialog is already pending. The dialog
  /// will be initiated asynchronously on the UI thread.
  pub fn run_file_dialog(&mut self, mode: crate::include::internal::CefFileDialogMode, title: Option<&crate::include::internal::CefString>, default_file_path: Option<&crate::include::internal::CefString>, accept_filters: &crate::include::internal::CefStringList, selected_accept_filter: i32, callback: crate::include::CefRunFileDialogCallback) -> () {
    unsafe {
      let ret = match self.raw.as_ref().run_file_dialog {
        Some(f) => f(self.raw.as_ptr(),mode.into(),crate::include::internal::IntoCef::into_cef(title),crate::include::internal::IntoCef::into_cef(default_file_path),accept_filters.into(),selected_accept_filter,crate::include::CefRunFileDialogCallback::to_cef_own(callback),),
        None => panic!(),
      };
      ret
    }
  }
  /// Download the file at |url| using CefDownloadHandler.
  pub fn start_download(&mut self, url: &crate::include::internal::CefString) -> () {
    unsafe {
      let ret = match self.raw.as_ref().start_download {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(url),),
        None => panic!(),
      };
      ret
    }
  }
  /// Download |image_url| and execute |callback| on completion with the images
  /// received from the renderer. If |is_favicon| is true then cookies are not
  /// sent and not accepted during download. Images with density independent
  /// pixel (DIP) sizes larger than |max_image_size| are filtered out from the
  /// image results. Versions of the image at different scale factors may be
  /// downloaded up to the maximum scale factor supported by the system. If there
  /// are no image results <= |max_image_size| then the smallest image is resized
  /// to |max_image_size| and is the only result. A |max_image_size| of 0 means
  /// unlimited. If |bypass_cache| is true then |image_url| is requested from the
  /// server even if it is present in the browser cache.
  pub fn download_image(&mut self, image_url: &crate::include::internal::CefString, is_favicon: bool, max_image_size: u32, bypass_cache: bool, callback: crate::include::CefDownloadImageCallback) -> () {
    unsafe {
      let ret = match self.raw.as_ref().download_image {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(image_url),if is_favicon { 1 } else { 0 },max_image_size,if bypass_cache { 1 } else { 0 },crate::include::CefDownloadImageCallback::to_cef_own(callback),),
        None => panic!(),
      };
      ret
    }
  }
  /// Print the current browser contents.
  pub fn print(&mut self) -> () {
    unsafe {
      let ret = match self.raw.as_ref().print {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  /// Print the current browser contents to the PDF file specified by |path| and
  /// execute |callback| on completion. The caller is responsible for deleting
  /// |path| when done. For PDF printing to work on Linux you must implement the
  /// CefPrintHandler::GetPdfPaperSize method.
  pub fn print_to_pdf(&mut self, path: &crate::include::internal::CefString, settings: &crate::include::internal::CefPdfPrintSettings, callback: Option<crate::include::CefPdfPrintCallback>) -> () {
    unsafe {
      let ret = match self.raw.as_ref().print_to_pdf {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(path),settings as *const _ as *const _,callback.map_or(std::ptr::null_mut(), |o| crate::include::CefPdfPrintCallback::to_cef_own(o)),),
        None => panic!(),
      };
      ret
    }
  }
  /// Search for |searchText|. |identifier| must be a unique ID and these IDs
  /// must strictly increase so that newer requests always have greater IDs than
  /// older requests. If |identifier| is zero or less than the previous ID value
  /// then it will be automatically assigned a new valid ID. |forward| indicates
  /// whether to search forward or backward within the page. |matchCase|
  /// indicates whether the search should be case-sensitive. |findNext| indicates
  /// whether this is the first request or a follow-up. The CefFindHandler
  /// instance, if any, returned via CefClient::GetFindHandler will be called to
  /// report find results.
  pub fn find(&mut self, identifier: i32, searchText: &crate::include::internal::CefString, forward: bool, matchCase: bool, findNext: bool) -> () {
    unsafe {
      let ret = match self.raw.as_ref().find {
        Some(f) => f(self.raw.as_ptr(),identifier,crate::include::internal::IntoCef::into_cef(searchText),if forward { 1 } else { 0 },if matchCase { 1 } else { 0 },if findNext { 1 } else { 0 },),
        None => panic!(),
      };
      ret
    }
  }
  /// Cancel all searches that are currently going on.
  pub fn stop_finding(&mut self, clearSelection: bool) -> () {
    unsafe {
      let ret = match self.raw.as_ref().stop_finding {
        Some(f) => f(self.raw.as_ptr(),if clearSelection { 1 } else { 0 },),
        None => panic!(),
      };
      ret
    }
  }
  /// Open developer tools (DevTools) in its own browser. The DevTools browser
  /// will remain associated with this browser. If the DevTools browser is
  /// already open then it will be focused, in which case the |windowInfo|,
  /// |client| and |settings| parameters will be ignored. If |inspect_element_at|
  /// is non-empty then the element at the specified (x,y) location will be
  /// inspected. The |windowInfo| parameter will be ignored if this browser is
  /// wrapped in a CefBrowserView.
  pub fn show_dev_tools(&mut self, windowInfo: Option<&crate::include::internal::CefWindowInfo>, client: Option<crate::include::CefClient>, settings: Option<&crate::include::internal::CefBrowserSettings>, inspect_element_at: Option<&crate::include::internal::CefPoint>) -> () {
    unsafe {
      let ret = match self.raw.as_ref().show_dev_tools {
        Some(f) => f(self.raw.as_ptr(),match windowInfo { Some(windowInfo) => windowInfo as *const _ as *const _, None => std::ptr::null_mut() },client.map_or(std::ptr::null_mut(), |o| crate::include::CefClient::to_cef_own(o)),match settings { Some(settings) => settings as *const _ as *const _, None => std::ptr::null_mut() },match inspect_element_at { Some(inspect_element_at) => inspect_element_at as *const _ as *const _, None => std::ptr::null_mut() },),
        None => panic!(),
      };
      ret
    }
  }
  /// Explicitly close the associated DevTools browser, if any.
  pub fn close_dev_tools(&mut self) -> () {
    unsafe {
      let ret = match self.raw.as_ref().close_dev_tools {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  /// Returns true if this browser currently has an associated DevTools browser.
  /// Must be called on the browser process UI thread.
  pub fn has_dev_tools(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().has_dev_tools {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Execute a method call over the DevTools protocol. This is a more structured
  /// version of SendDevToolsMessage. |message_id| is an incremental number that
  /// uniquely identifies the message (pass 0 to have the next number assigned
  /// automatically based on previous values). |method| is the method name.
  /// |params| are the method parameters, which may be empty. See the DevTools
  /// protocol documentation (linked above) for details of supported methods and
  /// the expected |params| dictionary contents. This method will return the
  /// assigned message ID if called on the UI thread and the message was
  /// successfully submitted for validation, otherwise 0. See the
  /// SendDevToolsMessage documentation for additional usage information.
  pub fn execute_dev_tools_method(&mut self, message_id: i32, method: &crate::include::internal::CefString, params: Option<crate::include::CefDictionaryValue>) -> i32 {
    unsafe {
      let ret = match self.raw.as_ref().execute_dev_tools_method {
        Some(f) => f(self.raw.as_ptr(),message_id,crate::include::internal::IntoCef::into_cef(method),params.map_or(std::ptr::null_mut(), |o| crate::include::CefDictionaryValue::to_cef_own(o)),),
        None => panic!(),
      };
      ret
    }
  }
  /// Add an observer for DevTools protocol messages (method results and events).
  /// The observer will remain registered until the returned Registration object
  /// is destroyed. See the SendDevToolsMessage documentation for additional
  /// usage information.
  pub fn add_dev_tools_message_observer(&mut self, observer: crate::include::CefDevToolsMessageObserver) -> Option<crate::include::CefRegistration> {
    unsafe {
      let ret = match self.raw.as_ref().add_dev_tools_message_observer {
        Some(f) => f(self.raw.as_ptr(),crate::include::CefDevToolsMessageObserver::to_cef_own(observer),),
        None => panic!(),
      };
      crate::include::CefRegistration::from_cef_own(ret)
    }
  }
  /// Retrieve a snapshot of current navigation entries as values sent to the
  /// specified visitor. If |current_only| is true only the current navigation
  /// entry will be sent, otherwise all navigation entries will be sent.
  pub fn get_navigation_entries(&mut self, visitor: crate::include::CefNavigationEntryVisitor, current_only: bool) -> () {
    unsafe {
      let ret = match self.raw.as_ref().get_navigation_entries {
        Some(f) => f(self.raw.as_ptr(),crate::include::CefNavigationEntryVisitor::to_cef_own(visitor),if current_only { 1 } else { 0 },),
        None => panic!(),
      };
      ret
    }
  }
  /// Set whether mouse cursor change is disabled.
  pub fn set_mouse_cursor_change_disabled(&mut self, disabled: bool) -> () {
    unsafe {
      let ret = match self.raw.as_ref().set_mouse_cursor_change_disabled {
        Some(f) => f(self.raw.as_ptr(),if disabled { 1 } else { 0 },),
        None => panic!(),
      };
      ret
    }
  }
  /// Returns true if mouse cursor change is disabled.
  pub fn is_mouse_cursor_change_disabled(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_mouse_cursor_change_disabled {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// If a misspelled word is currently selected in an editable node calling
  /// this method will replace it with the specified |word|.
  pub fn replace_misspelling(&mut self, word: &crate::include::internal::CefString) -> () {
    unsafe {
      let ret = match self.raw.as_ref().replace_misspelling {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(word),),
        None => panic!(),
      };
      ret
    }
  }
  /// Add the specified |word| to the spelling dictionary.
  pub fn add_word_to_dictionary(&mut self, word: &crate::include::internal::CefString) -> () {
    unsafe {
      let ret = match self.raw.as_ref().add_word_to_dictionary {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(word),),
        None => panic!(),
      };
      ret
    }
  }
  /// Returns true if window rendering is disabled.
  pub fn is_window_rendering_disabled(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_window_rendering_disabled {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Notify the browser that the widget has been resized. The browser will first
  /// call CefRenderHandler::GetViewRect to get the new size and then call
  /// CefRenderHandler::OnPaint asynchronously with the updated regions. This
  /// method is only used when window rendering is disabled.
  pub fn was_resized(&mut self) -> () {
    unsafe {
      let ret = match self.raw.as_ref().was_resized {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  /// Notify the browser that it has been hidden or shown. Layouting and
  /// CefRenderHandler::OnPaint notification will stop when the browser is
  /// hidden. This method is only used when window rendering is disabled.
  pub fn was_hidden(&mut self, hidden: bool) -> () {
    unsafe {
      let ret = match self.raw.as_ref().was_hidden {
        Some(f) => f(self.raw.as_ptr(),if hidden { 1 } else { 0 },),
        None => panic!(),
      };
      ret
    }
  }
  /// Send a notification to the browser that the screen info has changed. The
  /// browser will then call CefRenderHandler::GetScreenInfo to update the
  /// screen information with the new values. This simulates moving the webview
  /// window from one display to another, or changing the properties of the
  /// current display. This method is only used when window rendering is
  /// disabled.
  pub fn notify_screen_info_changed(&mut self) -> () {
    unsafe {
      let ret = match self.raw.as_ref().notify_screen_info_changed {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  /// Invalidate the view. The browser will call CefRenderHandler::OnPaint
  /// asynchronously. This method is only used when window rendering is
  /// disabled.
  pub fn invalidate(&mut self, type_: crate::include::internal::CefPaintElementType) -> () {
    unsafe {
      let ret = match self.raw.as_ref().invalidate {
        Some(f) => f(self.raw.as_ptr(),type_.into(),),
        None => panic!(),
      };
      ret
    }
  }
  /// Issue a BeginFrame request to Chromium.  Only valid when
  /// CefWindowInfo::external_begin_frame_enabled is set to true.
  pub fn send_external_begin_frame(&mut self) -> () {
    unsafe {
      let ret = match self.raw.as_ref().send_external_begin_frame {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  /// Send a key event to the browser.
  pub fn send_key_event(&mut self, event: &crate::include::internal::CefKeyEvent) -> () {
    unsafe {
      let ret = match self.raw.as_ref().send_key_event {
        Some(f) => f(self.raw.as_ptr(),event as *const _ as *const _,),
        None => panic!(),
      };
      ret
    }
  }
  /// Send a mouse click event to the browser. The |x| and |y| coordinates are
  /// relative to the upper-left corner of the view.
  pub fn send_mouse_click_event(&mut self, event: &crate::include::internal::CefMouseEvent, type_: crate::include::internal::CefMouseButtonType, mouseUp: bool, clickCount: i32) -> () {
    unsafe {
      let ret = match self.raw.as_ref().send_mouse_click_event {
        Some(f) => f(self.raw.as_ptr(),event as *const _ as *const _,type_.into(),if mouseUp { 1 } else { 0 },clickCount,),
        None => panic!(),
      };
      ret
    }
  }
  /// Send a mouse move event to the browser. The |x| and |y| coordinates are
  /// relative to the upper-left corner of the view.
  pub fn send_mouse_move_event(&mut self, event: &crate::include::internal::CefMouseEvent, mouseLeave: bool) -> () {
    unsafe {
      let ret = match self.raw.as_ref().send_mouse_move_event {
        Some(f) => f(self.raw.as_ptr(),event as *const _ as *const _,if mouseLeave { 1 } else { 0 },),
        None => panic!(),
      };
      ret
    }
  }
  /// Send a mouse wheel event to the browser. The |x| and |y| coordinates are
  /// relative to the upper-left corner of the view. The |deltaX| and |deltaY|
  /// values represent the movement delta in the X and Y directions respectively.
  /// In order to scroll inside select popups with window rendering disabled
  /// CefRenderHandler::GetScreenPoint should be implemented properly.
  pub fn send_mouse_wheel_event(&mut self, event: &crate::include::internal::CefMouseEvent, deltaX: i32, deltaY: i32) -> () {
    unsafe {
      let ret = match self.raw.as_ref().send_mouse_wheel_event {
        Some(f) => f(self.raw.as_ptr(),event as *const _ as *const _,deltaX,deltaY,),
        None => panic!(),
      };
      ret
    }
  }
  /// Send a touch event to the browser for a windowless browser.
  pub fn send_touch_event(&mut self, event: &crate::include::internal::CefTouchEvent) -> () {
    unsafe {
      let ret = match self.raw.as_ref().send_touch_event {
        Some(f) => f(self.raw.as_ptr(),event as *const _ as *const _,),
        None => panic!(),
      };
      ret
    }
  }
  /// Send a focus event to the browser.
  pub fn send_focus_event(&mut self, setFocus: bool) -> () {
    unsafe {
      let ret = match self.raw.as_ref().send_focus_event {
        Some(f) => f(self.raw.as_ptr(),if setFocus { 1 } else { 0 },),
        None => panic!(),
      };
      ret
    }
  }
  /// Send a capture lost event to the browser.
  pub fn send_capture_lost_event(&mut self) -> () {
    unsafe {
      let ret = match self.raw.as_ref().send_capture_lost_event {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  /// Notify the browser that the window hosting it is about to be moved or
  /// resized. This method is only used on Windows and Linux.
  pub fn notify_move_or_resize_started(&mut self) -> () {
    unsafe {
      let ret = match self.raw.as_ref().notify_move_or_resize_started {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  /// Returns the maximum rate in frames per second (fps) that CefRenderHandler::
  /// OnPaint will be called for a windowless browser. The actual fps may be
  /// lower if the browser cannot generate frames at the requested rate. The
  /// minimum value is 1 and the maximum value is 60 (default 30). This method
  /// can only be called on the UI thread.
  pub fn get_windowless_frame_rate(&mut self) -> i32 {
    unsafe {
      let ret = match self.raw.as_ref().get_windowless_frame_rate {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  /// Set the maximum rate in frames per second (fps) that CefRenderHandler::
  /// OnPaint will be called for a windowless browser. The actual fps may be
  /// lower if the browser cannot generate frames at the requested rate. The
  /// minimum value is 1 and the maximum value is 60 (default 30). Can also be
  /// set at browser creation via CefBrowserSettings.windowless_frame_rate.
  pub fn set_windowless_frame_rate(&mut self, frame_rate: i32) -> () {
    unsafe {
      let ret = match self.raw.as_ref().set_windowless_frame_rate {
        Some(f) => f(self.raw.as_ptr(),frame_rate,),
        None => panic!(),
      };
      ret
    }
  }
  /// Completes the existing composition by optionally inserting the specified
  /// |text| into the composition node. |replacement_range| is an optional range
  /// of the existing text that will be replaced. |relative_cursor_pos| is where
  /// the cursor will be positioned relative to the current cursor position. See
  /// comments on ImeSetComposition for usage. The |replacement_range| and
  /// |relative_cursor_pos| values are only used on OS X.
  /// This method is only used when window rendering is disabled.
  pub fn ime_commit_text(&mut self, text: Option<&crate::include::internal::CefString>, replacement_range: &crate::include::internal::CefRange, relative_cursor_pos: i32) -> () {
    unsafe {
      let ret = match self.raw.as_ref().ime_commit_text {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(text),replacement_range as *const _ as *const _,relative_cursor_pos,),
        None => panic!(),
      };
      ret
    }
  }
  /// Completes the existing composition by applying the current composition node
  /// contents. If |keep_selection| is false the current selection, if any, will
  /// be discarded. See comments on ImeSetComposition for usage.
  /// This method is only used when window rendering is disabled.
  pub fn ime_finish_composing_text(&mut self, keep_selection: bool) -> () {
    unsafe {
      let ret = match self.raw.as_ref().ime_finish_composing_text {
        Some(f) => f(self.raw.as_ptr(),if keep_selection { 1 } else { 0 },),
        None => panic!(),
      };
      ret
    }
  }
  /// Cancels the existing composition and discards the composition node
  /// contents without applying them. See comments on ImeSetComposition for
  /// usage.
  /// This method is only used when window rendering is disabled.
  pub fn ime_cancel_composition(&mut self) -> () {
    unsafe {
      let ret = match self.raw.as_ref().ime_cancel_composition {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  /// Call this method when the user drags the mouse into the web view (before
  /// calling DragTargetDragOver/DragTargetLeave/DragTargetDrop).
  /// |drag_data| should not contain file contents as this type of data is not
  /// allowed to be dragged into the web view. File contents can be removed using
  /// CefDragData::ResetFileContents (for example, if |drag_data| comes from
  /// CefRenderHandler::StartDragging).
  /// This method is only used when window rendering is disabled.
  pub fn drag_target_drag_enter(&mut self, drag_data: crate::include::CefDragData, event: &crate::include::internal::CefMouseEvent, allowed_ops: crate::include::internal::CefDragOperationsMask) -> () {
    unsafe {
      let ret = match self.raw.as_ref().drag_target_drag_enter {
        Some(f) => f(self.raw.as_ptr(),crate::include::CefDragData::to_cef_own(drag_data),event as *const _ as *const _,allowed_ops.into(),),
        None => panic!(),
      };
      ret
    }
  }
  /// Call this method each time the mouse is moved across the web view during
  /// a drag operation (after calling DragTargetDragEnter and before calling
  /// DragTargetDragLeave/DragTargetDrop).
  /// This method is only used when window rendering is disabled.
  pub fn drag_target_drag_over(&mut self, event: &crate::include::internal::CefMouseEvent, allowed_ops: crate::include::internal::CefDragOperationsMask) -> () {
    unsafe {
      let ret = match self.raw.as_ref().drag_target_drag_over {
        Some(f) => f(self.raw.as_ptr(),event as *const _ as *const _,allowed_ops.into(),),
        None => panic!(),
      };
      ret
    }
  }
  /// Call this method when the user drags the mouse out of the web view (after
  /// calling DragTargetDragEnter).
  /// This method is only used when window rendering is disabled.
  pub fn drag_target_drag_leave(&mut self) -> () {
    unsafe {
      let ret = match self.raw.as_ref().drag_target_drag_leave {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  /// Call this method when the user completes the drag operation by dropping
  /// the object onto the web view (after calling DragTargetDragEnter).
  /// The object being dropped is |drag_data|, given as an argument to
  /// the previous DragTargetDragEnter call.
  /// This method is only used when window rendering is disabled.
  pub fn drag_target_drop(&mut self, event: &crate::include::internal::CefMouseEvent) -> () {
    unsafe {
      let ret = match self.raw.as_ref().drag_target_drop {
        Some(f) => f(self.raw.as_ptr(),event as *const _ as *const _,),
        None => panic!(),
      };
      ret
    }
  }
  /// Call this method when the drag operation started by a
  /// CefRenderHandler::StartDragging call has ended either in a drop or
  /// by being cancelled. |x| and |y| are mouse coordinates relative to the
  /// upper-left corner of the view. If the web view is both the drag source
  /// and the drag target then all DragTarget* methods should be called before
  /// DragSource* mthods.
  /// This method is only used when window rendering is disabled.
  pub fn drag_source_ended_at(&mut self, x: i32, y: i32, op: crate::include::internal::CefDragOperationsMask) -> () {
    unsafe {
      let ret = match self.raw.as_ref().drag_source_ended_at {
        Some(f) => f(self.raw.as_ptr(),x,y,op.into(),),
        None => panic!(),
      };
      ret
    }
  }
  /// Call this method when the drag operation started by a
  /// CefRenderHandler::StartDragging call has completed. This method may be
  /// called immediately without first calling DragSourceEndedAt to cancel a
  /// drag operation. If the web view is both the drag source and the drag
  /// target then all DragTarget* methods should be called before DragSource*
  /// mthods.
  /// This method is only used when window rendering is disabled.
  pub fn drag_source_system_drag_ended(&mut self) -> () {
    unsafe {
      let ret = match self.raw.as_ref().drag_source_system_drag_ended {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  /// Returns the current visible navigation entry for this browser. This method
  /// can only be called on the UI thread.
  pub fn get_visible_navigation_entry(&mut self) -> Option<crate::include::CefNavigationEntry> {
    unsafe {
      let ret = match self.raw.as_ref().get_visible_navigation_entry {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefNavigationEntry::from_cef_own(ret)
    }
  }
  /// Set accessibility state for all frames. |accessibility_state| may be
  /// default, enabled or disabled. If |accessibility_state| is STATE_DEFAULT
  /// then accessibility will be disabled by default and the state may be further
  /// controlled with the "force-renderer-accessibility" and
  /// "disable-renderer-accessibility" command-line switches. If
  /// |accessibility_state| is STATE_ENABLED then accessibility will be enabled.
  /// If |accessibility_state| is STATE_DISABLED then accessibility will be
  /// completely disabled.
  /// 
  /// For windowed browsers accessibility will be enabled in Complete mode (which
  /// corresponds to kAccessibilityModeComplete in Chromium). In this mode all
  /// platform accessibility objects will be created and managed by Chromium's
  /// internal implementation. The client needs only to detect the screen reader
  /// and call this method appropriately. For example, on macOS the client can
  /// handle the @"AXEnhancedUserInterface" accessibility attribute to detect
  /// VoiceOver state changes and on Windows the client can handle WM_GETOBJECT
  /// with OBJID_CLIENT to detect accessibility readers.
  /// 
  /// For windowless browsers accessibility will be enabled in TreeOnly mode
  /// (which corresponds to kAccessibilityModeWebContentsOnly in Chromium). In
  /// this mode renderer accessibility is enabled, the full tree is computed, and
  /// events are passed to CefAccessibiltyHandler, but platform accessibility
  /// objects are not created. The client may implement platform accessibility
  /// objects using CefAccessibiltyHandler callbacks if desired.
  pub fn set_accessibility_state(&mut self, accessibility_state: crate::include::internal::CefState) -> () {
    unsafe {
      let ret = match self.raw.as_ref().set_accessibility_state {
        Some(f) => f(self.raw.as_ptr(),accessibility_state.into(),),
        None => panic!(),
      };
      ret
    }
  }
  /// Enable notifications of auto resize via CefDisplayHandler::OnAutoResize.
  /// Notifications are disabled by default. |min_size| and |max_size| define the
  /// range of allowed sizes.
  pub fn set_auto_resize_enabled(&mut self, enabled: bool, min_size: &crate::include::internal::CefSize, max_size: &crate::include::internal::CefSize) -> () {
    unsafe {
      let ret = match self.raw.as_ref().set_auto_resize_enabled {
        Some(f) => f(self.raw.as_ptr(),if enabled { 1 } else { 0 },min_size as *const _ as *const _,max_size as *const _ as *const _,),
        None => panic!(),
      };
      ret
    }
  }
  /// Returns the extension hosted in this browser or NULL if no extension is
  /// hosted. See CefRequestContext::LoadExtension for details.
  pub fn get_extension(&mut self) -> Option<crate::include::CefExtension> {
    unsafe {
      let ret = match self.raw.as_ref().get_extension {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefExtension::from_cef_own(ret)
    }
  }
  /// Returns true if this browser is hosting an extension background script.
  /// Background hosts do not have a window and are not displayable. See
  /// CefRequestContext::LoadExtension for details.
  pub fn is_background_host(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_background_host {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Set whether the browser's audio is muted.
  pub fn set_audio_muted(&mut self, mute: bool) -> () {
    unsafe {
      let ret = match self.raw.as_ref().set_audio_muted {
        Some(f) => f(self.raw.as_ptr(),if mute { 1 } else { 0 },),
        None => panic!(),
      };
      ret
    }
  }
  /// Returns true if the browser's audio is muted.  This method can only be
  /// called on the UI thread.
  pub fn is_audio_muted(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_audio_muted {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
}
