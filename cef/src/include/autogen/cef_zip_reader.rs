pub type CefZipReader = crate::include::refcounting::CefProxy<cef_sys::cef_zip_reader_t>;
#[allow(non_snake_case)]
impl CefZipReader {
  /// Create a new CefZipReader object. The returned object's methods can only
  /// be called from the thread that created the object.
  #[allow(non_snake_case)]
  pub fn create(stream: crate::include::CefStreamReader, ) -> Option<crate::include::CefZipReader> {
    unsafe {
      let ret = cef_sys::cef_zip_reader_create(crate::include::CefStreamReader::to_cef_own(stream),);
      crate::include::CefZipReader::from_cef_own(ret)
    }
  }
  /// Moves the cursor to the first file in the archive. Returns true if the
  /// cursor position was set successfully.
  pub fn move_to_first_file(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().move_to_first_file {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Moves the cursor to the next file in the archive. Returns true if the
  /// cursor position was set successfully.
  pub fn move_to_next_file(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().move_to_next_file {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Moves the cursor to the specified file in the archive. If |caseSensitive|
  /// is true then the search will be case sensitive. Returns true if the cursor
  /// position was set successfully.
  pub fn move_to_file(&mut self, fileName: &crate::include::internal::CefString, caseSensitive: bool) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().move_to_file {
        Some(f) => f(self.raw.as_ptr(),fileName as *const _ as *const _,if caseSensitive { 1 } else { 0 },),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Closes the archive. This should be called directly to ensure that cleanup
  /// occurs on the correct thread.
  pub fn close(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().close {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// The below methods act on the file at the current cursor position.
  /// Returns the name of the file.
  pub fn get_file_name(&mut self) -> crate::include::internal::CefStringUserFree {
    unsafe {
      let ret = match self.raw.as_ref().get_file_name {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefStringUserFree::from_cef(ret).unwrap()
    }
  }
  /// Returns the uncompressed size of the file.
  pub fn get_file_size(&mut self) -> i64 {
    unsafe {
      let ret = match self.raw.as_ref().get_file_size {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  /// Returns the last modified timestamp for the file.
  pub fn get_file_last_modified(&mut self) -> crate::include::internal::CefTime {
    unsafe {
      let ret = match self.raw.as_ref().get_file_last_modified {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret.into()
    }
  }
  /// Opens the file for reading of uncompressed data. A read password may
  /// optionally be specified.
  pub fn open_file(&mut self, password: Option<&crate::include::internal::CefString>) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().open_file {
        Some(f) => f(self.raw.as_ptr(),match password { Some(password) => password as *const _ as *const _, None => std::ptr::null_mut() },),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Closes the file.
  pub fn close_file(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().close_file {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Read uncompressed file contents into the specified buffer. Returns < 0 if
  /// an error occurred, 0 if at the end of file, or the number of bytes read.
  pub fn read_file(&mut self, buffer: &mut [u8]) -> i32 {
    unsafe {
      let ret = match self.raw.as_ref().read_file {
        Some(f) => f(self.raw.as_ptr(),buffer.as_mut_ptr() as *mut _,buffer.len() as _,),
        None => panic!(),
      };
      ret
    }
  }
  /// Returns the current offset in the uncompressed file contents.
  pub fn tell(&mut self) -> i64 {
    unsafe {
      let ret = match self.raw.as_ref().tell {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  /// Returns true if at end of the file contents.
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
