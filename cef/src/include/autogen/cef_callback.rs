pub type CefCallback = crate::include::base::CefProxy<cef_sys::cef_callback_t>;
#[allow(non_snake_case)]
impl CefCallback {
  /// Continue processing.
  pub fn cont(&mut self) -> () {
    unsafe {
      let ret = match self.raw.as_ref().cont {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  /// Cancel processing.
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
/// Generic callback interface used for asynchronous completion.
#[allow(non_snake_case)]
#[allow(unused_variables)]
pub trait CompletionCallback {
  /// Method that will be called once the task is complete.
  fn on_complete(&mut self) -> () { Default::default() }
}
define_refcounted!(CompletionCallback, CefCompletionCallback, cef_completion_callback_t, on_complete: cef_completion_callback_t_on_complete,);
#[allow(non_snake_case)]
unsafe extern "C" fn cef_completion_callback_t_on_complete(_self: *mut cef_sys::cef_completion_callback_t) -> () {
  let ret = CefCompletionCallback::from_cef(_self, true).get().on_complete();
  ret
}
