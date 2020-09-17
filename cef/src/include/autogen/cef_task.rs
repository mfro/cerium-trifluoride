/// Implement this interface for asynchronous task execution. If the task is
/// posted successfully and if the associated message loop is still running then
/// the Execute() method will be called on the target thread. If the task fails
/// to post then the task object may be destroyed on the source thread instead of
/// the target thread. For this reason be cautious when performing work in the
/// task object destructor.
#[allow(non_snake_case)]
#[allow(unused_variables)]
pub trait Task {
  /// Method that will be executed on the target thread.
  fn execute(&mut self) -> () { Default::default() }
}
define_refcounted!(Task, CefTask, cef_task_t, execute: cef_task_t_execute,);
#[allow(non_snake_case)]
unsafe extern "C" fn cef_task_t_execute(_self: *mut cef_sys::cef_task_t) -> () {
  let ret = CefTask::from_cef(_self, true).get().execute();
  ret
}
pub type CefTaskRunner = crate::include::refcounting::CefProxy<cef_sys::cef_task_runner_t>;
#[allow(non_snake_case)]
impl CefTaskRunner {
  /// Returns the task runner for the current thread. Only CEF threads will have
  /// task runners. An empty reference will be returned if this method is called
  /// on an invalid thread.
  #[allow(non_snake_case)]
  pub fn get_for_current_thread() -> Option<crate::include::CefTaskRunner> {
    unsafe {
      let ret = cef_sys::cef_task_runner_get_for_current_thread();
      crate::include::CefTaskRunner::from_cef_own(ret)
    }
  }
  /// Returns the task runner for the specified CEF thread.
  #[allow(non_snake_case)]
  pub fn get_for_thread(threadId: crate::include::internal::CefThreadId, ) -> Option<crate::include::CefTaskRunner> {
    unsafe {
      let ret = cef_sys::cef_task_runner_get_for_thread(threadId.into(),);
      crate::include::CefTaskRunner::from_cef_own(ret)
    }
  }
  /// Returns true if this object is pointing to the same task runner as |that|
  /// object.
  pub fn is_same(&mut self, that: crate::include::CefTaskRunner) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_same {
        Some(f) => f(self.raw.as_ptr(),crate::include::CefTaskRunner::to_cef_own(that),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns true if this task runner belongs to the current thread.
  pub fn belongs_to_current_thread(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().belongs_to_current_thread {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns true if this task runner is for the specified CEF thread.
  pub fn belongs_to_thread(&mut self, threadId: crate::include::internal::CefThreadId) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().belongs_to_thread {
        Some(f) => f(self.raw.as_ptr(),threadId.into(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Post a task for execution on the thread associated with this task runner.
  /// Execution will occur asynchronously.
  pub fn post_task(&mut self, task: crate::include::CefTask) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().post_task {
        Some(f) => f(self.raw.as_ptr(),crate::include::CefTask::to_cef_own(task),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Post a task for delayed execution on the thread associated with this task
  /// runner. Execution will occur asynchronously. Delayed tasks are not
  /// supported on V8 WebWorker threads and will be executed without the
  /// specified delay.
  pub fn post_delayed_task(&mut self, task: crate::include::CefTask, delay_ms: i64) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().post_delayed_task {
        Some(f) => f(self.raw.as_ptr(),crate::include::CefTask::to_cef_own(task),delay_ms,),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
}
/// Returns true if called on the specified thread. Equivalent to using
/// CefTaskRunner::GetForThread(threadId)->BelongsToCurrentThread().
#[allow(non_snake_case)]
pub fn cef_currently_on(threadId: crate::include::internal::CefThreadId, ) -> bool {
  unsafe {
    let ret = cef_sys::cef_currently_on(threadId.into(),);
    if ret == 0 { false } else { true }
  }
}
/// Post a task for execution on the specified thread. Equivalent to
/// using CefTaskRunner::GetForThread(threadId)->PostTask(task).
#[allow(non_snake_case)]
pub fn cef_post_task(threadId: crate::include::internal::CefThreadId, task: crate::include::CefTask, ) -> bool {
  unsafe {
    let ret = cef_sys::cef_post_task(threadId.into(),crate::include::CefTask::to_cef_own(task),);
    if ret == 0 { false } else { true }
  }
}
/// Post a task for delayed execution on the specified thread. Equivalent to
/// using CefTaskRunner::GetForThread(threadId)->PostDelayedTask(task, delay_ms).
#[allow(non_snake_case)]
pub fn cef_post_delayed_task(threadId: crate::include::internal::CefThreadId, task: crate::include::CefTask, delay_ms: i64, ) -> bool {
  unsafe {
    let ret = cef_sys::cef_post_delayed_task(threadId.into(),crate::include::CefTask::to_cef_own(task),delay_ms,);
    if ret == 0 { false } else { true }
  }
}
