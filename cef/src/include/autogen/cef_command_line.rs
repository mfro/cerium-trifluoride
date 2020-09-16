pub type CefCommandLine = crate::include::base::CefProxy<cef_sys::cef_command_line_t>;
#[allow(non_snake_case)]
impl CefCommandLine {
  pub fn is_valid(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_valid {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn is_read_only(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_read_only {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn copy(&mut self) -> Option<crate::include::CefCommandLine> {
    unsafe {
      let ret = match self.raw.as_ref().copy {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefCommandLine::from_cef_own(ret)
    }
  }
  pub fn init_from_string(&mut self, command_line: &crate::include::internal::CefString) -> () {
    unsafe {
      let ret = match self.raw.as_ref().init_from_string {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(command_line),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn reset(&mut self) -> () {
    unsafe {
      let ret = match self.raw.as_ref().reset {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn get_command_line_string(&mut self) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_command_line_string {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  pub fn get_program(&mut self) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_program {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  pub fn set_program(&mut self, program: &crate::include::internal::CefString) -> () {
    unsafe {
      let ret = match self.raw.as_ref().set_program {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(program),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn has_switches(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().has_switches {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn has_switch(&mut self, name: &crate::include::internal::CefString) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().has_switch {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(name),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn get_switch_value(&mut self, name: &crate::include::internal::CefString) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_switch_value {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(name),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  pub fn append_switch(&mut self, name: &crate::include::internal::CefString) -> () {
    unsafe {
      let ret = match self.raw.as_ref().append_switch {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(name),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn append_switch_with_value(&mut self, name: &crate::include::internal::CefString, value: &crate::include::internal::CefString) -> () {
    unsafe {
      let ret = match self.raw.as_ref().append_switch_with_value {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(name),crate::include::internal::IntoCef::into_cef(value),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn has_arguments(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().has_arguments {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn append_argument(&mut self, argument: &crate::include::internal::CefString) -> () {
    unsafe {
      let ret = match self.raw.as_ref().append_argument {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(argument),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn prepend_wrapper(&mut self, wrapper: &crate::include::internal::CefString) -> () {
    unsafe {
      let ret = match self.raw.as_ref().prepend_wrapper {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(wrapper),),
        None => panic!(),
      };
      ret
    }
  }
}
