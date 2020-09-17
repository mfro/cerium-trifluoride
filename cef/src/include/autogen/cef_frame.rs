pub type CefFrame = crate::include::base::CefProxy<cef_sys::cef_frame_t>;
#[allow(non_snake_case)]
impl CefFrame {
  /// True if this object is currently attached to a valid frame.
  pub fn is_valid(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_valid {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Execute undo in this frame.
  pub fn undo(&mut self) -> () {
    unsafe {
      let ret = match self.raw.as_ref().undo {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  /// Execute redo in this frame.
  pub fn redo(&mut self) -> () {
    unsafe {
      let ret = match self.raw.as_ref().redo {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  /// Execute cut in this frame.
  pub fn cut(&mut self) -> () {
    unsafe {
      let ret = match self.raw.as_ref().cut {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  /// Execute copy in this frame.
  pub fn copy(&mut self) -> () {
    unsafe {
      let ret = match self.raw.as_ref().copy {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  /// Execute paste in this frame.
  pub fn paste(&mut self) -> () {
    unsafe {
      let ret = match self.raw.as_ref().paste {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  /// Execute delete in this frame.
  pub fn del(&mut self) -> () {
    unsafe {
      let ret = match self.raw.as_ref().del {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  /// Execute select all in this frame.
  pub fn select_all(&mut self) -> () {
    unsafe {
      let ret = match self.raw.as_ref().select_all {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  /// Save this frame's HTML source to a temporary file and open it in the
  /// default text viewing application. This method can only be called from the
  /// browser process.
  pub fn view_source(&mut self) -> () {
    unsafe {
      let ret = match self.raw.as_ref().view_source {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  /// Retrieve this frame's HTML source as a string sent to the specified
  /// visitor.
  pub fn get_source(&mut self, visitor: crate::include::CefStringVisitor) -> () {
    unsafe {
      let ret = match self.raw.as_ref().get_source {
        Some(f) => f(self.raw.as_ptr(),crate::include::CefStringVisitor::to_cef_own(visitor),),
        None => panic!(),
      };
      ret
    }
  }
  /// Retrieve this frame's display text as a string sent to the specified
  /// visitor.
  pub fn get_text(&mut self, visitor: crate::include::CefStringVisitor) -> () {
    unsafe {
      let ret = match self.raw.as_ref().get_text {
        Some(f) => f(self.raw.as_ptr(),crate::include::CefStringVisitor::to_cef_own(visitor),),
        None => panic!(),
      };
      ret
    }
  }
  /// Load the request represented by the |request| object.
  /// 
  /// WARNING: This method will fail with "bad IPC message" reason
  /// INVALID_INITIATOR_ORIGIN (213) unless you first navigate to the
  /// request origin using some other mechanism (LoadURL, link click, etc).
  pub fn load_request(&mut self, request: crate::include::CefRequest) -> () {
    unsafe {
      let ret = match self.raw.as_ref().load_request {
        Some(f) => f(self.raw.as_ptr(),crate::include::CefRequest::to_cef_own(request),),
        None => panic!(),
      };
      ret
    }
  }
  /// Load the specified |url|.
  pub fn load_url(&mut self, url: &crate::include::internal::CefString) -> () {
    unsafe {
      let ret = match self.raw.as_ref().load_url {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(url),),
        None => panic!(),
      };
      ret
    }
  }
  /// Execute a string of JavaScript code in this frame. The |script_url|
  /// parameter is the URL where the script in question can be found, if any.
  /// The renderer may request this URL to show the developer the source of the
  /// error.  The |start_line| parameter is the base line number to use for error
  /// reporting.
  pub fn execute_java_script(&mut self, code: &crate::include::internal::CefString, script_url: Option<&crate::include::internal::CefString>, start_line: i32) -> () {
    unsafe {
      let ret = match self.raw.as_ref().execute_java_script {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(code),crate::include::internal::IntoCef::into_cef(script_url),start_line,),
        None => panic!(),
      };
      ret
    }
  }
  /// Returns true if this is the main (top-level) frame.
  pub fn is_main(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_main {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns true if this is the focused frame.
  pub fn is_focused(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_focused {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns the name for this frame. If the frame has an assigned name (for
  /// example, set via the iframe "name" attribute) then that value will be
  /// returned. Otherwise a unique name will be constructed based on the frame
  /// parent hierarchy. The main (top-level) frame will always have an empty name
  /// value.
  pub fn get_name(&mut self) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_name {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  /// Returns the globally unique identifier for this frame or < 0 if the
  /// underlying frame does not yet exist.
  pub fn get_identifier(&mut self) -> i64 {
    unsafe {
      let ret = match self.raw.as_ref().get_identifier {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  /// Returns the parent of this frame or NULL if this is the main (top-level)
  /// frame.
  pub fn get_parent(&mut self) -> Option<crate::include::CefFrame> {
    unsafe {
      let ret = match self.raw.as_ref().get_parent {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefFrame::from_cef_own(ret)
    }
  }
  /// Returns the URL currently loaded in this frame.
  pub fn get_url(&mut self) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_url {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  /// Returns the browser that this frame belongs to.
  pub fn get_browser(&mut self) -> Option<crate::include::CefBrowser> {
    unsafe {
      let ret = match self.raw.as_ref().get_browser {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefBrowser::from_cef_own(ret)
    }
  }
  /// Get the V8 context associated with the frame. This method can only be
  /// called from the render process.
  pub fn get_v8context(&mut self) -> Option<crate::include::CefV8Context> {
    unsafe {
      let ret = match self.raw.as_ref().get_v8context {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefV8Context::from_cef_own(ret)
    }
  }
  /// Visit the DOM document. This method can only be called from the render
  /// process.
  pub fn visit_dom(&mut self, visitor: crate::include::CefDOMVisitor) -> () {
    unsafe {
      let ret = match self.raw.as_ref().visit_dom {
        Some(f) => f(self.raw.as_ptr(),crate::include::CefDOMVisitor::to_cef_own(visitor),),
        None => panic!(),
      };
      ret
    }
  }
  /// Create a new URL request that will be treated as originating from this
  /// frame and the associated browser. This request may be intercepted by the
  /// client via CefResourceRequestHandler or CefSchemeHandlerFactory. Use
  /// CefURLRequest::Create instead if you do not want the request to have this
  /// association, in which case it may be handled differently (see documentation
  /// on that method). Requests may originate from both the browser process and
  /// the render process.
  /// 
  /// For requests originating from the browser process:
  /// - POST data may only contain a single element of type PDE_TYPE_FILE or
  /// PDE_TYPE_BYTES.
  /// For requests originating from the render process:
  /// - POST data may only contain a single element of type PDE_TYPE_BYTES.
  /// - If the response contains Content-Disposition or Mime-Type header values
  /// that would not normally be rendered then the response may receive
  /// special handling inside the browser (for example, via the file download
  /// code path instead of the URL request code path).
  /// 
  /// The |request| object will be marked as read-only after calling this method.
  pub fn create_urlrequest(&mut self, request: crate::include::CefRequest, client: crate::include::CefURLRequestClient) -> Option<crate::include::CefURLRequest> {
    unsafe {
      let ret = match self.raw.as_ref().create_urlrequest {
        Some(f) => f(self.raw.as_ptr(),crate::include::CefRequest::to_cef_own(request),crate::include::CefURLRequestClient::to_cef_own(client),),
        None => panic!(),
      };
      crate::include::CefURLRequest::from_cef_own(ret)
    }
  }
  /// Send a message to the specified |target_process|. Message delivery is not
  /// guaranteed in all cases (for example, if the browser is closing,
  /// navigating, or if the target process crashes). Send an ACK message back
  /// from the target process if confirmation is required.
  pub fn send_process_message(&mut self, target_process: crate::include::internal::CefProcessId, message: crate::include::CefProcessMessage) -> () {
    unsafe {
      let ret = match self.raw.as_ref().send_process_message {
        Some(f) => f(self.raw.as_ptr(),target_process.into(),crate::include::CefProcessMessage::to_cef_own(message),),
        None => panic!(),
      };
      ret
    }
  }
}
