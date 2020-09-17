pub type CefPrintDialogCallback = crate::include::refcounting::CefProxy<cef_sys::cef_print_dialog_callback_t>;
#[allow(non_snake_case)]
impl CefPrintDialogCallback {
  /// Continue printing with the specified |settings|.
  pub fn cont(&mut self, settings: crate::include::CefPrintSettings) -> () {
    unsafe {
      let ret = match self.raw.as_ref().cont {
        Some(f) => f(self.raw.as_ptr(),crate::include::CefPrintSettings::to_cef_own(settings),),
        None => panic!(),
      };
      ret
    }
  }
  /// Cancel the printing.
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
pub type CefPrintJobCallback = crate::include::refcounting::CefProxy<cef_sys::cef_print_job_callback_t>;
#[allow(non_snake_case)]
impl CefPrintJobCallback {
  /// Indicate completion of the print job.
  pub fn cont(&mut self) -> () {
    unsafe {
      let ret = match self.raw.as_ref().cont {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
}
/// Implement this interface to handle printing on Linux. Each browser will have
/// only one print job in progress at a time. The methods of this class will be
/// called on the browser process UI thread.
#[allow(non_snake_case)]
#[allow(unused_variables)]
pub trait PrintHandler {
  /// Called when printing has started for the specified |browser|. This method
  /// will be called before the other OnPrint*() methods and irrespective of how
  /// printing was initiated (e.g. CefBrowserHost::Print(), JavaScript
  /// window.print() or PDF extension print button).
  fn on_print_start(&mut self, browser: crate::include::CefBrowser) -> () { Default::default() }
  /// Synchronize |settings| with client state. If |get_defaults| is true then
  /// populate |settings| with the default print settings. Do not keep a
  /// reference to |settings| outside of this callback.
  fn on_print_settings(&mut self, browser: crate::include::CefBrowser, settings: crate::include::CefPrintSettings, get_defaults: bool) -> () { Default::default() }
  /// Show the print dialog. Execute |callback| once the dialog is dismissed.
  /// Return true if the dialog will be displayed or false to cancel the
  /// printing immediately.
  fn on_print_dialog(&mut self, browser: crate::include::CefBrowser, has_selection: bool, callback: crate::include::CefPrintDialogCallback) -> bool { Default::default() }
  /// Send the print job to the printer. Execute |callback| once the job is
  /// completed. Return true if the job will proceed or false to cancel the job
  /// immediately.
  fn on_print_job(&mut self, browser: crate::include::CefBrowser, document_name: &crate::include::internal::CefString, pdf_file_path: &crate::include::internal::CefString, callback: crate::include::CefPrintJobCallback) -> bool { Default::default() }
  /// Reset client state related to printing.
  fn on_print_reset(&mut self, browser: crate::include::CefBrowser) -> () { Default::default() }
  /// Return the PDF paper size in device units. Used in combination with
  /// CefBrowserHost::PrintToPDF().
  fn get_pdf_paper_size(&mut self, device_units_per_inch: i32) -> crate::include::internal::CefSize { Default::default() }
}
define_refcounted!(PrintHandler, CefPrintHandler, cef_print_handler_t, on_print_start: cef_print_handler_t_on_print_start,on_print_settings: cef_print_handler_t_on_print_settings,on_print_dialog: cef_print_handler_t_on_print_dialog,on_print_job: cef_print_handler_t_on_print_job,on_print_reset: cef_print_handler_t_on_print_reset,get_pdf_paper_size: cef_print_handler_t_get_pdf_paper_size,);
#[allow(non_snake_case)]
unsafe extern "C" fn cef_print_handler_t_on_print_start(_self: *mut cef_sys::cef_print_handler_t, browser: *mut cef_sys::cef_browser_t) -> () {
  let ret = CefPrintHandler::from_cef(_self, true).get().on_print_start(crate::include::CefBrowser::from_cef_own(browser).unwrap(),);
  ret
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_print_handler_t_on_print_settings(_self: *mut cef_sys::cef_print_handler_t, browser: *mut cef_sys::cef_browser_t, settings: *mut cef_sys::cef_print_settings_t, get_defaults: i32) -> () {
  let ret = CefPrintHandler::from_cef(_self, true).get().on_print_settings(crate::include::CefBrowser::from_cef_own(browser).unwrap(),crate::include::CefPrintSettings::from_cef_own(settings).unwrap(),if get_defaults == 0 { false } else { true },);
  ret
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_print_handler_t_on_print_dialog(_self: *mut cef_sys::cef_print_handler_t, browser: *mut cef_sys::cef_browser_t, has_selection: i32, callback: *mut cef_sys::cef_print_dialog_callback_t) -> i32 {
  let ret = CefPrintHandler::from_cef(_self, true).get().on_print_dialog(crate::include::CefBrowser::from_cef_own(browser).unwrap(),if has_selection == 0 { false } else { true },crate::include::CefPrintDialogCallback::from_cef_own(callback).unwrap(),);
  if ret { 1 } else { 0 }
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_print_handler_t_on_print_job(_self: *mut cef_sys::cef_print_handler_t, browser: *mut cef_sys::cef_browser_t, document_name: *const cef_sys::cef_string_t, pdf_file_path: *const cef_sys::cef_string_t, callback: *mut cef_sys::cef_print_job_callback_t) -> i32 {
  let ret = CefPrintHandler::from_cef(_self, true).get().on_print_job(crate::include::CefBrowser::from_cef_own(browser).unwrap(),&*(document_name as *const _),&*(pdf_file_path as *const _),crate::include::CefPrintJobCallback::from_cef_own(callback).unwrap(),);
  if ret { 1 } else { 0 }
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_print_handler_t_on_print_reset(_self: *mut cef_sys::cef_print_handler_t, browser: *mut cef_sys::cef_browser_t) -> () {
  let ret = CefPrintHandler::from_cef(_self, true).get().on_print_reset(crate::include::CefBrowser::from_cef_own(browser).unwrap(),);
  ret
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_print_handler_t_get_pdf_paper_size(_self: *mut cef_sys::cef_print_handler_t, device_units_per_inch: i32) -> cef_sys::cef_size_t {
  let ret = CefPrintHandler::from_cef(_self, true).get().get_pdf_paper_size(device_units_per_inch,);
  ret.into()
}
