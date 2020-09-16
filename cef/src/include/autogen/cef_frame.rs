pub type CefFrame = crate::include::base::CefProxy<cef_sys::cef_frame_t>;
#[allow(non_snake_case)]
impl CefFrame {
  pub fn is_valid(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_valid {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn undo(&mut self) -> () {
    unsafe {
      let ret = match self.raw.as_ref().undo {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn redo(&mut self) -> () {
    unsafe {
      let ret = match self.raw.as_ref().redo {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn cut(&mut self) -> () {
    unsafe {
      let ret = match self.raw.as_ref().cut {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn copy(&mut self) -> () {
    unsafe {
      let ret = match self.raw.as_ref().copy {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn paste(&mut self) -> () {
    unsafe {
      let ret = match self.raw.as_ref().paste {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn del(&mut self) -> () {
    unsafe {
      let ret = match self.raw.as_ref().del {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn select_all(&mut self) -> () {
    unsafe {
      let ret = match self.raw.as_ref().select_all {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn view_source(&mut self) -> () {
    unsafe {
      let ret = match self.raw.as_ref().view_source {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn get_source(&mut self, visitor: crate::include::CefStringVisitor) -> () {
    unsafe {
      let ret = match self.raw.as_ref().get_source {
        Some(f) => f(self.raw.as_ptr(),crate::include::CefStringVisitor::to_cef_own(visitor),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn get_text(&mut self, visitor: crate::include::CefStringVisitor) -> () {
    unsafe {
      let ret = match self.raw.as_ref().get_text {
        Some(f) => f(self.raw.as_ptr(),crate::include::CefStringVisitor::to_cef_own(visitor),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn load_request(&mut self, request: crate::include::CefRequest) -> () {
    unsafe {
      let ret = match self.raw.as_ref().load_request {
        Some(f) => f(self.raw.as_ptr(),crate::include::CefRequest::to_cef_own(request),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn load_url(&mut self, url: &crate::include::internal::CefString) -> () {
    unsafe {
      let ret = match self.raw.as_ref().load_url {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(url),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn execute_java_script(&mut self, code: &crate::include::internal::CefString, script_url: Option<&crate::include::internal::CefString>, start_line: i32) -> () {
    unsafe {
      let ret = match self.raw.as_ref().execute_java_script {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(code),crate::include::internal::IntoCef::into_cef(script_url),start_line,),
        None => panic!(),
      };
      ret
    }
  }
  pub fn is_main(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_main {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn is_focused(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_focused {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn get_name(&mut self) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_name {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  pub fn get_identifier(&mut self) -> i64 {
    unsafe {
      let ret = match self.raw.as_ref().get_identifier {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn get_parent(&mut self) -> Option<crate::include::CefFrame> {
    unsafe {
      let ret = match self.raw.as_ref().get_parent {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefFrame::from_cef_own(ret)
    }
  }
  pub fn get_url(&mut self) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_url {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  pub fn get_browser(&mut self) -> Option<crate::include::CefBrowser> {
    unsafe {
      let ret = match self.raw.as_ref().get_browser {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefBrowser::from_cef_own(ret)
    }
  }
  pub fn get_v8context(&mut self) -> Option<crate::include::CefV8Context> {
    unsafe {
      let ret = match self.raw.as_ref().get_v8context {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefV8Context::from_cef_own(ret)
    }
  }
  pub fn visit_dom(&mut self, visitor: crate::include::CefDOMVisitor) -> () {
    unsafe {
      let ret = match self.raw.as_ref().visit_dom {
        Some(f) => f(self.raw.as_ptr(),crate::include::CefDOMVisitor::to_cef_own(visitor),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn create_urlrequest(&mut self, request: crate::include::CefRequest, client: crate::include::CefURLRequestClient) -> Option<crate::include::CefURLRequest> {
    unsafe {
      let ret = match self.raw.as_ref().create_urlrequest {
        Some(f) => f(self.raw.as_ptr(),crate::include::CefRequest::to_cef_own(request),crate::include::CefURLRequestClient::to_cef_own(client),),
        None => panic!(),
      };
      crate::include::CefURLRequest::from_cef_own(ret)
    }
  }
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
