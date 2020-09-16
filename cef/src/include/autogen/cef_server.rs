pub type CefServer = crate::include::base::CefProxy<cef_sys::cef_server_t>;
#[allow(non_snake_case)]
impl CefServer {
  pub fn get_task_runner(&mut self) -> Option<crate::include::CefTaskRunner> {
    unsafe {
      let ret = match self.raw.as_ref().get_task_runner {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefTaskRunner::from_cef_own(ret)
    }
  }
  pub fn shutdown(&mut self) -> () {
    unsafe {
      let ret = match self.raw.as_ref().shutdown {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn is_running(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_running {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn get_address(&mut self) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_address {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  pub fn has_connection(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().has_connection {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn is_valid_connection(&mut self, connection_id: i32) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_valid_connection {
        Some(f) => f(self.raw.as_ptr(),connection_id,),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn send_http404response(&mut self, connection_id: i32) -> () {
    unsafe {
      let ret = match self.raw.as_ref().send_http404response {
        Some(f) => f(self.raw.as_ptr(),connection_id,),
        None => panic!(),
      };
      ret
    }
  }
  pub fn send_http500response(&mut self, connection_id: i32, error_message: &crate::include::internal::CefString) -> () {
    unsafe {
      let ret = match self.raw.as_ref().send_http500response {
        Some(f) => f(self.raw.as_ptr(),connection_id,crate::include::internal::IntoCef::into_cef(error_message),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn close_connection(&mut self, connection_id: i32) -> () {
    unsafe {
      let ret = match self.raw.as_ref().close_connection {
        Some(f) => f(self.raw.as_ptr(),connection_id,),
        None => panic!(),
      };
      ret
    }
  }
}
/// Implement this interface to handle HTTP server requests. A new thread will be
/// created for each CefServer::CreateServer call (the "dedicated server
/// thread"), and the methods of this class will be called on that thread. It is
/// therefore recommended to use a different CefServerHandler instance for each
/// CefServer::CreateServer call to avoid thread safety issues in the
/// CefServerHandler implementation.
#[allow(non_snake_case)]
pub trait ServerHandler {
  fn on_server_created(&mut self, server: crate::include::CefServer) -> () { Default::default() }
  fn on_server_destroyed(&mut self, server: crate::include::CefServer) -> () { Default::default() }
  fn on_client_connected(&mut self, server: crate::include::CefServer, connection_id: i32) -> () { Default::default() }
  fn on_client_disconnected(&mut self, server: crate::include::CefServer, connection_id: i32) -> () { Default::default() }
  fn on_http_request(&mut self, server: crate::include::CefServer, connection_id: i32, client_address: &crate::include::internal::CefString, request: crate::include::CefRequest) -> () { Default::default() }
  fn on_web_socket_request(&mut self, server: crate::include::CefServer, connection_id: i32, client_address: &crate::include::internal::CefString, request: crate::include::CefRequest, callback: crate::include::CefCallback) -> () { Default::default() }
  fn on_web_socket_connected(&mut self, server: crate::include::CefServer, connection_id: i32) -> () { Default::default() }
}
define_refcounted!(ServerHandler, server_handler, on_server_created,on_server_destroyed,on_client_connected,on_client_disconnected,on_http_request,on_web_socket_request,on_web_socket_connected,);
#[allow(non_snake_case)]
unsafe extern "C" fn cef_server_handler_t_on_server_created(_self: *mut cef_sys::cef_server_handler_t, server: *mut cef_sys::cef_server_t) -> () {
  let ret = CefServerHandler::from_cef(_self, true).get().on_server_created(crate::include::CefServer::from_cef_own(server).unwrap(),);
  ret
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_server_handler_t_on_server_destroyed(_self: *mut cef_sys::cef_server_handler_t, server: *mut cef_sys::cef_server_t) -> () {
  let ret = CefServerHandler::from_cef(_self, true).get().on_server_destroyed(crate::include::CefServer::from_cef_own(server).unwrap(),);
  ret
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_server_handler_t_on_client_connected(_self: *mut cef_sys::cef_server_handler_t, server: *mut cef_sys::cef_server_t, connection_id: i32) -> () {
  let ret = CefServerHandler::from_cef(_self, true).get().on_client_connected(crate::include::CefServer::from_cef_own(server).unwrap(),connection_id,);
  ret
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_server_handler_t_on_client_disconnected(_self: *mut cef_sys::cef_server_handler_t, server: *mut cef_sys::cef_server_t, connection_id: i32) -> () {
  let ret = CefServerHandler::from_cef(_self, true).get().on_client_disconnected(crate::include::CefServer::from_cef_own(server).unwrap(),connection_id,);
  ret
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_server_handler_t_on_http_request(_self: *mut cef_sys::cef_server_handler_t, server: *mut cef_sys::cef_server_t, connection_id: i32, client_address: *const cef_sys::cef_string_t, request: *mut cef_sys::cef_request_t) -> () {
  let ret = CefServerHandler::from_cef(_self, true).get().on_http_request(crate::include::CefServer::from_cef_own(server).unwrap(),connection_id,&crate::include::internal::CefString::from_cef(client_address).unwrap(),crate::include::CefRequest::from_cef_own(request).unwrap(),);
  ret
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_server_handler_t_on_web_socket_request(_self: *mut cef_sys::cef_server_handler_t, server: *mut cef_sys::cef_server_t, connection_id: i32, client_address: *const cef_sys::cef_string_t, request: *mut cef_sys::cef_request_t, callback: *mut cef_sys::cef_callback_t) -> () {
  let ret = CefServerHandler::from_cef(_self, true).get().on_web_socket_request(crate::include::CefServer::from_cef_own(server).unwrap(),connection_id,&crate::include::internal::CefString::from_cef(client_address).unwrap(),crate::include::CefRequest::from_cef_own(request).unwrap(),crate::include::CefCallback::from_cef_own(callback).unwrap(),);
  ret
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_server_handler_t_on_web_socket_connected(_self: *mut cef_sys::cef_server_handler_t, server: *mut cef_sys::cef_server_t, connection_id: i32) -> () {
  let ret = CefServerHandler::from_cef(_self, true).get().on_web_socket_connected(crate::include::CefServer::from_cef_own(server).unwrap(),connection_id,);
  ret
}
