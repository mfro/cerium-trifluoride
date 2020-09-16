pub type CefRunContextMenuCallback = crate::include::base::CefProxy<cef_sys::cef_run_context_menu_callback_t>;
#[allow(non_snake_case)]
impl CefRunContextMenuCallback {
  pub fn cont(&mut self, command_id: i32, event_flags: crate::include::internal::CefEventFlags) -> () {
    unsafe {
      let ret = match self.raw.as_ref().cont {
        Some(f) => f(self.raw.as_ptr(),command_id,event_flags.into(),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn cancel(&mut self) -> () {
    unsafe {
      let ret = match self.raw.as_ref().cancel {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
}
/// Implement this interface to handle context menu events. The methods of this
/// class will be called on the UI thread.
#[allow(non_snake_case)]
pub trait ContextMenuHandler {
  fn on_before_context_menu(&mut self, browser: crate::include::CefBrowser, frame: crate::include::CefFrame, params: crate::include::CefContextMenuParams, model: crate::include::CefMenuModel) -> () { Default::default() }
  fn run_context_menu(&mut self, browser: crate::include::CefBrowser, frame: crate::include::CefFrame, params: crate::include::CefContextMenuParams, model: crate::include::CefMenuModel, callback: crate::include::CefRunContextMenuCallback) -> bool { Default::default() }
  fn on_context_menu_command(&mut self, browser: crate::include::CefBrowser, frame: crate::include::CefFrame, params: crate::include::CefContextMenuParams, command_id: i32, event_flags: crate::include::internal::CefEventFlags) -> bool { Default::default() }
  fn on_context_menu_dismissed(&mut self, browser: crate::include::CefBrowser, frame: crate::include::CefFrame) -> () { Default::default() }
}
define_refcounted!(ContextMenuHandler, context_menu_handler, on_before_context_menu,run_context_menu,on_context_menu_command,on_context_menu_dismissed,);
#[allow(non_snake_case)]
unsafe extern "C" fn cef_context_menu_handler_t_on_before_context_menu(_self: *mut cef_sys::cef_context_menu_handler_t, browser: *mut cef_sys::cef_browser_t, frame: *mut cef_sys::cef_frame_t, params: *mut cef_sys::cef_context_menu_params_t, model: *mut cef_sys::cef_menu_model_t) -> () {
  let ret = CefContextMenuHandler::from_cef(_self, true).get().on_before_context_menu(crate::include::CefBrowser::from_cef_own(browser).unwrap(),crate::include::CefFrame::from_cef_own(frame).unwrap(),crate::include::CefContextMenuParams::from_cef_own(params).unwrap(),crate::include::CefMenuModel::from_cef_own(model).unwrap(),);
  ret
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_context_menu_handler_t_run_context_menu(_self: *mut cef_sys::cef_context_menu_handler_t, browser: *mut cef_sys::cef_browser_t, frame: *mut cef_sys::cef_frame_t, params: *mut cef_sys::cef_context_menu_params_t, model: *mut cef_sys::cef_menu_model_t, callback: *mut cef_sys::cef_run_context_menu_callback_t) -> i32 {
  let ret = CefContextMenuHandler::from_cef(_self, true).get().run_context_menu(crate::include::CefBrowser::from_cef_own(browser).unwrap(),crate::include::CefFrame::from_cef_own(frame).unwrap(),crate::include::CefContextMenuParams::from_cef_own(params).unwrap(),crate::include::CefMenuModel::from_cef_own(model).unwrap(),crate::include::CefRunContextMenuCallback::from_cef_own(callback).unwrap(),);
  if ret { 1 } else { 0 }
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_context_menu_handler_t_on_context_menu_command(_self: *mut cef_sys::cef_context_menu_handler_t, browser: *mut cef_sys::cef_browser_t, frame: *mut cef_sys::cef_frame_t, params: *mut cef_sys::cef_context_menu_params_t, command_id: i32, event_flags: cef_sys::cef_event_flags_t) -> i32 {
  let ret = CefContextMenuHandler::from_cef(_self, true).get().on_context_menu_command(crate::include::CefBrowser::from_cef_own(browser).unwrap(),crate::include::CefFrame::from_cef_own(frame).unwrap(),crate::include::CefContextMenuParams::from_cef_own(params).unwrap(),command_id,event_flags.into(),);
  if ret { 1 } else { 0 }
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_context_menu_handler_t_on_context_menu_dismissed(_self: *mut cef_sys::cef_context_menu_handler_t, browser: *mut cef_sys::cef_browser_t, frame: *mut cef_sys::cef_frame_t) -> () {
  let ret = CefContextMenuHandler::from_cef(_self, true).get().on_context_menu_dismissed(crate::include::CefBrowser::from_cef_own(browser).unwrap(),crate::include::CefFrame::from_cef_own(frame).unwrap(),);
  ret
}
pub type CefContextMenuParams = crate::include::base::CefProxy<cef_sys::cef_context_menu_params_t>;
#[allow(non_snake_case)]
impl CefContextMenuParams {
  pub fn get_xcoord(&mut self) -> i32 {
    unsafe {
      let ret = match self.raw.as_ref().get_xcoord {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn get_ycoord(&mut self) -> i32 {
    unsafe {
      let ret = match self.raw.as_ref().get_ycoord {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn get_type_flags(&mut self) -> crate::include::internal::CefContextMenuTypeFlags {
    unsafe {
      let ret = match self.raw.as_ref().get_type_flags {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret.into()
    }
  }
  pub fn get_link_url(&mut self) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_link_url {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  pub fn get_unfiltered_link_url(&mut self) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_unfiltered_link_url {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  pub fn get_source_url(&mut self) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_source_url {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  pub fn has_image_contents(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().has_image_contents {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn get_title_text(&mut self) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_title_text {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  pub fn get_page_url(&mut self) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_page_url {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  pub fn get_frame_url(&mut self) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_frame_url {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  pub fn get_frame_charset(&mut self) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_frame_charset {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  pub fn get_media_type(&mut self) -> crate::include::internal::CefContextMenuMediaType {
    unsafe {
      let ret = match self.raw.as_ref().get_media_type {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret.into()
    }
  }
  pub fn get_media_state_flags(&mut self) -> crate::include::internal::CefContextMenuMediaStateFlags {
    unsafe {
      let ret = match self.raw.as_ref().get_media_state_flags {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret.into()
    }
  }
  pub fn get_selection_text(&mut self) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_selection_text {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  pub fn get_misspelled_word(&mut self) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_misspelled_word {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  pub fn is_editable(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_editable {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn is_spell_check_enabled(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_spell_check_enabled {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn get_edit_state_flags(&mut self) -> crate::include::internal::CefContextMenuEditStateFlags {
    unsafe {
      let ret = match self.raw.as_ref().get_edit_state_flags {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret.into()
    }
  }
  pub fn is_custom_menu(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_custom_menu {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn is_pepper_menu(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_pepper_menu {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
}
