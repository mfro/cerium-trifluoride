pub type CefWaitableEvent = crate::include::base::CefProxy<cef_sys::cef_waitable_event_t>;
#[allow(non_snake_case)]
impl CefWaitableEvent {
  pub fn reset(&mut self) -> () {
    unsafe {
      let ret = match self.raw.as_ref().reset {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn signal(&mut self) -> () {
    unsafe {
      let ret = match self.raw.as_ref().signal {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn is_signaled(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_signaled {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn wait(&mut self) -> () {
    unsafe {
      let ret = match self.raw.as_ref().wait {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn timed_wait(&mut self, max_ms: i64) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().timed_wait {
        Some(f) => f(self.raw.as_ptr(),max_ms,),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
}
