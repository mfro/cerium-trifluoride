/// Interface the client can implement to provide a custom stream reader. The
/// methods of this class may be called on any thread.
#[allow(non_snake_case)]
#[allow(unused_variables)]
pub trait ReadHandler {
  /// Read raw binary data.
  fn read(&mut self, ptr: &mut [u8], n: u64) -> u64 { Default::default() }
  /// Seek to the specified offset position. |whence| may be any one of
  /// SEEK_CUR, SEEK_END or SEEK_SET. Return zero on success and non-zero on
  /// failure.
  fn seek(&mut self, offset: i64, whence: i32) -> i32 { Default::default() }
  /// Return the current offset position.
  fn tell(&mut self) -> i64 { Default::default() }
  /// Return non-zero if at end of file.
  fn eof(&mut self) -> i32 { Default::default() }
  /// Return true if this handler performs work like accessing the file system
  /// which may block. Used as a hint for determining the thread to access the
  /// handler from.
  fn may_block(&mut self) -> bool { Default::default() }
}
define_refcounted!(ReadHandler, CefReadHandler, cef_read_handler_t, read: cef_read_handler_t_read,seek: cef_read_handler_t_seek,tell: cef_read_handler_t_tell,eof: cef_read_handler_t_eof,may_block: cef_read_handler_t_may_block,);
#[allow(non_snake_case)]
unsafe extern "C" fn cef_read_handler_t_read(_self: *mut cef_sys::cef_read_handler_t, ptr0: *mut std::os::raw::c_void, ptr1: u64, n: u64) -> u64 {
  let ret = CefReadHandler::from_cef(_self, true).get().read(std::slice::from_raw_parts_mut(ptr0 as *mut _, ptr1 as _),n,);
  ret
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_read_handler_t_seek(_self: *mut cef_sys::cef_read_handler_t, offset: i64, whence: i32) -> i32 {
  let ret = CefReadHandler::from_cef(_self, true).get().seek(offset,whence,);
  ret
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_read_handler_t_tell(_self: *mut cef_sys::cef_read_handler_t) -> i64 {
  let ret = CefReadHandler::from_cef(_self, true).get().tell();
  ret
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_read_handler_t_eof(_self: *mut cef_sys::cef_read_handler_t) -> i32 {
  let ret = CefReadHandler::from_cef(_self, true).get().eof();
  ret
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_read_handler_t_may_block(_self: *mut cef_sys::cef_read_handler_t) -> i32 {
  let ret = CefReadHandler::from_cef(_self, true).get().may_block();
  if ret { 1 } else { 0 }
}
pub type CefStreamReader = crate::include::refcounting::CefProxy<cef_sys::cef_stream_reader_t>;
#[allow(non_snake_case)]
impl CefStreamReader {
  /// Create a new CefStreamReader object from a file.
  #[allow(non_snake_case)]
  pub fn create_for_file(fileName: &crate::include::internal::CefString, ) -> Option<crate::include::CefStreamReader> {
    unsafe {
      let ret = cef_sys::cef_stream_reader_create_for_file(fileName as *const _ as *const _,);
      crate::include::CefStreamReader::from_cef_own(ret)
    }
  }
  /// Create a new CefStreamReader object from data.
  #[allow(non_snake_case)]
  pub fn create_for_data(data: &mut [u8], ) -> Option<crate::include::CefStreamReader> {
    unsafe {
      let ret = cef_sys::cef_stream_reader_create_for_data(data.as_mut_ptr() as *mut _,data.len() as _,);
      crate::include::CefStreamReader::from_cef_own(ret)
    }
  }
  /// Create a new CefStreamReader object from a custom handler.
  #[allow(non_snake_case)]
  pub fn create_for_handler(handler: crate::include::CefReadHandler, ) -> Option<crate::include::CefStreamReader> {
    unsafe {
      let ret = cef_sys::cef_stream_reader_create_for_handler(crate::include::CefReadHandler::to_cef_own(handler),);
      crate::include::CefStreamReader::from_cef_own(ret)
    }
  }
  /// Read raw binary data.
  pub fn read(&mut self, ptr: &mut [u8], n: u64) -> u64 {
    unsafe {
      let ret = match self.raw.as_ref().read {
        Some(f) => f(self.raw.as_ptr(),ptr.as_mut_ptr() as *mut _,ptr.len() as _,n,),
        None => panic!(),
      };
      ret
    }
  }
  /// Seek to the specified offset position. |whence| may be any one of
  /// SEEK_CUR, SEEK_END or SEEK_SET. Returns zero on success and non-zero on
  /// failure.
  pub fn seek(&mut self, offset: i64, whence: i32) -> i32 {
    unsafe {
      let ret = match self.raw.as_ref().seek {
        Some(f) => f(self.raw.as_ptr(),offset,whence,),
        None => panic!(),
      };
      ret
    }
  }
  /// Return the current offset position.
  pub fn tell(&mut self) -> i64 {
    unsafe {
      let ret = match self.raw.as_ref().tell {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  /// Return non-zero if at end of file.
  pub fn eof(&mut self) -> i32 {
    unsafe {
      let ret = match self.raw.as_ref().eof {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  /// Returns true if this reader performs work like accessing the file system
  /// which may block. Used as a hint for determining the thread to access the
  /// reader from.
  pub fn may_block(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().may_block {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
}
/// Interface the client can implement to provide a custom stream writer. The
/// methods of this class may be called on any thread.
#[allow(non_snake_case)]
#[allow(unused_variables)]
pub trait WriteHandler {
  /// Write raw binary data.
  fn write(&mut self, ptr: &[u8], n: u64) -> u64 { Default::default() }
  /// Seek to the specified offset position. |whence| may be any one of
  /// SEEK_CUR, SEEK_END or SEEK_SET. Return zero on success and non-zero on
  /// failure.
  fn seek(&mut self, offset: i64, whence: i32) -> i32 { Default::default() }
  /// Return the current offset position.
  fn tell(&mut self) -> i64 { Default::default() }
  /// Flush the stream.
  fn flush(&mut self) -> i32 { Default::default() }
  /// Return true if this handler performs work like accessing the file system
  /// which may block. Used as a hint for determining the thread to access the
  /// handler from.
  fn may_block(&mut self) -> bool { Default::default() }
}
define_refcounted!(WriteHandler, CefWriteHandler, cef_write_handler_t, write: cef_write_handler_t_write,seek: cef_write_handler_t_seek,tell: cef_write_handler_t_tell,flush: cef_write_handler_t_flush,may_block: cef_write_handler_t_may_block,);
#[allow(non_snake_case)]
unsafe extern "C" fn cef_write_handler_t_write(_self: *mut cef_sys::cef_write_handler_t, ptr0: *const std::os::raw::c_void, ptr1: u64, n: u64) -> u64 {
  let ret = CefWriteHandler::from_cef(_self, true).get().write(std::slice::from_raw_parts(ptr0 as *const _, ptr1 as _),n,);
  ret
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_write_handler_t_seek(_self: *mut cef_sys::cef_write_handler_t, offset: i64, whence: i32) -> i32 {
  let ret = CefWriteHandler::from_cef(_self, true).get().seek(offset,whence,);
  ret
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_write_handler_t_tell(_self: *mut cef_sys::cef_write_handler_t) -> i64 {
  let ret = CefWriteHandler::from_cef(_self, true).get().tell();
  ret
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_write_handler_t_flush(_self: *mut cef_sys::cef_write_handler_t) -> i32 {
  let ret = CefWriteHandler::from_cef(_self, true).get().flush();
  ret
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_write_handler_t_may_block(_self: *mut cef_sys::cef_write_handler_t) -> i32 {
  let ret = CefWriteHandler::from_cef(_self, true).get().may_block();
  if ret { 1 } else { 0 }
}
pub type CefStreamWriter = crate::include::refcounting::CefProxy<cef_sys::cef_stream_writer_t>;
#[allow(non_snake_case)]
impl CefStreamWriter {
  /// Create a new CefStreamWriter object for a file.
  #[allow(non_snake_case)]
  pub fn create_for_file(fileName: &crate::include::internal::CefString, ) -> Option<crate::include::CefStreamWriter> {
    unsafe {
      let ret = cef_sys::cef_stream_writer_create_for_file(fileName as *const _ as *const _,);
      crate::include::CefStreamWriter::from_cef_own(ret)
    }
  }
  /// Create a new CefStreamWriter object for a custom handler.
  #[allow(non_snake_case)]
  pub fn create_for_handler(handler: crate::include::CefWriteHandler, ) -> Option<crate::include::CefStreamWriter> {
    unsafe {
      let ret = cef_sys::cef_stream_writer_create_for_handler(crate::include::CefWriteHandler::to_cef_own(handler),);
      crate::include::CefStreamWriter::from_cef_own(ret)
    }
  }
  /// Write raw binary data.
  pub fn write(&mut self, ptr: &[u8], n: u64) -> u64 {
    unsafe {
      let ret = match self.raw.as_ref().write {
        Some(f) => f(self.raw.as_ptr(),ptr.as_ptr() as *const _,ptr.len() as _,n,),
        None => panic!(),
      };
      ret
    }
  }
  /// Seek to the specified offset position. |whence| may be any one of
  /// SEEK_CUR, SEEK_END or SEEK_SET. Returns zero on success and non-zero on
  /// failure.
  pub fn seek(&mut self, offset: i64, whence: i32) -> i32 {
    unsafe {
      let ret = match self.raw.as_ref().seek {
        Some(f) => f(self.raw.as_ptr(),offset,whence,),
        None => panic!(),
      };
      ret
    }
  }
  /// Return the current offset position.
  pub fn tell(&mut self) -> i64 {
    unsafe {
      let ret = match self.raw.as_ref().tell {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  /// Flush the stream.
  pub fn flush(&mut self) -> i32 {
    unsafe {
      let ret = match self.raw.as_ref().flush {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  /// Returns true if this writer performs work like accessing the file system
  /// which may block. Used as a hint for determining the thread to access the
  /// writer from.
  pub fn may_block(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().may_block {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
}
