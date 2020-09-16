/// Implement this interface to receive notification when tracing has completed.
/// The methods of this class will be called on the browser process UI thread.
#[allow(non_snake_case)]
pub trait EndTracingCallback {
  fn on_end_tracing_complete(&mut self, tracing_file: &crate::include::internal::CefString) -> () { Default::default() }
}
define_refcounted!(EndTracingCallback, end_tracing_callback, on_end_tracing_complete,);
#[allow(non_snake_case)]
unsafe extern "C" fn cef_end_tracing_callback_t_on_end_tracing_complete(_self: *mut cef_sys::cef_end_tracing_callback_t, tracing_file: *const cef_sys::cef_string_t) -> () {
  let ret = CefEndTracingCallback::from_cef(_self, true).get().on_end_tracing_complete(&crate::include::internal::CefString::from_cef(tracing_file).unwrap(),);
  ret
}
