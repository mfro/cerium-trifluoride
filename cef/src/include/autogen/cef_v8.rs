pub type CefV8Context = crate::include::base::CefProxy<cef_sys::cef_v8context_t>;
#[allow(non_snake_case)]
impl CefV8Context {
  pub fn get_task_runner(&mut self) -> Option<crate::include::CefTaskRunner> {
    unsafe {
      let ret = match self.raw.as_ref().get_task_runner {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefTaskRunner::from_cef_own(ret)
    }
  }
  pub fn is_valid(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_valid {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn get_browser(&mut self) -> Option<crate::include::CefBrowser> {
    unsafe {
      let ret = match self.raw.as_ref().get_browser {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefBrowser::from_cef_own(ret)
    }
  }
  pub fn get_frame(&mut self) -> Option<crate::include::CefFrame> {
    unsafe {
      let ret = match self.raw.as_ref().get_frame {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefFrame::from_cef_own(ret)
    }
  }
  pub fn get_global(&mut self) -> Option<crate::include::CefV8Value> {
    unsafe {
      let ret = match self.raw.as_ref().get_global {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefV8Value::from_cef_own(ret)
    }
  }
  pub fn enter(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().enter {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn exit(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().exit {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn is_same(&mut self, that: crate::include::CefV8Context) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_same {
        Some(f) => f(self.raw.as_ptr(),crate::include::CefV8Context::to_cef_own(that),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn eval(&mut self, code: &crate::include::internal::CefString, script_url: Option<&crate::include::internal::CefString>, start_line: i32, retval: &mut crate::include::CefV8Value, exception: &mut crate::include::CefV8Exception) -> bool {
    unsafe {
      let mut retval__tmp = crate::include::CefV8Value::to_cef_ref(retval);
      let mut exception__tmp = crate::include::CefV8Exception::to_cef_ref(exception);
      let ret = match self.raw.as_ref().eval {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(code),crate::include::internal::IntoCef::into_cef(script_url),start_line,&mut retval__tmp,&mut exception__tmp,),
        None => panic!(),
      };
      *retval = crate::include::CefV8Value::from_cef_own(retval__tmp).unwrap();
      *exception = crate::include::CefV8Exception::from_cef_own(exception__tmp).unwrap();
      if ret == 0 { false } else { true }
    }
  }
}
/// Interface that should be implemented to handle V8 function calls. The methods
/// of this class will be called on the thread associated with the V8 function.
#[allow(non_snake_case)]
pub trait V8Handler {
}
define_refcounted!(V8Handler, v8handler, );
/// Interface that should be implemented to handle V8 accessor calls. Accessor
/// identifiers are registered by calling CefV8Value::SetValue(). The methods
/// of this class will be called on the thread associated with the V8 accessor.
#[allow(non_snake_case)]
pub trait V8Accessor {
}
define_refcounted!(V8Accessor, v8accessor, );
/// Interface that should be implemented to handle V8 interceptor calls. The
/// methods of this class will be called on the thread associated with the V8
/// interceptor. Interceptor's named property handlers (with first argument of
/// type CefString) are called when object is indexed by string. Indexed property
/// handlers (with first argument of type int) are called when object is indexed
/// by integer.
#[allow(non_snake_case)]
pub trait V8Interceptor {
}
define_refcounted!(V8Interceptor, v8interceptor, );
pub type CefV8Exception = crate::include::base::CefProxy<cef_sys::cef_v8exception_t>;
#[allow(non_snake_case)]
impl CefV8Exception {
  pub fn get_message(&mut self) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_message {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  pub fn get_source_line(&mut self) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_source_line {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  pub fn get_script_resource_name(&mut self) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_script_resource_name {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  pub fn get_line_number(&mut self) -> i32 {
    unsafe {
      let ret = match self.raw.as_ref().get_line_number {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn get_start_position(&mut self) -> i32 {
    unsafe {
      let ret = match self.raw.as_ref().get_start_position {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn get_end_position(&mut self) -> i32 {
    unsafe {
      let ret = match self.raw.as_ref().get_end_position {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn get_start_column(&mut self) -> i32 {
    unsafe {
      let ret = match self.raw.as_ref().get_start_column {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn get_end_column(&mut self) -> i32 {
    unsafe {
      let ret = match self.raw.as_ref().get_end_column {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
}
/// Callback interface that is passed to CefV8Value::CreateArrayBuffer.
#[allow(non_snake_case)]
pub trait V8ArrayBufferReleaseCallback {
  fn release_buffer(&mut self, buffer: &mut std::os::raw::c_void) -> () { Default::default() }
}
define_refcounted!(V8ArrayBufferReleaseCallback, v8array_buffer_release_callback, release_buffer,);
#[allow(non_snake_case)]
unsafe extern "C" fn cef_v8array_buffer_release_callback_t_release_buffer(_self: *mut cef_sys::cef_v8array_buffer_release_callback_t, buffer: *mut std::os::raw::c_void) -> () {
  let ret = CefV8ArrayBufferReleaseCallback::from_cef(_self, true).get().release_buffer(&mut *buffer,);
  ret
}
pub type CefV8Value = crate::include::base::CefProxy<cef_sys::cef_v8value_t>;
#[allow(non_snake_case)]
impl CefV8Value {
  pub fn is_valid(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_valid {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn is_undefined(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_undefined {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn is_null(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_null {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn is_bool(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_bool {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn is_int(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_int {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn is_uint(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_uint {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn is_double(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_double {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn is_date(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_date {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn is_string(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_string {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn is_object(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_object {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn is_array(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_array {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn is_array_buffer(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_array_buffer {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn is_function(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_function {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn is_same(&mut self, that: crate::include::CefV8Value) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_same {
        Some(f) => f(self.raw.as_ptr(),crate::include::CefV8Value::to_cef_own(that),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn get_bool_value(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().get_bool_value {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn get_uint_value(&mut self) -> u32 {
    unsafe {
      let ret = match self.raw.as_ref().get_uint_value {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn get_double_value(&mut self) -> f64 {
    unsafe {
      let ret = match self.raw.as_ref().get_double_value {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn get_date_value(&mut self) -> crate::include::internal::CefTime {
    unsafe {
      let ret = match self.raw.as_ref().get_date_value {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret.into()
    }
  }
  pub fn get_string_value(&mut self) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_string_value {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  pub fn is_user_created(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_user_created {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn has_exception(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().has_exception {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn get_exception(&mut self) -> Option<crate::include::CefV8Exception> {
    unsafe {
      let ret = match self.raw.as_ref().get_exception {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefV8Exception::from_cef_own(ret)
    }
  }
  pub fn clear_exception(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().clear_exception {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn will_rethrow_exceptions(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().will_rethrow_exceptions {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn set_rethrow_exceptions(&mut self, rethrow: bool) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().set_rethrow_exceptions {
        Some(f) => f(self.raw.as_ptr(),if rethrow { 1 } else { 0 },),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn has_value_bykey(&mut self, key: Option<&crate::include::internal::CefString>) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().has_value_bykey {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(key),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn has_value_byindex(&mut self, index: i32) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().has_value_byindex {
        Some(f) => f(self.raw.as_ptr(),index,),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn delete_value_bykey(&mut self, key: Option<&crate::include::internal::CefString>) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().delete_value_bykey {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(key),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn delete_value_byindex(&mut self, index: i32) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().delete_value_byindex {
        Some(f) => f(self.raw.as_ptr(),index,),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn get_value_bykey(&mut self, key: Option<&crate::include::internal::CefString>) -> Option<crate::include::CefV8Value> {
    unsafe {
      let ret = match self.raw.as_ref().get_value_bykey {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(key),),
        None => panic!(),
      };
      crate::include::CefV8Value::from_cef_own(ret)
    }
  }
  pub fn get_value_byindex(&mut self, index: i32) -> Option<crate::include::CefV8Value> {
    unsafe {
      let ret = match self.raw.as_ref().get_value_byindex {
        Some(f) => f(self.raw.as_ptr(),index,),
        None => panic!(),
      };
      crate::include::CefV8Value::from_cef_own(ret)
    }
  }
  pub fn set_value_bykey(&mut self, key: Option<&crate::include::internal::CefString>, value: crate::include::CefV8Value, attribute: crate::include::internal::CefV8Propertyattribute) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().set_value_bykey {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(key),crate::include::CefV8Value::to_cef_own(value),attribute.into(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn set_value_byindex(&mut self, index: i32, value: crate::include::CefV8Value) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().set_value_byindex {
        Some(f) => f(self.raw.as_ptr(),index,crate::include::CefV8Value::to_cef_own(value),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn set_value_byaccessor(&mut self, key: Option<&crate::include::internal::CefString>, settings: crate::include::internal::CefV8Accesscontrol, attribute: crate::include::internal::CefV8Propertyattribute) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().set_value_byaccessor {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(key),settings.into(),attribute.into(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn get_externally_allocated_memory(&mut self) -> i32 {
    unsafe {
      let ret = match self.raw.as_ref().get_externally_allocated_memory {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn adjust_externally_allocated_memory(&mut self, change_in_bytes: i32) -> i32 {
    unsafe {
      let ret = match self.raw.as_ref().adjust_externally_allocated_memory {
        Some(f) => f(self.raw.as_ptr(),change_in_bytes,),
        None => panic!(),
      };
      ret
    }
  }
  pub fn get_array_length(&mut self) -> i32 {
    unsafe {
      let ret = match self.raw.as_ref().get_array_length {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn get_array_buffer_release_callback(&mut self) -> Option<crate::include::CefV8ArrayBufferReleaseCallback> {
    unsafe {
      let ret = match self.raw.as_ref().get_array_buffer_release_callback {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefV8ArrayBufferReleaseCallback::from_cef_own(ret)
    }
  }
  pub fn neuter_array_buffer(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().neuter_array_buffer {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn get_function_name(&mut self) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_function_name {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  pub fn get_function_handler(&mut self) -> Option<crate::include::CefV8Handler> {
    unsafe {
      let ret = match self.raw.as_ref().get_function_handler {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefV8Handler::from_cef_own(ret)
    }
  }
}
pub type CefV8StackTrace = crate::include::base::CefProxy<cef_sys::cef_v8stack_trace_t>;
#[allow(non_snake_case)]
impl CefV8StackTrace {
  pub fn is_valid(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_valid {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn get_frame_count(&mut self) -> i32 {
    unsafe {
      let ret = match self.raw.as_ref().get_frame_count {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn get_frame(&mut self, index: i32) -> Option<crate::include::CefV8StackFrame> {
    unsafe {
      let ret = match self.raw.as_ref().get_frame {
        Some(f) => f(self.raw.as_ptr(),index,),
        None => panic!(),
      };
      crate::include::CefV8StackFrame::from_cef_own(ret)
    }
  }
}
pub type CefV8StackFrame = crate::include::base::CefProxy<cef_sys::cef_v8stack_frame_t>;
#[allow(non_snake_case)]
impl CefV8StackFrame {
  pub fn is_valid(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_valid {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn get_script_name(&mut self) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_script_name {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  pub fn get_script_name_or_source_url(&mut self) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_script_name_or_source_url {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  pub fn get_function_name(&mut self) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_function_name {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  pub fn get_line_number(&mut self) -> i32 {
    unsafe {
      let ret = match self.raw.as_ref().get_line_number {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn get_column(&mut self) -> i32 {
    unsafe {
      let ret = match self.raw.as_ref().get_column {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn is_eval(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_eval {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn is_constructor(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_constructor {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
}
