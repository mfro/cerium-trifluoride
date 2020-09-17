pub type CefWaitableEvent = crate::include::base::CefProxy<cef_sys::cef_waitable_event_t>;
#[allow(non_snake_case)]
impl CefWaitableEvent {
  /// Create a new waitable event. If |automatic_reset| is true then the event
  /// state is automatically reset to un-signaled after a single waiting thread
  /// has been released; otherwise, the state remains signaled until Reset() is
  /// called manually. If |initially_signaled| is true then the event will start
  /// in the signaled state.
  #[allow(non_snake_case)]
  pub fn create_waitable_event(automatic_reset: bool, initially_signaled: bool, ) -> Option<crate::include::CefWaitableEvent> {
    unsafe {
      let ret = cef_sys::cef_waitable_event_create(if automatic_reset { 1 } else { 0 },if initially_signaled { 1 } else { 0 },);
      crate::include::CefWaitableEvent::from_cef_own(ret)
    }
  }
  /// Put the event in the un-signaled state.
  pub fn reset(&mut self) -> () {
    unsafe {
      let ret = match self.raw.as_ref().reset {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  /// Put the event in the signaled state. This causes any thread blocked on Wait
  /// to be woken up.
  pub fn signal(&mut self) -> () {
    unsafe {
      let ret = match self.raw.as_ref().signal {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  /// Returns true if the event is in the signaled state, else false. If the
  /// event was created with |automatic_reset| set to true then calling this
  /// method will also cause a reset.
  pub fn is_signaled(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_signaled {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Wait indefinitely for the event to be signaled. This method will not return
  /// until after the call to Signal() has completed. This method cannot be
  /// called on the browser process UI or IO threads.
  pub fn wait(&mut self) -> () {
    unsafe {
      let ret = match self.raw.as_ref().wait {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  /// Wait up to |max_ms| milliseconds for the event to be signaled. Returns true
  /// if the event was signaled. A return value of false does not necessarily
  /// mean that |max_ms| was exceeded. This method will not return until after
  /// the call to Signal() has completed. This method cannot be called on the
  /// browser process UI or IO threads.
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
