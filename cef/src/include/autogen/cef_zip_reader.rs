pub type CefZipReader = crate::include::base::CefProxy<cef_sys::cef_zip_reader_t>;
#[allow(non_snake_case)]
impl CefZipReader {
  pub fn move_to_first_file(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().move_to_first_file {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn move_to_next_file(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().move_to_next_file {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn move_to_file(&mut self, fileName: &crate::include::internal::CefString, caseSensitive: bool) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().move_to_file {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(fileName),if caseSensitive { 1 } else { 0 },),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn close(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().close {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn get_file_name(&mut self) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_file_name {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  pub fn get_file_size(&mut self) -> i64 {
    unsafe {
      let ret = match self.raw.as_ref().get_file_size {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn get_file_last_modified(&mut self) -> crate::include::internal::CefTime {
    unsafe {
      let ret = match self.raw.as_ref().get_file_last_modified {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret.into()
    }
  }
  pub fn open_file(&mut self, password: Option<&crate::include::internal::CefString>) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().open_file {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(password),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn close_file(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().close_file {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn read_file(&mut self, buffer: &mut std::os::raw::c_void, bufferSize: u64) -> i32 {
    unsafe {
      let ret = match self.raw.as_ref().read_file {
        Some(f) => f(self.raw.as_ptr(),buffer,bufferSize,),
        None => panic!(),
      };
      ret
    }
  }
  pub fn tell(&mut self) -> i64 {
    unsafe {
      let ret = match self.raw.as_ref().tell {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn eof(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().eof {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
}
