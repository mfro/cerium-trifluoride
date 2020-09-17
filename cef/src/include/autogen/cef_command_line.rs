pub type CefCommandLine = crate::include::refcounting::CefProxy<cef_sys::cef_command_line_t>;
#[allow(non_snake_case)]
impl CefCommandLine {
  /// Create a new CefCommandLine instance.
  #[allow(non_snake_case)]
  pub fn create_command_line() -> Option<crate::include::CefCommandLine> {
    unsafe {
      let ret = cef_sys::cef_command_line_create();
      crate::include::CefCommandLine::from_cef_own(ret)
    }
  }
  /// Returns the singleton global CefCommandLine object. The returned object
  /// will be read-only.
  #[allow(non_snake_case)]
  pub fn get_global_command_line() -> Option<crate::include::CefCommandLine> {
    unsafe {
      let ret = cef_sys::cef_command_line_get_global();
      crate::include::CefCommandLine::from_cef_own(ret)
    }
  }
  /// Returns true if this object is valid. Do not call any other methods if this
  /// function returns false.
  pub fn is_valid(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_valid {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns true if the values of this object are read-only. Some APIs may
  /// expose read-only objects.
  pub fn is_read_only(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_read_only {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns a writable copy of this object.
  pub fn copy(&mut self) -> Option<crate::include::CefCommandLine> {
    unsafe {
      let ret = match self.raw.as_ref().copy {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefCommandLine::from_cef_own(ret)
    }
  }
  /// Initialize the command line with the string returned by calling
  /// GetCommandLineW(). This method is only supported on Windows.
  pub fn init_from_string(&mut self, command_line: &crate::include::internal::CefString) -> () {
    unsafe {
      let ret = match self.raw.as_ref().init_from_string {
        Some(f) => f(self.raw.as_ptr(),command_line as *const _ as *const _,),
        None => panic!(),
      };
      ret
    }
  }
  /// Reset the command-line switches and arguments but leave the program
  /// component unchanged.
  pub fn reset(&mut self) -> () {
    unsafe {
      let ret = match self.raw.as_ref().reset {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  /// Constructs and returns the represented command line string. Use this method
  /// cautiously because quoting behavior is unclear.
  pub fn get_command_line_string(&mut self) -> crate::include::internal::CefStringUserFree {
    unsafe {
      let ret = match self.raw.as_ref().get_command_line_string {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefStringUserFree::from_cef(ret).unwrap()
    }
  }
  /// Get the program part of the command line string (the first item).
  pub fn get_program(&mut self) -> crate::include::internal::CefStringUserFree {
    unsafe {
      let ret = match self.raw.as_ref().get_program {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefStringUserFree::from_cef(ret).unwrap()
    }
  }
  /// Set the program part of the command line string (the first item).
  pub fn set_program(&mut self, program: &crate::include::internal::CefString) -> () {
    unsafe {
      let ret = match self.raw.as_ref().set_program {
        Some(f) => f(self.raw.as_ptr(),program as *const _ as *const _,),
        None => panic!(),
      };
      ret
    }
  }
  /// Returns true if the command line has switches.
  pub fn has_switches(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().has_switches {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns true if the command line contains the given switch.
  pub fn has_switch(&mut self, name: &crate::include::internal::CefString) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().has_switch {
        Some(f) => f(self.raw.as_ptr(),name as *const _ as *const _,),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns the value associated with the given switch. If the switch has no
  /// value or isn't present this method returns the empty string.
  pub fn get_switch_value(&mut self, name: &crate::include::internal::CefString) -> crate::include::internal::CefStringUserFree {
    unsafe {
      let ret = match self.raw.as_ref().get_switch_value {
        Some(f) => f(self.raw.as_ptr(),name as *const _ as *const _,),
        None => panic!(),
      };
      crate::include::internal::CefStringUserFree::from_cef(ret).unwrap()
    }
  }
  /// Add a switch to the end of the command line. If the switch has no value
  /// pass an empty value string.
  pub fn append_switch(&mut self, name: &crate::include::internal::CefString) -> () {
    unsafe {
      let ret = match self.raw.as_ref().append_switch {
        Some(f) => f(self.raw.as_ptr(),name as *const _ as *const _,),
        None => panic!(),
      };
      ret
    }
  }
  /// Add a switch with the specified value to the end of the command line.
  pub fn append_switch_with_value(&mut self, name: &crate::include::internal::CefString, value: &crate::include::internal::CefString) -> () {
    unsafe {
      let ret = match self.raw.as_ref().append_switch_with_value {
        Some(f) => f(self.raw.as_ptr(),name as *const _ as *const _,value as *const _ as *const _,),
        None => panic!(),
      };
      ret
    }
  }
  /// True if there are remaining command line arguments.
  pub fn has_arguments(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().has_arguments {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Add an argument to the end of the command line.
  pub fn append_argument(&mut self, argument: &crate::include::internal::CefString) -> () {
    unsafe {
      let ret = match self.raw.as_ref().append_argument {
        Some(f) => f(self.raw.as_ptr(),argument as *const _ as *const _,),
        None => panic!(),
      };
      ret
    }
  }
  /// Insert a command before the current command.
  /// Common for debuggers, like "valgrind" or "gdb --args".
  pub fn prepend_wrapper(&mut self, wrapper: &crate::include::internal::CefString) -> () {
    unsafe {
      let ret = match self.raw.as_ref().prepend_wrapper {
        Some(f) => f(self.raw.as_ptr(),wrapper as *const _ as *const _,),
        None => panic!(),
      };
      ret
    }
  }
}
