/// Implement this interface to receive notification when tracing has completed.
/// The methods of this class will be called on the browser process UI thread.
#[allow(non_snake_case)]
#[allow(unused_variables)]
pub trait EndTracingCallback {
  /// Called after all processes have sent their trace data. |tracing_file| is
  /// the path at which tracing data was written. The client is responsible for
  /// deleting |tracing_file|.
  fn on_end_tracing_complete(&mut self, tracing_file: &crate::include::internal::CefString) -> () { Default::default() }
}
define_refcounted!(EndTracingCallback, CefEndTracingCallback, cef_end_tracing_callback_t, on_end_tracing_complete: cef_end_tracing_callback_t_on_end_tracing_complete,);
#[allow(non_snake_case)]
unsafe extern "C" fn cef_end_tracing_callback_t_on_end_tracing_complete(_self: *mut cef_sys::cef_end_tracing_callback_t, tracing_file: *const cef_sys::cef_string_t) -> () {
  let ret = CefEndTracingCallback::from_cef(_self, true).get().on_end_tracing_complete(&*(tracing_file as *const _),);
  ret
}
/// Start tracing events on all processes. Tracing is initialized asynchronously
/// and |callback| will be executed on the UI thread after initialization is
/// complete.
/// 
/// If CefBeginTracing was called previously, or if a CefEndTracingAsync call is
/// pending, CefBeginTracing will fail and return false.
/// 
/// |categories| is a comma-delimited list of category wildcards. A category can
/// have an optional '-' prefix to make it an excluded category. Having both
/// included and excluded categories in the same list is not supported.
/// 
/// Example: "test_MyTest*"
/// Example: "test_MyTest*,test_OtherStuff"
/// Example: "-excluded_category1,-excluded_category2"
/// 
/// This function must be called on the browser process UI thread.
#[allow(non_snake_case)]
pub fn cef_begin_tracing(categories: Option<&crate::include::internal::CefString>, callback: Option<crate::include::CefCompletionCallback>, ) -> bool {
  unsafe {
    let ret = cef_sys::cef_begin_tracing(match categories { Some(categories) => categories as *const _ as *const _, None => std::ptr::null_mut() },callback.map_or(std::ptr::null_mut(), |o| crate::include::CefCompletionCallback::to_cef_own(o)),);
    if ret == 0 { false } else { true }
  }
}
/// Stop tracing events on all processes.
/// 
/// This function will fail and return false if a previous call to
/// CefEndTracingAsync is already pending or if CefBeginTracing was not called.
/// 
/// |tracing_file| is the path at which tracing data will be written and
/// |callback| is the callback that will be executed once all processes have
/// sent their trace data. If |tracing_file| is empty a new temporary file path
/// will be used. If |callback| is empty no trace data will be written.
/// 
/// This function must be called on the browser process UI thread.
#[allow(non_snake_case)]
pub fn cef_end_tracing(tracing_file: Option<&crate::include::internal::CefString>, callback: Option<crate::include::CefEndTracingCallback>, ) -> bool {
  unsafe {
    let ret = cef_sys::cef_end_tracing(match tracing_file { Some(tracing_file) => tracing_file as *const _ as *const _, None => std::ptr::null_mut() },callback.map_or(std::ptr::null_mut(), |o| crate::include::CefEndTracingCallback::to_cef_own(o)),);
    if ret == 0 { false } else { true }
  }
}
/// Returns the current system trace time or, if none is defined, the current
/// high-res time. Can be used by clients to synchronize with the time
/// information in trace events.
#[allow(non_snake_case)]
pub fn cef_now_from_system_trace_time() -> i64 {
  unsafe {
    let ret = cef_sys::cef_now_from_system_trace_time();
    ret
  }
}
