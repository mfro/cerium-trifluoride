/// Implement this interface for asynchronous task execution. If the task is
/// posted successfully and if the associated message loop is still running then
/// the Execute() method will be called on the target thread. If the task fails
/// to post then the task object may be destroyed on the source thread instead of
/// the target thread. For this reason be cautious when performing work in the
/// task object destructor.
#[allow(non_snake_case)]
pub trait Task {
  fn execute(&mut self) -> () { Default::default() }
}
define_refcounted!(Task, task, execute,);
#[allow(non_snake_case)]
unsafe extern "C" fn cef_task_t_execute(_self: *mut cef_sys::cef_task_t) -> () {
  let ret = CefTask::from_cef(_self, true).get().execute();
  ret
}
pub type CefTaskRunner = crate::include::base::CefProxy<cef_sys::cef_task_runner_t>;
#[allow(non_snake_case)]
impl CefTaskRunner {
  pub fn is_same(&mut self, that: crate::include::CefTaskRunner) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_same {
        Some(f) => f(self.raw.as_ptr(),crate::include::CefTaskRunner::to_cef_own(that),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn belongs_to_current_thread(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().belongs_to_current_thread {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn belongs_to_thread(&mut self, threadId: crate::include::internal::CefThreadId) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().belongs_to_thread {
        Some(f) => f(self.raw.as_ptr(),threadId.into(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn post_task(&mut self, task: crate::include::CefTask) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().post_task {
        Some(f) => f(self.raw.as_ptr(),crate::include::CefTask::to_cef_own(task),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
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
