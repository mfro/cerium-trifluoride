pub type CefServer = crate::include::refcounting::CefProxy<cef_sys::cef_server_t>;
#[allow(non_snake_case)]
impl CefServer {
  /// Returns the task runner for the dedicated server thread.
  pub fn get_task_runner(&mut self) -> Option<crate::include::CefTaskRunner> {
    unsafe {
      let ret = match self.raw.as_ref().get_task_runner {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefTaskRunner::from_cef_own(ret)
    }
  }
  /// Stop the server and shut down the dedicated server thread. See
  /// CefServerHandler::OnServerCreated documentation for a description of
  /// server lifespan.
  pub fn shutdown(&mut self) -> () {
    unsafe {
      let ret = match self.raw.as_ref().shutdown {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  /// Returns true if the server is currently running and accepting incoming
  /// connections. See CefServerHandler::OnServerCreated documentation for a
  /// description of server lifespan. This method must be called on the dedicated
  /// server thread.
  pub fn is_running(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_running {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns the server address including the port number.
  pub fn get_address(&mut self) -> crate::include::internal::CefStringUserFree {
    unsafe {
      let ret = match self.raw.as_ref().get_address {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefStringUserFree::from_cef(ret).unwrap()
    }
  }
  /// Returns true if the server currently has a connection. This method must be
  /// called on the dedicated server thread.
  pub fn has_connection(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().has_connection {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns true if |connection_id| represents a valid connection. This method
  /// must be called on the dedicated server thread.
  pub fn is_valid_connection(&mut self, connection_id: i32) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_valid_connection {
        Some(f) => f(self.raw.as_ptr(),connection_id,),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Send an HTTP 200 "OK" response to the connection identified by
  /// |connection_id|. |content_type| is the response content type (e.g.
  /// "text/html"), |data| is the response content, and |data_size| is the size
  /// of |data| in bytes. The contents of |data| will be copied. The connection
  /// will be closed automatically after the response is sent.
  pub fn send_http200response(&mut self, connection_id: i32, content_type: &crate::include::internal::CefString, data: &[u8]) -> () {
    unsafe {
      let ret = match self.raw.as_ref().send_http200response {
        Some(f) => f(self.raw.as_ptr(),connection_id,content_type as *const _ as *const _,data.as_ptr() as *const _,data.len() as _,),
        None => panic!(),
      };
      ret
    }
  }
  /// Send an HTTP 404 "Not Found" response to the connection identified by
  /// |connection_id|. The connection will be closed automatically after the
  /// response is sent.
  pub fn send_http404response(&mut self, connection_id: i32) -> () {
    unsafe {
      let ret = match self.raw.as_ref().send_http404response {
        Some(f) => f(self.raw.as_ptr(),connection_id,),
        None => panic!(),
      };
      ret
    }
  }
  /// Send an HTTP 500 "Internal Server Error" response to the connection
  /// identified by |connection_id|. |error_message| is the associated error
  /// message. The connection will be closed automatically after the response is
  /// sent.
  pub fn send_http500response(&mut self, connection_id: i32, error_message: &crate::include::internal::CefString) -> () {
    unsafe {
      let ret = match self.raw.as_ref().send_http500response {
        Some(f) => f(self.raw.as_ptr(),connection_id,error_message as *const _ as *const _,),
        None => panic!(),
      };
      ret
    }
  }
  /// Send raw data directly to the connection identified by |connection_id|.
  /// |data| is the raw data and |data_size| is the size of |data| in bytes.
  /// The contents of |data| will be copied. No validation of |data| is
  /// performed internally so the client should be careful to send the amount
  /// indicated by the "Content-Length" header, if specified. See
  /// SendHttpResponse documentation for intended usage.
  pub fn send_raw_data(&mut self, connection_id: &[u8], data_size: u64) -> () {
    unsafe {
      let ret = match self.raw.as_ref().send_raw_data {
        Some(f) => f(self.raw.as_ptr(),connection_id.len() as _,connection_id.as_ptr() as *const _,data_size,),
        None => panic!(),
      };
      ret
    }
  }
  /// Close the connection identified by |connection_id|. See SendHttpResponse
  /// documentation for intended usage.
  pub fn close_connection(&mut self, connection_id: i32) -> () {
    unsafe {
      let ret = match self.raw.as_ref().close_connection {
        Some(f) => f(self.raw.as_ptr(),connection_id,),
        None => panic!(),
      };
      ret
    }
  }
  /// Send a WebSocket message to the connection identified by |connection_id|.
  /// |data| is the response content and |data_size| is the size of |data| in
  /// bytes. The contents of |data| will be copied. See
  /// CefServerHandler::OnWebSocketRequest documentation for intended usage.
  pub fn send_web_socket_message(&mut self, connection_id: &[u8], data_size: u64) -> () {
    unsafe {
      let ret = match self.raw.as_ref().send_web_socket_message {
        Some(f) => f(self.raw.as_ptr(),connection_id.len() as _,connection_id.as_ptr() as *const _,data_size,),
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
#[allow(unused_variables)]
pub trait ServerHandler {
  /// Called when |server| is created. If the server was started successfully
  /// then CefServer::IsRunning will return true. The server will continue
  /// running until CefServer::Shutdown is called, after which time
  /// OnServerDestroyed will be called. If the server failed to start then
  /// OnServerDestroyed will be called immediately after this method returns.
  fn on_server_created(&mut self, server: crate::include::CefServer) -> () { Default::default() }
  /// Called when |server| is destroyed. The server thread will be stopped after
  /// this method returns. The client should release any references to |server|
  /// when this method is called. See OnServerCreated documentation for a
  /// description of server lifespan.
  fn on_server_destroyed(&mut self, server: crate::include::CefServer) -> () { Default::default() }
  /// Called when a client connects to |server|. |connection_id| uniquely
  /// identifies the connection. Each call to this method will have a matching
  /// call to OnClientDisconnected.
  fn on_client_connected(&mut self, server: crate::include::CefServer, connection_id: i32) -> () { Default::default() }
  /// Called when a client disconnects from |server|. |connection_id| uniquely
  /// identifies the connection. The client should release any data associated
  /// with |connection_id| when this method is called and |connection_id| should
  /// no longer be passed to CefServer methods. Disconnects can originate from
  /// either the client or the server. For example, the server will disconnect
  /// automatically after a CefServer::SendHttpXXXResponse method is called.
  fn on_client_disconnected(&mut self, server: crate::include::CefServer, connection_id: i32) -> () { Default::default() }
  /// Called when |server| receives an HTTP request. |connection_id| uniquely
  /// identifies the connection, |client_address| is the requesting IPv4 or IPv6
  /// client address including port number, and |request| contains the request
  /// contents (URL, method, headers and optional POST data). Call CefServer
  /// methods either synchronously or asynchronusly to send a response.
  fn on_http_request(&mut self, server: crate::include::CefServer, connection_id: i32, client_address: &crate::include::internal::CefString, request: crate::include::CefRequest) -> () { Default::default() }
  /// Called when |server| receives a WebSocket request. |connection_id| uniquely
  /// identifies the connection, |client_address| is the requesting IPv4 or
  /// IPv6 client address including port number, and |request| contains the
  /// request contents (URL, method, headers and optional POST data). Execute
  /// |callback| either synchronously or asynchronously to accept or decline the
  /// WebSocket connection. If the request is accepted then OnWebSocketConnected
  /// will be called after the WebSocket has connected and incoming messages will
  /// be delivered to the OnWebSocketMessage callback. If the request is declined
  /// then the client will be disconnected and OnClientDisconnected will be
  /// called. Call the CefServer::SendWebSocketMessage method after receiving the
  /// OnWebSocketConnected callback to respond with WebSocket messages.
  fn on_web_socket_request(&mut self, server: crate::include::CefServer, connection_id: i32, client_address: &crate::include::internal::CefString, request: crate::include::CefRequest, callback: crate::include::CefCallback) -> () { Default::default() }
  /// Called after the client has accepted the WebSocket connection for |server|
  /// and |connection_id| via the OnWebSocketRequest callback. See
  /// OnWebSocketRequest documentation for intended usage.
  fn on_web_socket_connected(&mut self, server: crate::include::CefServer, connection_id: i32) -> () { Default::default() }
  /// Called when |server| receives an WebSocket message. |connection_id|
  /// uniquely identifies the connection, |data| is the message content and
  /// |data_size| is the size of |data| in bytes. Do not keep a reference to
  /// |data| outside of this method. See OnWebSocketRequest documentation for
  /// intended usage.
  fn on_web_socket_message(&mut self, server: crate::include::CefServer, connection_id: &[u8], data_size: u64) -> () { Default::default() }
}
define_refcounted!(ServerHandler, CefServerHandler, cef_server_handler_t, on_server_created: cef_server_handler_t_on_server_created,on_server_destroyed: cef_server_handler_t_on_server_destroyed,on_client_connected: cef_server_handler_t_on_client_connected,on_client_disconnected: cef_server_handler_t_on_client_disconnected,on_http_request: cef_server_handler_t_on_http_request,on_web_socket_request: cef_server_handler_t_on_web_socket_request,on_web_socket_connected: cef_server_handler_t_on_web_socket_connected,on_web_socket_message: cef_server_handler_t_on_web_socket_message,);
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
  let ret = CefServerHandler::from_cef(_self, true).get().on_http_request(crate::include::CefServer::from_cef_own(server).unwrap(),connection_id,&*(client_address as *const _),crate::include::CefRequest::from_cef_own(request).unwrap(),);
  ret
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_server_handler_t_on_web_socket_request(_self: *mut cef_sys::cef_server_handler_t, server: *mut cef_sys::cef_server_t, connection_id: i32, client_address: *const cef_sys::cef_string_t, request: *mut cef_sys::cef_request_t, callback: *mut cef_sys::cef_callback_t) -> () {
  let ret = CefServerHandler::from_cef(_self, true).get().on_web_socket_request(crate::include::CefServer::from_cef_own(server).unwrap(),connection_id,&*(client_address as *const _),crate::include::CefRequest::from_cef_own(request).unwrap(),crate::include::CefCallback::from_cef_own(callback).unwrap(),);
  ret
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_server_handler_t_on_web_socket_connected(_self: *mut cef_sys::cef_server_handler_t, server: *mut cef_sys::cef_server_t, connection_id: i32) -> () {
  let ret = CefServerHandler::from_cef(_self, true).get().on_web_socket_connected(crate::include::CefServer::from_cef_own(server).unwrap(),connection_id,);
  ret
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_server_handler_t_on_web_socket_message(_self: *mut cef_sys::cef_server_handler_t, server: *mut cef_sys::cef_server_t, connection_id0: i32, connection_id1: *const std::os::raw::c_void, data_size: u64) -> () {
  let ret = CefServerHandler::from_cef(_self, true).get().on_web_socket_message(crate::include::CefServer::from_cef_own(server).unwrap(),std::slice::from_raw_parts(connection_id0 as *const _, connection_id1 as _),data_size,);
  ret
}
