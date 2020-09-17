pub type CefV8Context = crate::include::refcounting::CefProxy<cef_sys::cef_v8context_t>;
#[allow(non_snake_case)]
impl CefV8Context {
  /// Returns the current (top) context object in the V8 context stack.
  #[allow(non_snake_case)]
  pub fn get_current_context() -> Option<crate::include::CefV8Context> {
    unsafe {
      let ret = cef_sys::cef_v8context_get_current_context();
      crate::include::CefV8Context::from_cef_own(ret)
    }
  }
  /// Returns the entered (bottom) context object in the V8 context stack.
  #[allow(non_snake_case)]
  pub fn get_entered_context() -> Option<crate::include::CefV8Context> {
    unsafe {
      let ret = cef_sys::cef_v8context_get_entered_context();
      crate::include::CefV8Context::from_cef_own(ret)
    }
  }
  /// Returns true if V8 is currently inside a context.
  #[allow(non_snake_case)]
  pub fn in_context() -> bool {
    unsafe {
      let ret = cef_sys::cef_v8context_in_context();
      if ret == 0 { false } else { true }
    }
  }
  /// Returns the task runner associated with this context. V8 handles can only
  /// be accessed from the thread on which they are created. This method can be
  /// called on any render process thread.
  pub fn get_task_runner(&mut self) -> Option<crate::include::CefTaskRunner> {
    unsafe {
      let ret = match self.raw.as_ref().get_task_runner {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefTaskRunner::from_cef_own(ret)
    }
  }
  /// Returns true if the underlying handle is valid and it can be accessed on
  /// the current thread. Do not call any other methods if this method returns
  /// false.
  pub fn is_valid(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_valid {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns the browser for this context. This method will return an empty
  /// reference for WebWorker contexts.
  pub fn get_browser(&mut self) -> Option<crate::include::CefBrowser> {
    unsafe {
      let ret = match self.raw.as_ref().get_browser {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefBrowser::from_cef_own(ret)
    }
  }
  /// Returns the frame for this context. This method will return an empty
  /// reference for WebWorker contexts.
  pub fn get_frame(&mut self) -> Option<crate::include::CefFrame> {
    unsafe {
      let ret = match self.raw.as_ref().get_frame {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefFrame::from_cef_own(ret)
    }
  }
  /// Returns the global object for this context. The context must be entered
  /// before calling this method.
  pub fn get_global(&mut self) -> Option<crate::include::CefV8Value> {
    unsafe {
      let ret = match self.raw.as_ref().get_global {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefV8Value::from_cef_own(ret)
    }
  }
  /// Enter this context. A context must be explicitly entered before creating a
  /// V8 Object, Array, Function or Date asynchronously. Exit() must be called
  /// the same number of times as Enter() before releasing this context. V8
  /// objects belong to the context in which they are created. Returns true if
  /// the scope was entered successfully.
  pub fn enter(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().enter {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Exit this context. Call this method only after calling Enter(). Returns
  /// true if the scope was exited successfully.
  pub fn exit(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().exit {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns true if this object is pointing to the same handle as |that|
  /// object.
  pub fn is_same(&mut self, that: crate::include::CefV8Context) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_same {
        Some(f) => f(self.raw.as_ptr(),crate::include::CefV8Context::to_cef_own(that),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Execute a string of JavaScript code in this V8 context. The |script_url|
  /// parameter is the URL where the script in question can be found, if any.
  /// The |start_line| parameter is the base line number to use for error
  /// reporting. On success |retval| will be set to the return value, if any, and
  /// the function will return true. On failure |exception| will be set to the
  /// exception, if any, and the function will return false.
  pub fn eval(&mut self, code: &crate::include::internal::CefString, script_url: Option<&crate::include::internal::CefString>, start_line: i32, retval: &mut crate::include::CefV8Value, exception: &mut crate::include::CefV8Exception) -> bool {
    unsafe {
      let mut retval__tmp = crate::include::CefV8Value::to_cef_ref(retval);
      let mut exception__tmp = crate::include::CefV8Exception::to_cef_ref(exception);
      let ret = match self.raw.as_ref().eval {
        Some(f) => f(self.raw.as_ptr(),code as *const _ as *const _,match script_url { Some(script_url) => script_url as *const _ as *const _, None => std::ptr::null_mut() },start_line,&mut retval__tmp,&mut exception__tmp,),
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
#[allow(unused_variables)]
pub trait V8Handler {
}
define_refcounted!(V8Handler, CefV8Handler, cef_v8handler_t, );
/// Interface that should be implemented to handle V8 accessor calls. Accessor
/// identifiers are registered by calling CefV8Value::SetValue(). The methods
/// of this class will be called on the thread associated with the V8 accessor.
#[allow(non_snake_case)]
#[allow(unused_variables)]
pub trait V8Accessor {
  /// Handle retrieval the accessor value identified by |name|. |object| is the
  /// receiver ('this' object) of the accessor. If retrieval succeeds set
  /// |retval| to the return value. If retrieval fails set |exception| to the
  /// exception that will be thrown. Return true if accessor retrieval was
  /// handled.
  fn get(&mut self, name: &crate::include::internal::CefString, object: &crate::include::CefV8Value, retval: &mut crate::include::CefV8Value, exception: &mut crate::include::internal::CefString) -> bool { Default::default() }
  /// Handle assignment of the accessor value identified by |name|. |object| is
  /// the receiver ('this' object) of the accessor. |value| is the new value
  /// being assigned to the accessor. If assignment fails set |exception| to the
  /// exception that will be thrown. Return true if accessor assignment was
  /// handled.
  fn set(&mut self, name: &crate::include::internal::CefString, object: &crate::include::CefV8Value, value: &crate::include::CefV8Value, exception: &mut crate::include::internal::CefString) -> bool { Default::default() }
}
define_refcounted!(V8Accessor, CefV8Accessor, cef_v8accessor_t, get: cef_v8accessor_t_get,set: cef_v8accessor_t_set,);
#[allow(non_snake_case)]
unsafe extern "C" fn cef_v8accessor_t_get(_self: *mut cef_sys::cef_v8accessor_t, name: *const cef_sys::cef_string_t, object: *mut cef_sys::cef_v8value_t, retval: *mut *mut cef_sys::cef_v8value_t, exception: *mut cef_sys::cef_string_t) -> i32 {
  let mut retval__tmp = crate::include::CefV8Value::from_cef_own(*retval).unwrap();
  let ret = CefV8Accessor::from_cef(_self, true).get().get(&*(name as *const _),&*(object as *const _),&mut retval__tmp,&mut *(exception as *mut _),);
  *retval = crate::include::CefV8Value::to_cef_own(retval__tmp);
  if ret { 1 } else { 0 }
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_v8accessor_t_set(_self: *mut cef_sys::cef_v8accessor_t, name: *const cef_sys::cef_string_t, object: *mut cef_sys::cef_v8value_t, value: *mut cef_sys::cef_v8value_t, exception: *mut cef_sys::cef_string_t) -> i32 {
  let ret = CefV8Accessor::from_cef(_self, true).get().set(&*(name as *const _),&*(object as *const _),&*(value as *const _),&mut *(exception as *mut _),);
  if ret { 1 } else { 0 }
}
/// Interface that should be implemented to handle V8 interceptor calls. The
/// methods of this class will be called on the thread associated with the V8
/// interceptor. Interceptor's named property handlers (with first argument of
/// type CefString) are called when object is indexed by string. Indexed property
/// handlers (with first argument of type int) are called when object is indexed
/// by integer.
#[allow(non_snake_case)]
#[allow(unused_variables)]
pub trait V8Interceptor {
  /// Handle retrieval of the interceptor value identified by |name|. |object| is
  /// the receiver ('this' object) of the interceptor. If retrieval succeeds, set
  /// |retval| to the return value. If the requested value does not exist, don't
  /// set either |retval| or |exception|. If retrieval fails, set |exception| to
  /// the exception that will be thrown. If the property has an associated
  /// accessor, it will be called only if you don't set |retval|.
  /// Return true if interceptor retrieval was handled, false otherwise.
  fn get_byname(&mut self, name: &crate::include::internal::CefString, object: &crate::include::CefV8Value, retval: &mut crate::include::CefV8Value, exception: &mut crate::include::internal::CefString) -> bool { Default::default() }
  /// Handle retrieval of the interceptor value identified by |index|. |object|
  /// is the receiver ('this' object) of the interceptor. If retrieval succeeds,
  /// set |retval| to the return value. If the requested value does not exist,
  /// don't set either |retval| or |exception|. If retrieval fails, set
  /// |exception| to the exception that will be thrown.
  /// Return true if interceptor retrieval was handled, false otherwise.
  fn get_byindex(&mut self, index: i32, object: &crate::include::CefV8Value, retval: &mut crate::include::CefV8Value, exception: &mut crate::include::internal::CefString) -> bool { Default::default() }
  /// Handle assignment of the interceptor value identified by |name|. |object|
  /// is the receiver ('this' object) of the interceptor. |value| is the new
  /// value being assigned to the interceptor. If assignment fails, set
  /// |exception| to the exception that will be thrown. This setter will always
  /// be called, even when the property has an associated accessor.
  /// Return true if interceptor assignment was handled, false otherwise.
  fn set_byname(&mut self, name: &crate::include::internal::CefString, object: &crate::include::CefV8Value, value: &crate::include::CefV8Value, exception: &mut crate::include::internal::CefString) -> bool { Default::default() }
  /// Handle assignment of the interceptor value identified by |index|. |object|
  /// is the receiver ('this' object) of the interceptor. |value| is the new
  /// value being assigned to the interceptor. If assignment fails, set
  /// |exception| to the exception that will be thrown.
  /// Return true if interceptor assignment was handled, false otherwise.
  fn set_byindex(&mut self, index: i32, object: &crate::include::CefV8Value, value: &crate::include::CefV8Value, exception: &mut crate::include::internal::CefString) -> bool { Default::default() }
}
define_refcounted!(V8Interceptor, CefV8Interceptor, cef_v8interceptor_t, get_byname: cef_v8interceptor_t_get_byname,get_byindex: cef_v8interceptor_t_get_byindex,set_byname: cef_v8interceptor_t_set_byname,set_byindex: cef_v8interceptor_t_set_byindex,);
#[allow(non_snake_case)]
unsafe extern "C" fn cef_v8interceptor_t_get_byname(_self: *mut cef_sys::cef_v8interceptor_t, name: *const cef_sys::cef_string_t, object: *mut cef_sys::cef_v8value_t, retval: *mut *mut cef_sys::cef_v8value_t, exception: *mut cef_sys::cef_string_t) -> i32 {
  let mut retval__tmp = crate::include::CefV8Value::from_cef_own(*retval).unwrap();
  let ret = CefV8Interceptor::from_cef(_self, true).get().get_byname(&*(name as *const _),&*(object as *const _),&mut retval__tmp,&mut *(exception as *mut _),);
  *retval = crate::include::CefV8Value::to_cef_own(retval__tmp);
  if ret { 1 } else { 0 }
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_v8interceptor_t_get_byindex(_self: *mut cef_sys::cef_v8interceptor_t, index: i32, object: *mut cef_sys::cef_v8value_t, retval: *mut *mut cef_sys::cef_v8value_t, exception: *mut cef_sys::cef_string_t) -> i32 {
  let mut retval__tmp = crate::include::CefV8Value::from_cef_own(*retval).unwrap();
  let ret = CefV8Interceptor::from_cef(_self, true).get().get_byindex(index,&*(object as *const _),&mut retval__tmp,&mut *(exception as *mut _),);
  *retval = crate::include::CefV8Value::to_cef_own(retval__tmp);
  if ret { 1 } else { 0 }
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_v8interceptor_t_set_byname(_self: *mut cef_sys::cef_v8interceptor_t, name: *const cef_sys::cef_string_t, object: *mut cef_sys::cef_v8value_t, value: *mut cef_sys::cef_v8value_t, exception: *mut cef_sys::cef_string_t) -> i32 {
  let ret = CefV8Interceptor::from_cef(_self, true).get().set_byname(&*(name as *const _),&*(object as *const _),&*(value as *const _),&mut *(exception as *mut _),);
  if ret { 1 } else { 0 }
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_v8interceptor_t_set_byindex(_self: *mut cef_sys::cef_v8interceptor_t, index: i32, object: *mut cef_sys::cef_v8value_t, value: *mut cef_sys::cef_v8value_t, exception: *mut cef_sys::cef_string_t) -> i32 {
  let ret = CefV8Interceptor::from_cef(_self, true).get().set_byindex(index,&*(object as *const _),&*(value as *const _),&mut *(exception as *mut _),);
  if ret { 1 } else { 0 }
}
pub type CefV8Exception = crate::include::refcounting::CefProxy<cef_sys::cef_v8exception_t>;
#[allow(non_snake_case)]
impl CefV8Exception {
  /// Returns the exception message.
  pub fn get_message(&mut self) -> crate::include::internal::CefStringUserFree {
    unsafe {
      let ret = match self.raw.as_ref().get_message {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefStringUserFree::from_cef(ret).unwrap()
    }
  }
  /// Returns the line of source code that the exception occurred within.
  pub fn get_source_line(&mut self) -> crate::include::internal::CefStringUserFree {
    unsafe {
      let ret = match self.raw.as_ref().get_source_line {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefStringUserFree::from_cef(ret).unwrap()
    }
  }
  /// Returns the resource name for the script from where the function causing
  /// the error originates.
  pub fn get_script_resource_name(&mut self) -> crate::include::internal::CefStringUserFree {
    unsafe {
      let ret = match self.raw.as_ref().get_script_resource_name {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefStringUserFree::from_cef(ret).unwrap()
    }
  }
  /// Returns the 1-based number of the line where the error occurred or 0 if the
  /// line number is unknown.
  pub fn get_line_number(&mut self) -> i32 {
    unsafe {
      let ret = match self.raw.as_ref().get_line_number {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  /// Returns the index within the script of the first character where the error
  /// occurred.
  pub fn get_start_position(&mut self) -> i32 {
    unsafe {
      let ret = match self.raw.as_ref().get_start_position {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  /// Returns the index within the script of the last character where the error
  /// occurred.
  pub fn get_end_position(&mut self) -> i32 {
    unsafe {
      let ret = match self.raw.as_ref().get_end_position {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  /// Returns the index within the line of the first character where the error
  /// occurred.
  pub fn get_start_column(&mut self) -> i32 {
    unsafe {
      let ret = match self.raw.as_ref().get_start_column {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  /// Returns the index within the line of the last character where the error
  /// occurred.
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
#[allow(unused_variables)]
pub trait V8ArrayBufferReleaseCallback {
  /// Called to release |buffer| when the ArrayBuffer JS object is garbage
  /// collected. |buffer| is the value that was passed to CreateArrayBuffer along
  /// with this object.
  fn release_buffer(&mut self, buffer: &mut std::os::raw::c_void) -> () { Default::default() }
}
define_refcounted!(V8ArrayBufferReleaseCallback, CefV8ArrayBufferReleaseCallback, cef_v8array_buffer_release_callback_t, release_buffer: cef_v8array_buffer_release_callback_t_release_buffer,);
#[allow(non_snake_case)]
unsafe extern "C" fn cef_v8array_buffer_release_callback_t_release_buffer(_self: *mut cef_sys::cef_v8array_buffer_release_callback_t, buffer: *mut std::os::raw::c_void) -> () {
  let ret = CefV8ArrayBufferReleaseCallback::from_cef(_self, true).get().release_buffer(&mut *buffer,);
  ret
}
pub type CefV8Value = crate::include::refcounting::CefProxy<cef_sys::cef_v8value_t>;
#[allow(non_snake_case)]
impl CefV8Value {
  /// Create a new CefV8Value object of type undefined.
  #[allow(non_snake_case)]
  pub fn create_undefined() -> Option<crate::include::CefV8Value> {
    unsafe {
      let ret = cef_sys::cef_v8value_create_undefined();
      crate::include::CefV8Value::from_cef_own(ret)
    }
  }
  /// Create a new CefV8Value object of type null.
  #[allow(non_snake_case)]
  pub fn create_null() -> Option<crate::include::CefV8Value> {
    unsafe {
      let ret = cef_sys::cef_v8value_create_null();
      crate::include::CefV8Value::from_cef_own(ret)
    }
  }
  /// Create a new CefV8Value object of type bool.
  #[allow(non_snake_case)]
  pub fn create_bool(value: bool, ) -> Option<crate::include::CefV8Value> {
    unsafe {
      let ret = cef_sys::cef_v8value_create_bool(if value { 1 } else { 0 },);
      crate::include::CefV8Value::from_cef_own(ret)
    }
  }
  /// Create a new CefV8Value object of type unsigned int.
  #[allow(non_snake_case)]
  pub fn create_uint(value: u32, ) -> Option<crate::include::CefV8Value> {
    unsafe {
      let ret = cef_sys::cef_v8value_create_uint(value,);
      crate::include::CefV8Value::from_cef_own(ret)
    }
  }
  /// Create a new CefV8Value object of type double.
  #[allow(non_snake_case)]
  pub fn create_double(value: f64, ) -> Option<crate::include::CefV8Value> {
    unsafe {
      let ret = cef_sys::cef_v8value_create_double(value,);
      crate::include::CefV8Value::from_cef_own(ret)
    }
  }
  /// Create a new CefV8Value object of type Date. This method should only be
  /// called from within the scope of a CefRenderProcessHandler, CefV8Handler or
  /// CefV8Accessor callback, or in combination with calling Enter() and Exit()
  /// on a stored CefV8Context reference.
  #[allow(non_snake_case)]
  pub fn create_date(date: &crate::include::internal::CefTime, ) -> Option<crate::include::CefV8Value> {
    unsafe {
      let ret = cef_sys::cef_v8value_create_date(date as *const _ as *const _,);
      crate::include::CefV8Value::from_cef_own(ret)
    }
  }
  /// Create a new CefV8Value object of type string.
  #[allow(non_snake_case)]
  pub fn create_string(value: Option<&crate::include::internal::CefString>, ) -> Option<crate::include::CefV8Value> {
    unsafe {
      let ret = cef_sys::cef_v8value_create_string(match value { Some(value) => value as *const _ as *const _, None => std::ptr::null_mut() },);
      crate::include::CefV8Value::from_cef_own(ret)
    }
  }
  /// Create a new CefV8Value object of type object with optional accessor and/or
  /// interceptor. This method should only be called from within the scope of a
  /// CefRenderProcessHandler, CefV8Handler or CefV8Accessor callback, or in
  /// combination with calling Enter() and Exit() on a stored CefV8Context
  /// reference.
  #[allow(non_snake_case)]
  pub fn create_object(accessor: Option<crate::include::CefV8Accessor>, interceptor: Option<crate::include::CefV8Interceptor>, ) -> Option<crate::include::CefV8Value> {
    unsafe {
      let ret = cef_sys::cef_v8value_create_object(accessor.map_or(std::ptr::null_mut(), |o| crate::include::CefV8Accessor::to_cef_own(o)),interceptor.map_or(std::ptr::null_mut(), |o| crate::include::CefV8Interceptor::to_cef_own(o)),);
      crate::include::CefV8Value::from_cef_own(ret)
    }
  }
  /// Create a new CefV8Value object of type array with the specified |length|.
  /// If |length| is negative the returned array will have length 0. This method
  /// should only be called from within the scope of a CefRenderProcessHandler,
  /// CefV8Handler or CefV8Accessor callback, or in combination with calling
  /// Enter() and Exit() on a stored CefV8Context reference.
  #[allow(non_snake_case)]
  pub fn create_array(length: i32, ) -> Option<crate::include::CefV8Value> {
    unsafe {
      let ret = cef_sys::cef_v8value_create_array(length,);
      crate::include::CefV8Value::from_cef_own(ret)
    }
  }
  /// Create a new CefV8Value object of type ArrayBuffer which wraps the provided
  /// |buffer| of size |length| bytes. The ArrayBuffer is externalized, meaning
  /// that it does not own |buffer|. The caller is responsible for freeing
  /// |buffer| when requested via a call to CefV8ArrayBufferReleaseCallback::
  /// ReleaseBuffer. This method should only be called from within the scope of a
  /// CefRenderProcessHandler, CefV8Handler or CefV8Accessor callback, or in
  /// combination with calling Enter() and Exit() on a stored CefV8Context
  /// reference.
  #[allow(non_snake_case)]
  pub fn create_array_buffer(buffer: &mut [u8], release_callback: crate::include::CefV8ArrayBufferReleaseCallback, ) -> Option<crate::include::CefV8Value> {
    unsafe {
      let ret = cef_sys::cef_v8value_create_array_buffer(buffer.as_mut_ptr() as *mut _,buffer.len() as _,crate::include::CefV8ArrayBufferReleaseCallback::to_cef_own(release_callback),);
      crate::include::CefV8Value::from_cef_own(ret)
    }
  }
  /// Create a new CefV8Value object of type function. This method should only be
  /// called from within the scope of a CefRenderProcessHandler, CefV8Handler or
  /// CefV8Accessor callback, or in combination with calling Enter() and Exit()
  /// on a stored CefV8Context reference.
  #[allow(non_snake_case)]
  pub fn create_function(name: &crate::include::internal::CefString, handler: crate::include::CefV8Handler, ) -> Option<crate::include::CefV8Value> {
    unsafe {
      let ret = cef_sys::cef_v8value_create_function(name as *const _ as *const _,crate::include::CefV8Handler::to_cef_own(handler),);
      crate::include::CefV8Value::from_cef_own(ret)
    }
  }
  /// Returns true if the underlying handle is valid and it can be accessed on
  /// the current thread. Do not call any other methods if this method returns
  /// false.
  pub fn is_valid(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_valid {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// True if the value type is undefined.
  pub fn is_undefined(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_undefined {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// True if the value type is null.
  pub fn is_null(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_null {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// True if the value type is bool.
  pub fn is_bool(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_bool {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// True if the value type is int.
  pub fn is_int(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_int {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// True if the value type is unsigned int.
  pub fn is_uint(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_uint {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// True if the value type is double.
  pub fn is_double(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_double {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// True if the value type is Date.
  pub fn is_date(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_date {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// True if the value type is string.
  pub fn is_string(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_string {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// True if the value type is object.
  pub fn is_object(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_object {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// True if the value type is array.
  pub fn is_array(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_array {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// True if the value type is an ArrayBuffer.
  pub fn is_array_buffer(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_array_buffer {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// True if the value type is function.
  pub fn is_function(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_function {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns true if this object is pointing to the same handle as |that|
  /// object.
  pub fn is_same(&mut self, that: crate::include::CefV8Value) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_same {
        Some(f) => f(self.raw.as_ptr(),crate::include::CefV8Value::to_cef_own(that),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Return a bool value.
  pub fn get_bool_value(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().get_bool_value {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Return an unsigned int value.
  pub fn get_uint_value(&mut self) -> u32 {
    unsafe {
      let ret = match self.raw.as_ref().get_uint_value {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  /// Return a double value.
  pub fn get_double_value(&mut self) -> f64 {
    unsafe {
      let ret = match self.raw.as_ref().get_double_value {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  /// Return a Date value.
  pub fn get_date_value(&mut self) -> crate::include::internal::CefTime {
    unsafe {
      let ret = match self.raw.as_ref().get_date_value {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret.into()
    }
  }
  /// Return a string value.
  pub fn get_string_value(&mut self) -> crate::include::internal::CefStringUserFree {
    unsafe {
      let ret = match self.raw.as_ref().get_string_value {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefStringUserFree::from_cef(ret).unwrap()
    }
  }
  /// OBJECT METHODS - These methods are only available on objects. Arrays and
  /// functions are also objects. String- and integer-based keys can be used
  /// interchangably with the framework converting between them as necessary.
  /// Returns true if this is a user created object.
  pub fn is_user_created(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_user_created {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns true if the last method call resulted in an exception. This
  /// attribute exists only in the scope of the current CEF value object.
  pub fn has_exception(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().has_exception {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns the exception resulting from the last method call. This attribute
  /// exists only in the scope of the current CEF value object.
  pub fn get_exception(&mut self) -> Option<crate::include::CefV8Exception> {
    unsafe {
      let ret = match self.raw.as_ref().get_exception {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefV8Exception::from_cef_own(ret)
    }
  }
  /// Clears the last exception and returns true on success.
  pub fn clear_exception(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().clear_exception {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns true if this object will re-throw future exceptions. This attribute
  /// exists only in the scope of the current CEF value object.
  pub fn will_rethrow_exceptions(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().will_rethrow_exceptions {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Set whether this object will re-throw future exceptions. By default
  /// exceptions are not re-thrown. If a exception is re-thrown the current
  /// context should not be accessed again until after the exception has been
  /// caught and not re-thrown. Returns true on success. This attribute exists
  /// only in the scope of the current CEF value object.
  pub fn set_rethrow_exceptions(&mut self, rethrow: bool) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().set_rethrow_exceptions {
        Some(f) => f(self.raw.as_ptr(),if rethrow { 1 } else { 0 },),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns true if the object has a value with the specified identifier.
  pub fn has_value_bykey(&mut self, key: Option<&crate::include::internal::CefString>) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().has_value_bykey {
        Some(f) => f(self.raw.as_ptr(),match key { Some(key) => key as *const _ as *const _, None => std::ptr::null_mut() },),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns true if the object has a value with the specified identifier.
  pub fn has_value_byindex(&mut self, index: i32) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().has_value_byindex {
        Some(f) => f(self.raw.as_ptr(),index,),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Deletes the value with the specified identifier and returns true on
  /// success. Returns false if this method is called incorrectly or an exception
  /// is thrown. For read-only and don't-delete values this method will return
  /// true even though deletion failed.
  pub fn delete_value_bykey(&mut self, key: Option<&crate::include::internal::CefString>) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().delete_value_bykey {
        Some(f) => f(self.raw.as_ptr(),match key { Some(key) => key as *const _ as *const _, None => std::ptr::null_mut() },),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Deletes the value with the specified identifier and returns true on
  /// success. Returns false if this method is called incorrectly, deletion fails
  /// or an exception is thrown. For read-only and don't-delete values this
  /// method will return true even though deletion failed.
  pub fn delete_value_byindex(&mut self, index: i32) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().delete_value_byindex {
        Some(f) => f(self.raw.as_ptr(),index,),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns the value with the specified identifier on success. Returns NULL
  /// if this method is called incorrectly or an exception is thrown.
  pub fn get_value_bykey(&mut self, key: Option<&crate::include::internal::CefString>) -> Option<crate::include::CefV8Value> {
    unsafe {
      let ret = match self.raw.as_ref().get_value_bykey {
        Some(f) => f(self.raw.as_ptr(),match key { Some(key) => key as *const _ as *const _, None => std::ptr::null_mut() },),
        None => panic!(),
      };
      crate::include::CefV8Value::from_cef_own(ret)
    }
  }
  /// Returns the value with the specified identifier on success. Returns NULL
  /// if this method is called incorrectly or an exception is thrown.
  pub fn get_value_byindex(&mut self, index: i32) -> Option<crate::include::CefV8Value> {
    unsafe {
      let ret = match self.raw.as_ref().get_value_byindex {
        Some(f) => f(self.raw.as_ptr(),index,),
        None => panic!(),
      };
      crate::include::CefV8Value::from_cef_own(ret)
    }
  }
  /// Associates a value with the specified identifier and returns true on
  /// success. Returns false if this method is called incorrectly or an exception
  /// is thrown. For read-only values this method will return true even though
  /// assignment failed.
  pub fn set_value_bykey(&mut self, key: Option<&crate::include::internal::CefString>, value: crate::include::CefV8Value, attribute: crate::include::internal::CefV8Propertyattribute) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().set_value_bykey {
        Some(f) => f(self.raw.as_ptr(),match key { Some(key) => key as *const _ as *const _, None => std::ptr::null_mut() },crate::include::CefV8Value::to_cef_own(value),attribute.into(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Associates a value with the specified identifier and returns true on
  /// success. Returns false if this method is called incorrectly or an exception
  /// is thrown. For read-only values this method will return true even though
  /// assignment failed.
  pub fn set_value_byindex(&mut self, index: i32, value: crate::include::CefV8Value) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().set_value_byindex {
        Some(f) => f(self.raw.as_ptr(),index,crate::include::CefV8Value::to_cef_own(value),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Registers an identifier and returns true on success. Access to the
  /// identifier will be forwarded to the CefV8Accessor instance passed to
  /// CefV8Value::CreateObject(). Returns false if this method is called
  /// incorrectly or an exception is thrown. For read-only values this method
  /// will return true even though assignment failed.
  pub fn set_value_byaccessor(&mut self, key: Option<&crate::include::internal::CefString>, settings: crate::include::internal::CefV8Accesscontrol, attribute: crate::include::internal::CefV8Propertyattribute) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().set_value_byaccessor {
        Some(f) => f(self.raw.as_ptr(),match key { Some(key) => key as *const _ as *const _, None => std::ptr::null_mut() },settings.into(),attribute.into(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns the amount of externally allocated memory registered for the
  /// object.
  pub fn get_externally_allocated_memory(&mut self) -> i32 {
    unsafe {
      let ret = match self.raw.as_ref().get_externally_allocated_memory {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  /// Adjusts the amount of registered external memory for the object. Used to
  /// give V8 an indication of the amount of externally allocated memory that is
  /// kept alive by JavaScript objects. V8 uses this information to decide when
  /// to perform global garbage collection. Each CefV8Value tracks the amount of
  /// external memory associated with it and automatically decreases the global
  /// total by the appropriate amount on its destruction. |change_in_bytes|
  /// specifies the number of bytes to adjust by. This method returns the number
  /// of bytes associated with the object after the adjustment. This method can
  /// only be called on user created objects.
  pub fn adjust_externally_allocated_memory(&mut self, change_in_bytes: i32) -> i32 {
    unsafe {
      let ret = match self.raw.as_ref().adjust_externally_allocated_memory {
        Some(f) => f(self.raw.as_ptr(),change_in_bytes,),
        None => panic!(),
      };
      ret
    }
  }
  /// ARRAY METHODS - These methods are only available on arrays.
  /// Returns the number of elements in the array.
  pub fn get_array_length(&mut self) -> i32 {
    unsafe {
      let ret = match self.raw.as_ref().get_array_length {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  /// ARRAY BUFFER METHODS - These methods are only available on ArrayBuffers.
  /// Returns the ReleaseCallback object associated with the ArrayBuffer or NULL
  /// if the ArrayBuffer was not created with CreateArrayBuffer.
  pub fn get_array_buffer_release_callback(&mut self) -> Option<crate::include::CefV8ArrayBufferReleaseCallback> {
    unsafe {
      let ret = match self.raw.as_ref().get_array_buffer_release_callback {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefV8ArrayBufferReleaseCallback::from_cef_own(ret)
    }
  }
  /// Prevent the ArrayBuffer from using it's memory block by setting the length
  /// to zero. This operation cannot be undone. If the ArrayBuffer was created
  /// with CreateArrayBuffer then CefV8ArrayBufferReleaseCallback::ReleaseBuffer
  /// will be called to release the underlying buffer.
  pub fn neuter_array_buffer(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().neuter_array_buffer {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// FUNCTION METHODS - These methods are only available on functions.
  /// Returns the function name.
  pub fn get_function_name(&mut self) -> crate::include::internal::CefStringUserFree {
    unsafe {
      let ret = match self.raw.as_ref().get_function_name {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefStringUserFree::from_cef(ret).unwrap()
    }
  }
  /// Returns the function handler or NULL if not a CEF-created function.
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
pub type CefV8StackTrace = crate::include::refcounting::CefProxy<cef_sys::cef_v8stack_trace_t>;
#[allow(non_snake_case)]
impl CefV8StackTrace {
  /// Returns the stack trace for the currently active context. |frame_limit| is
  /// the maximum number of frames that will be captured.
  #[allow(non_snake_case)]
  pub fn get_current(frame_limit: i32, ) -> Option<crate::include::CefV8StackTrace> {
    unsafe {
      let ret = cef_sys::cef_v8stack_trace_get_current(frame_limit,);
      crate::include::CefV8StackTrace::from_cef_own(ret)
    }
  }
  /// Returns true if the underlying handle is valid and it can be accessed on
  /// the current thread. Do not call any other methods if this method returns
  /// false.
  pub fn is_valid(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_valid {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns the number of stack frames.
  pub fn get_frame_count(&mut self) -> i32 {
    unsafe {
      let ret = match self.raw.as_ref().get_frame_count {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  /// Returns the stack frame at the specified 0-based index.
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
pub type CefV8StackFrame = crate::include::refcounting::CefProxy<cef_sys::cef_v8stack_frame_t>;
#[allow(non_snake_case)]
impl CefV8StackFrame {
  /// Returns true if the underlying handle is valid and it can be accessed on
  /// the current thread. Do not call any other methods if this method returns
  /// false.
  pub fn is_valid(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_valid {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns the name of the resource script that contains the function.
  pub fn get_script_name(&mut self) -> crate::include::internal::CefStringUserFree {
    unsafe {
      let ret = match self.raw.as_ref().get_script_name {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefStringUserFree::from_cef(ret).unwrap()
    }
  }
  /// Returns the name of the resource script that contains the function or the
  /// sourceURL value if the script name is undefined and its source ends with
  /// a "//@ sourceURL=..." string.
  pub fn get_script_name_or_source_url(&mut self) -> crate::include::internal::CefStringUserFree {
    unsafe {
      let ret = match self.raw.as_ref().get_script_name_or_source_url {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefStringUserFree::from_cef(ret).unwrap()
    }
  }
  /// Returns the name of the function.
  pub fn get_function_name(&mut self) -> crate::include::internal::CefStringUserFree {
    unsafe {
      let ret = match self.raw.as_ref().get_function_name {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefStringUserFree::from_cef(ret).unwrap()
    }
  }
  /// Returns the 1-based line number for the function call or 0 if unknown.
  pub fn get_line_number(&mut self) -> i32 {
    unsafe {
      let ret = match self.raw.as_ref().get_line_number {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  /// Returns the 1-based column offset on the line for the function call or 0 if
  /// unknown.
  pub fn get_column(&mut self) -> i32 {
    unsafe {
      let ret = match self.raw.as_ref().get_column {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  /// Returns true if the function was compiled using eval().
  pub fn is_eval(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_eval {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns true if the function was called as a constructor via "new".
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
/// Register a new V8 extension with the specified JavaScript extension code and
/// handler. Functions implemented by the handler are prototyped using the
/// keyword 'native'. The calling of a native function is restricted to the scope
/// in which the prototype of the native function is defined. This function may
/// only be called on the render process main thread.
/// 
/// Example JavaScript extension code:
/// <pre>
/// // create the 'example' global object if it doesn't already exist.
/// if (!example)
/// example = {};
/// // create the 'example.test' global object if it doesn't already exist.
/// if (!example.test)
/// example.test = {};
/// (function() {
/// // Define the function 'example.test.myfunction'.
/// example.test.myfunction = function() {
/// // Call CefV8Handler::Execute() with the function name 'MyFunction'
/// // and no arguments.
/// native function MyFunction();
/// return MyFunction();
/// };
/// // Define the getter function for parameter 'example.test.myparam'.
/// example.test.__defineGetter__('myparam', function() {
/// // Call CefV8Handler::Execute() with the function name 'GetMyParam'
/// // and no arguments.
/// native function GetMyParam();
/// return GetMyParam();
/// });
/// // Define the setter function for parameter 'example.test.myparam'.
/// example.test.__defineSetter__('myparam', function(b) {
/// // Call CefV8Handler::Execute() with the function name 'SetMyParam'
/// // and a single argument.
/// native function SetMyParam();
/// if(b) SetMyParam(b);
/// });
/// 
/// // Extension definitions can also contain normal JavaScript variables
/// // and functions.
/// var myint = 0;
/// example.test.increment = function() {
/// myint += 1;
/// return myint;
/// };
/// })();
/// </pre>
/// Example usage in the page:
/// <pre>
/// // Call the function.
/// example.test.myfunction();
/// // Set the parameter.
/// example.test.myparam = value;
/// // Get the parameter.
/// value = example.test.myparam;
/// // Call another function.
/// example.test.increment();
/// </pre>
#[allow(non_snake_case)]
pub fn cef_register_extension(extension_name: &crate::include::internal::CefString, javascript_code: &crate::include::internal::CefString, handler: Option<crate::include::CefV8Handler>, ) -> bool {
  unsafe {
    let ret = cef_sys::cef_register_extension(extension_name as *const _ as *const _,javascript_code as *const _ as *const _,handler.map_or(std::ptr::null_mut(), |o| crate::include::CefV8Handler::to_cef_own(o)),);
    if ret == 0 { false } else { true }
  }
}
