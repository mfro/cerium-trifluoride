/// Interface the client can implement to provide a custom stream reader. The
/// methods of this class may be called on any thread.
#[allow(non_snake_case)]
pub trait ReadHandler {
  fn read(&mut self, ptr: &mut std::os::raw::c_void, size: u64, n: u64) -> u64 { Default::default() }
  fn seek(&mut self, offset: i64, whence: i32) -> i32 { Default::default() }
  fn tell(&mut self) -> i64 { Default::default() }
  fn eof(&mut self) -> i32 { Default::default() }
  fn may_block(&mut self) -> bool { Default::default() }
}
define_refcounted!(ReadHandler, read_handler, read,seek,tell,eof,may_block,);
#[allow(non_snake_case)]
unsafe extern "C" fn cef_read_handler_t_read(_self: *mut cef_sys::cef_read_handler_t, ptr: *mut std::os::raw::c_void, size: u64, n: u64) -> u64 {
  let ret = CefReadHandler::from_cef(_self, true).get().read(&mut *ptr,size,n,);
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
pub type CefStreamReader = crate::include::base::CefProxy<cef_sys::cef_stream_reader_t>;
#[allow(non_snake_case)]
impl CefStreamReader {
  pub fn read(&mut self, ptr: &mut std::os::raw::c_void, size: u64, n: u64) -> u64 {
    unsafe {
      let ret = match self.raw.as_ref().read {
        Some(f) => f(self.raw.as_ptr(),ptr,size,n,),
        None => panic!(),
      };
      ret
    }
  }
  pub fn seek(&mut self, offset: i64, whence: i32) -> i32 {
    unsafe {
      let ret = match self.raw.as_ref().seek {
        Some(f) => f(self.raw.as_ptr(),offset,whence,),
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
  pub fn eof(&mut self) -> i32 {
    unsafe {
      let ret = match self.raw.as_ref().eof {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
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
pub trait WriteHandler {
  fn seek(&mut self, offset: i64, whence: i32) -> i32 { Default::default() }
  fn tell(&mut self) -> i64 { Default::default() }
  fn flush(&mut self) -> i32 { Default::default() }
  fn may_block(&mut self) -> bool { Default::default() }
}
define_refcounted!(WriteHandler, write_handler, seek,tell,flush,may_block,);
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
pub type CefStreamWriter = crate::include::base::CefProxy<cef_sys::cef_stream_writer_t>;
#[allow(non_snake_case)]
impl CefStreamWriter {
  pub fn seek(&mut self, offset: i64, whence: i32) -> i32 {
    unsafe {
      let ret = match self.raw.as_ref().seek {
        Some(f) => f(self.raw.as_ptr(),offset,whence,),
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
  pub fn flush(&mut self) -> i32 {
    unsafe {
      let ret = match self.raw.as_ref().flush {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
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
