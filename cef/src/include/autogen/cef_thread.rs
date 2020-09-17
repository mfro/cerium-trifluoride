pub type CefThread = crate::include::base::CefProxy<cef_sys::cef_thread_t>;
#[allow(non_snake_case)]
impl CefThread {
  /// Create and start a new thread. This method does not block waiting for the
  /// thread to run initialization. |display_name| is the name that will be used
  /// to identify the thread. |priority| is the thread execution priority.
  /// |message_loop_type| indicates the set of asynchronous events that the
  /// thread can process. If |stoppable| is true the thread will stopped and
  /// joined on destruction or when Stop() is called; otherwise, the the thread
  /// cannot be stopped and will be leaked on shutdown. On Windows the
  /// |com_init_mode| value specifies how COM will be initialized for the thread.
  /// If |com_init_mode| is set to COM_INIT_MODE_STA then |message_loop_type|
  /// must be set to ML_TYPE_UI.
  #[allow(non_snake_case)]
  pub fn create_thread(display_name: Option<&crate::include::internal::CefString>, priority: crate::include::internal::CefThreadPriority, message_loop_type: crate::include::internal::CefMessageLoopType, stoppable: bool, com_init_mode: crate::include::internal::CefComInitMode, ) -> Option<crate::include::CefThread> {
    unsafe {
      let ret = cef_sys::cef_thread_create(crate::include::internal::IntoCef::into_cef(display_name),priority.into(),message_loop_type.into(),if stoppable { 1 } else { 0 },com_init_mode.into(),);
      crate::include::CefThread::from_cef_own(ret)
    }
  }
  /// Returns the CefTaskRunner that will execute code on this thread's message
  /// loop. This method is safe to call from any thread.
  pub fn get_task_runner(&mut self) -> Option<crate::include::CefTaskRunner> {
    unsafe {
      let ret = match self.raw.as_ref().get_task_runner {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefTaskRunner::from_cef_own(ret)
    }
  }
  /// Returns the platform thread ID. It will return the same value after Stop()
  /// is called. This method is safe to call from any thread.
  pub fn get_platform_thread_id(&mut self) -> crate::include::internal::CefPlatformThreadId {
    unsafe {
      let ret = match self.raw.as_ref().get_platform_thread_id {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret.into()
    }
  }
  /// Stop and join the thread. This method must be called from the same thread
  /// that called CreateThread(). Do not call this method if CreateThread() was
  /// called with a |stoppable| value of false.
  pub fn stop(&mut self) -> () {
    unsafe {
      let ret = match self.raw.as_ref().stop {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  /// Returns true if the thread is currently running. This method must be called
  /// from the same thread that called CreateThread().
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
