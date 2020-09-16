pub type CefBrowser = crate::include::base::CefProxy<cef_sys::cef_browser_t>;
#[allow(non_snake_case)]
impl CefBrowser {
  pub fn get_host(&mut self) -> Option<crate::include::CefBrowserHost> {
    unsafe {
      let ret = match self.raw.as_ref().get_host {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefBrowserHost::from_cef_own(ret)
    }
  }
  pub fn can_go_back(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().can_go_back {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn go_back(&mut self) -> () {
    unsafe {
      let ret = match self.raw.as_ref().go_back {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn can_go_forward(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().can_go_forward {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn go_forward(&mut self) -> () {
    unsafe {
      let ret = match self.raw.as_ref().go_forward {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn is_loading(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_loading {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn reload(&mut self) -> () {
    unsafe {
      let ret = match self.raw.as_ref().reload {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn reload_ignore_cache(&mut self) -> () {
    unsafe {
      let ret = match self.raw.as_ref().reload_ignore_cache {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn stop_load(&mut self) -> () {
    unsafe {
      let ret = match self.raw.as_ref().stop_load {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn get_identifier(&mut self) -> i32 {
    unsafe {
      let ret = match self.raw.as_ref().get_identifier {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn is_same(&mut self, that: crate::include::CefBrowser) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_same {
        Some(f) => f(self.raw.as_ptr(),crate::include::CefBrowser::to_cef_own(that),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn is_popup(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_popup {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn has_document(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().has_document {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn get_main_frame(&mut self) -> Option<crate::include::CefFrame> {
    unsafe {
      let ret = match self.raw.as_ref().get_main_frame {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefFrame::from_cef_own(ret)
    }
  }
  pub fn get_focused_frame(&mut self) -> Option<crate::include::CefFrame> {
    unsafe {
      let ret = match self.raw.as_ref().get_focused_frame {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefFrame::from_cef_own(ret)
    }
  }
  pub fn get_frame_byident(&mut self, identifier: i64) -> Option<crate::include::CefFrame> {
    unsafe {
      let ret = match self.raw.as_ref().get_frame_byident {
        Some(f) => f(self.raw.as_ptr(),identifier,),
        None => panic!(),
      };
      crate::include::CefFrame::from_cef_own(ret)
    }
  }
  pub fn get_frame(&mut self, name: Option<&crate::include::internal::CefString>) -> Option<crate::include::CefFrame> {
    unsafe {
      let ret = match self.raw.as_ref().get_frame {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(name),),
        None => panic!(),
      };
      crate::include::CefFrame::from_cef_own(ret)
    }
  }
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
pub trait RunFileDialogCallback {
  fn on_file_dialog_dismissed(&mut self, selected_accept_filter: i32, file_paths: &crate::include::internal::CefStringList) -> () { Default::default() }
}
define_refcounted!(RunFileDialogCallback, run_file_dialog_callback, on_file_dialog_dismissed,);
#[allow(non_snake_case)]
unsafe extern "C" fn cef_run_file_dialog_callback_t_on_file_dialog_dismissed(_self: *mut cef_sys::cef_run_file_dialog_callback_t, selected_accept_filter: i32, file_paths: cef_sys::cef_string_list_t) -> () {
  let ret = CefRunFileDialogCallback::from_cef(_self, true).get().on_file_dialog_dismissed(selected_accept_filter,&file_paths.into(),);
  ret
}
/// Callback interface for CefBrowserHost::GetNavigationEntries. The methods of
/// this class will be called on the browser process UI thread.
#[allow(non_snake_case)]
pub trait NavigationEntryVisitor {
  fn visit(&mut self, entry: crate::include::CefNavigationEntry, current: bool, index: i32, total: i32) -> bool { Default::default() }
}
define_refcounted!(NavigationEntryVisitor, navigation_entry_visitor, visit,);
#[allow(non_snake_case)]
unsafe extern "C" fn cef_navigation_entry_visitor_t_visit(_self: *mut cef_sys::cef_navigation_entry_visitor_t, entry: *mut cef_sys::cef_navigation_entry_t, current: i32, index: i32, total: i32) -> i32 {
  let ret = CefNavigationEntryVisitor::from_cef(_self, true).get().visit(crate::include::CefNavigationEntry::from_cef_own(entry).unwrap(),if current == 0 { false } else { true },index,total,);
  if ret { 1 } else { 0 }
}
/// Callback interface for CefBrowserHost::PrintToPDF. The methods of this class
/// will be called on the browser process UI thread.
#[allow(non_snake_case)]
pub trait PdfPrintCallback {
  fn on_pdf_print_finished(&mut self, path: &crate::include::internal::CefString, ok: bool) -> () { Default::default() }
}
define_refcounted!(PdfPrintCallback, pdf_print_callback, on_pdf_print_finished,);
#[allow(non_snake_case)]
unsafe extern "C" fn cef_pdf_print_callback_t_on_pdf_print_finished(_self: *mut cef_sys::cef_pdf_print_callback_t, path: *const cef_sys::cef_string_t, ok: i32) -> () {
  let ret = CefPdfPrintCallback::from_cef(_self, true).get().on_pdf_print_finished(&crate::include::internal::CefString::from_cef(path).unwrap(),if ok == 0 { false } else { true },);
  ret
}
/// Callback interface for CefBrowserHost::DownloadImage. The methods of this
/// class will be called on the browser process UI thread.
#[allow(non_snake_case)]
pub trait DownloadImageCallback {
  fn on_download_image_finished(&mut self, image_url: &crate::include::internal::CefString, http_status_code: i32, image: Option<crate::include::CefImage>) -> () { Default::default() }
}
define_refcounted!(DownloadImageCallback, download_image_callback, on_download_image_finished,);
#[allow(non_snake_case)]
unsafe extern "C" fn cef_download_image_callback_t_on_download_image_finished(_self: *mut cef_sys::cef_download_image_callback_t, image_url: *const cef_sys::cef_string_t, http_status_code: i32, image: *mut cef_sys::cef_image_t) -> () {
  let ret = CefDownloadImageCallback::from_cef(_self, true).get().on_download_image_finished(&crate::include::internal::CefString::from_cef(image_url).unwrap(),http_status_code,crate::include::CefImage::from_cef_own(image),);
  ret
}
pub type CefBrowserHost = crate::include::base::CefProxy<cef_sys::cef_browser_host_t>;
#[allow(non_snake_case)]
impl CefBrowserHost {
  pub fn get_browser(&mut self) -> Option<crate::include::CefBrowser> {
    unsafe {
      let ret = match self.raw.as_ref().get_browser {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefBrowser::from_cef_own(ret)
    }
  }
  pub fn close_browser(&mut self, force_close: bool) -> () {
    unsafe {
      let ret = match self.raw.as_ref().close_browser {
        Some(f) => f(self.raw.as_ptr(),if force_close { 1 } else { 0 },),
        None => panic!(),
      };
      ret
    }
  }
  pub fn try_close_browser(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().try_close_browser {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn set_focus(&mut self, focus: bool) -> () {
    unsafe {
      let ret = match self.raw.as_ref().set_focus {
        Some(f) => f(self.raw.as_ptr(),if focus { 1 } else { 0 },),
        None => panic!(),
      };
      ret
    }
  }
  pub fn get_window_handle(&mut self) -> crate::include::internal::CefWindowHandle {
    unsafe {
      let ret = match self.raw.as_ref().get_window_handle {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret.into()
    }
  }
  pub fn get_opener_window_handle(&mut self) -> crate::include::internal::CefWindowHandle {
    unsafe {
      let ret = match self.raw.as_ref().get_opener_window_handle {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret.into()
    }
  }
  pub fn has_view(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().has_view {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn get_client(&mut self) -> Option<crate::include::CefClient> {
    unsafe {
      let ret = match self.raw.as_ref().get_client {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefClient::from_cef_own(ret)
    }
  }
  pub fn get_request_context(&mut self) -> Option<crate::include::CefRequestContext> {
    unsafe {
      let ret = match self.raw.as_ref().get_request_context {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefRequestContext::from_cef_own(ret)
    }
  }
  pub fn get_zoom_level(&mut self) -> f64 {
    unsafe {
      let ret = match self.raw.as_ref().get_zoom_level {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn set_zoom_level(&mut self, zoomLevel: f64) -> () {
    unsafe {
      let ret = match self.raw.as_ref().set_zoom_level {
        Some(f) => f(self.raw.as_ptr(),zoomLevel,),
        None => panic!(),
      };
      ret
    }
  }
  pub fn run_file_dialog(&mut self, mode: crate::include::internal::CefFileDialogMode, title: &crate::include::internal::CefString, default_file_path: &crate::include::internal::CefString, accept_filters: &crate::include::internal::CefStringList, selected_accept_filter: i32, callback: crate::include::CefRunFileDialogCallback) -> () {
    unsafe {
      let ret = match self.raw.as_ref().run_file_dialog {
        Some(f) => f(self.raw.as_ptr(),mode.into(),crate::include::internal::IntoCef::into_cef(title),crate::include::internal::IntoCef::into_cef(default_file_path),accept_filters.into(),selected_accept_filter,crate::include::CefRunFileDialogCallback::to_cef_own(callback),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn start_download(&mut self, url: &crate::include::internal::CefString) -> () {
    unsafe {
      let ret = match self.raw.as_ref().start_download {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(url),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn download_image(&mut self, image_url: &crate::include::internal::CefString, is_favicon: bool, max_image_size: u32, bypass_cache: bool, callback: crate::include::CefDownloadImageCallback) -> () {
    unsafe {
      let ret = match self.raw.as_ref().download_image {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(image_url),if is_favicon { 1 } else { 0 },max_image_size,if bypass_cache { 1 } else { 0 },crate::include::CefDownloadImageCallback::to_cef_own(callback),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn print(&mut self) -> () {
    unsafe {
      let ret = match self.raw.as_ref().print {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn print_to_pdf(&mut self, path: &crate::include::internal::CefString, settings: &crate::include::internal::CefPdfPrintSettings, callback: Option<crate::include::CefPdfPrintCallback>) -> () {
    unsafe {
      let ret = match self.raw.as_ref().print_to_pdf {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(path),&settings.raw,callback.map_or(std::ptr::null_mut(), |o| crate::include::CefPdfPrintCallback::to_cef_own(o)),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn find(&mut self, identifier: i32, searchText: &crate::include::internal::CefString, forward: bool, matchCase: bool, findNext: bool) -> () {
    unsafe {
      let ret = match self.raw.as_ref().find {
        Some(f) => f(self.raw.as_ptr(),identifier,crate::include::internal::IntoCef::into_cef(searchText),if forward { 1 } else { 0 },if matchCase { 1 } else { 0 },if findNext { 1 } else { 0 },),
        None => panic!(),
      };
      ret
    }
  }
  pub fn stop_finding(&mut self, clearSelection: bool) -> () {
    unsafe {
      let ret = match self.raw.as_ref().stop_finding {
        Some(f) => f(self.raw.as_ptr(),if clearSelection { 1 } else { 0 },),
        None => panic!(),
      };
      ret
    }
  }
  pub fn show_dev_tools(&mut self, windowInfo: &crate::include::internal::CefWindowInfo, client: crate::include::CefClient, settings: &crate::include::internal::CefBrowserSettings, inspect_element_at: &crate::include::internal::CefPoint) -> () {
    unsafe {
      let ret = match self.raw.as_ref().show_dev_tools {
        Some(f) => f(self.raw.as_ptr(),&windowInfo.raw,crate::include::CefClient::to_cef_own(client),&settings.raw,&inspect_element_at.raw,),
        None => panic!(),
      };
      ret
    }
  }
  pub fn close_dev_tools(&mut self) -> () {
    unsafe {
      let ret = match self.raw.as_ref().close_dev_tools {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn has_dev_tools(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().has_dev_tools {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn execute_dev_tools_method(&mut self, message_id: i32, method: &crate::include::internal::CefString, params: Option<crate::include::CefDictionaryValue>) -> i32 {
    unsafe {
      let ret = match self.raw.as_ref().execute_dev_tools_method {
        Some(f) => f(self.raw.as_ptr(),message_id,crate::include::internal::IntoCef::into_cef(method),params.map_or(std::ptr::null_mut(), |o| crate::include::CefDictionaryValue::to_cef_own(o)),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn add_dev_tools_message_observer(&mut self, observer: crate::include::CefDevToolsMessageObserver) -> Option<crate::include::CefRegistration> {
    unsafe {
      let ret = match self.raw.as_ref().add_dev_tools_message_observer {
        Some(f) => f(self.raw.as_ptr(),crate::include::CefDevToolsMessageObserver::to_cef_own(observer),),
        None => panic!(),
      };
      crate::include::CefRegistration::from_cef_own(ret)
    }
  }
  pub fn get_navigation_entries(&mut self, visitor: crate::include::CefNavigationEntryVisitor, current_only: bool) -> () {
    unsafe {
      let ret = match self.raw.as_ref().get_navigation_entries {
        Some(f) => f(self.raw.as_ptr(),crate::include::CefNavigationEntryVisitor::to_cef_own(visitor),if current_only { 1 } else { 0 },),
        None => panic!(),
      };
      ret
    }
  }
  pub fn set_mouse_cursor_change_disabled(&mut self, disabled: bool) -> () {
    unsafe {
      let ret = match self.raw.as_ref().set_mouse_cursor_change_disabled {
        Some(f) => f(self.raw.as_ptr(),if disabled { 1 } else { 0 },),
        None => panic!(),
      };
      ret
    }
  }
  pub fn is_mouse_cursor_change_disabled(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_mouse_cursor_change_disabled {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn replace_misspelling(&mut self, word: &crate::include::internal::CefString) -> () {
    unsafe {
      let ret = match self.raw.as_ref().replace_misspelling {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(word),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn add_word_to_dictionary(&mut self, word: &crate::include::internal::CefString) -> () {
    unsafe {
      let ret = match self.raw.as_ref().add_word_to_dictionary {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(word),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn is_window_rendering_disabled(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_window_rendering_disabled {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn was_resized(&mut self) -> () {
    unsafe {
      let ret = match self.raw.as_ref().was_resized {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn was_hidden(&mut self, hidden: bool) -> () {
    unsafe {
      let ret = match self.raw.as_ref().was_hidden {
        Some(f) => f(self.raw.as_ptr(),if hidden { 1 } else { 0 },),
        None => panic!(),
      };
      ret
    }
  }
  pub fn notify_screen_info_changed(&mut self) -> () {
    unsafe {
      let ret = match self.raw.as_ref().notify_screen_info_changed {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn invalidate(&mut self, type_: crate::include::internal::CefPaintElementType) -> () {
    unsafe {
      let ret = match self.raw.as_ref().invalidate {
        Some(f) => f(self.raw.as_ptr(),type_.into(),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn send_external_begin_frame(&mut self) -> () {
    unsafe {
      let ret = match self.raw.as_ref().send_external_begin_frame {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn send_key_event(&mut self, event: &crate::include::internal::CefKeyEvent) -> () {
    unsafe {
      let ret = match self.raw.as_ref().send_key_event {
        Some(f) => f(self.raw.as_ptr(),&event.raw,),
        None => panic!(),
      };
      ret
    }
  }
  pub fn send_mouse_click_event(&mut self, event: &crate::include::internal::CefMouseEvent, type_: crate::include::internal::CefMouseButtonType, mouseUp: bool, clickCount: i32) -> () {
    unsafe {
      let ret = match self.raw.as_ref().send_mouse_click_event {
        Some(f) => f(self.raw.as_ptr(),&event.raw,type_.into(),if mouseUp { 1 } else { 0 },clickCount,),
        None => panic!(),
      };
      ret
    }
  }
  pub fn send_mouse_move_event(&mut self, event: &crate::include::internal::CefMouseEvent, mouseLeave: bool) -> () {
    unsafe {
      let ret = match self.raw.as_ref().send_mouse_move_event {
        Some(f) => f(self.raw.as_ptr(),&event.raw,if mouseLeave { 1 } else { 0 },),
        None => panic!(),
      };
      ret
    }
  }
  pub fn send_mouse_wheel_event(&mut self, event: &crate::include::internal::CefMouseEvent, deltaX: i32, deltaY: i32) -> () {
    unsafe {
      let ret = match self.raw.as_ref().send_mouse_wheel_event {
        Some(f) => f(self.raw.as_ptr(),&event.raw,deltaX,deltaY,),
        None => panic!(),
      };
      ret
    }
  }
  pub fn send_touch_event(&mut self, event: &crate::include::internal::CefTouchEvent) -> () {
    unsafe {
      let ret = match self.raw.as_ref().send_touch_event {
        Some(f) => f(self.raw.as_ptr(),&event.raw,),
        None => panic!(),
      };
      ret
    }
  }
  pub fn send_focus_event(&mut self, setFocus: bool) -> () {
    unsafe {
      let ret = match self.raw.as_ref().send_focus_event {
        Some(f) => f(self.raw.as_ptr(),if setFocus { 1 } else { 0 },),
        None => panic!(),
      };
      ret
    }
  }
  pub fn send_capture_lost_event(&mut self) -> () {
    unsafe {
      let ret = match self.raw.as_ref().send_capture_lost_event {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn notify_move_or_resize_started(&mut self) -> () {
    unsafe {
      let ret = match self.raw.as_ref().notify_move_or_resize_started {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn get_windowless_frame_rate(&mut self) -> i32 {
    unsafe {
      let ret = match self.raw.as_ref().get_windowless_frame_rate {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn set_windowless_frame_rate(&mut self, frame_rate: i32) -> () {
    unsafe {
      let ret = match self.raw.as_ref().set_windowless_frame_rate {
        Some(f) => f(self.raw.as_ptr(),frame_rate,),
        None => panic!(),
      };
      ret
    }
  }
  pub fn ime_commit_text(&mut self, text: Option<&crate::include::internal::CefString>, replacement_range: &crate::include::internal::CefRange, relative_cursor_pos: i32) -> () {
    unsafe {
      let ret = match self.raw.as_ref().ime_commit_text {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(text),&replacement_range.raw,relative_cursor_pos,),
        None => panic!(),
      };
      ret
    }
  }
  pub fn ime_finish_composing_text(&mut self, keep_selection: bool) -> () {
    unsafe {
      let ret = match self.raw.as_ref().ime_finish_composing_text {
        Some(f) => f(self.raw.as_ptr(),if keep_selection { 1 } else { 0 },),
        None => panic!(),
      };
      ret
    }
  }
  pub fn ime_cancel_composition(&mut self) -> () {
    unsafe {
      let ret = match self.raw.as_ref().ime_cancel_composition {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn drag_target_drag_enter(&mut self, drag_data: crate::include::CefDragData, event: &crate::include::internal::CefMouseEvent, allowed_ops: crate::include::internal::CefDragOperationsMask) -> () {
    unsafe {
      let ret = match self.raw.as_ref().drag_target_drag_enter {
        Some(f) => f(self.raw.as_ptr(),crate::include::CefDragData::to_cef_own(drag_data),&event.raw,allowed_ops.into(),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn drag_target_drag_over(&mut self, event: &crate::include::internal::CefMouseEvent, allowed_ops: crate::include::internal::CefDragOperationsMask) -> () {
    unsafe {
      let ret = match self.raw.as_ref().drag_target_drag_over {
        Some(f) => f(self.raw.as_ptr(),&event.raw,allowed_ops.into(),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn drag_target_drag_leave(&mut self) -> () {
    unsafe {
      let ret = match self.raw.as_ref().drag_target_drag_leave {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn drag_target_drop(&mut self, event: &crate::include::internal::CefMouseEvent) -> () {
    unsafe {
      let ret = match self.raw.as_ref().drag_target_drop {
        Some(f) => f(self.raw.as_ptr(),&event.raw,),
        None => panic!(),
      };
      ret
    }
  }
  pub fn drag_source_ended_at(&mut self, x: i32, y: i32, op: crate::include::internal::CefDragOperationsMask) -> () {
    unsafe {
      let ret = match self.raw.as_ref().drag_source_ended_at {
        Some(f) => f(self.raw.as_ptr(),x,y,op.into(),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn drag_source_system_drag_ended(&mut self) -> () {
    unsafe {
      let ret = match self.raw.as_ref().drag_source_system_drag_ended {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn get_visible_navigation_entry(&mut self) -> Option<crate::include::CefNavigationEntry> {
    unsafe {
      let ret = match self.raw.as_ref().get_visible_navigation_entry {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefNavigationEntry::from_cef_own(ret)
    }
  }
  pub fn set_accessibility_state(&mut self, accessibility_state: crate::include::internal::CefState) -> () {
    unsafe {
      let ret = match self.raw.as_ref().set_accessibility_state {
        Some(f) => f(self.raw.as_ptr(),accessibility_state.into(),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn set_auto_resize_enabled(&mut self, enabled: bool, min_size: &crate::include::internal::CefSize, max_size: &crate::include::internal::CefSize) -> () {
    unsafe {
      let ret = match self.raw.as_ref().set_auto_resize_enabled {
        Some(f) => f(self.raw.as_ptr(),if enabled { 1 } else { 0 },&min_size.raw,&max_size.raw,),
        None => panic!(),
      };
      ret
    }
  }
  pub fn get_extension(&mut self) -> Option<crate::include::CefExtension> {
    unsafe {
      let ret = match self.raw.as_ref().get_extension {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefExtension::from_cef_own(ret)
    }
  }
  pub fn is_background_host(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_background_host {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn set_audio_muted(&mut self, mute: bool) -> () {
    unsafe {
      let ret = match self.raw.as_ref().set_audio_muted {
        Some(f) => f(self.raw.as_ptr(),if mute { 1 } else { 0 },),
        None => panic!(),
      };
      ret
    }
  }
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
