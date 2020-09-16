pub type CefThread = crate::include::base::CefProxy<cef_sys::cef_thread_t>;
#[allow(non_snake_case)]
impl CefThread {
  pub fn get_task_runner(&mut self) -> Option<crate::include::CefTaskRunner> {
    unsafe {
      let ret = match self.raw.as_ref().get_task_runner {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefTaskRunner::from_cef_own(ret)
    }
  }
  pub fn get_platform_thread_id(&mut self) -> crate::include::internal::CefPlatformThreadId {
    unsafe {
      let ret = match self.raw.as_ref().get_platform_thread_id {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret.into()
    }
  }
  pub fn stop(&mut self) -> () {
    unsafe {
      let ret = match self.raw.as_ref().stop {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn is_running(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_running {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
}
