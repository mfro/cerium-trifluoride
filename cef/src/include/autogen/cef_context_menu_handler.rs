pub type CefRunContextMenuCallback = crate::include::refcounting::CefProxy<cef_sys::cef_run_context_menu_callback_t>;
#[allow(non_snake_case)]
impl CefRunContextMenuCallback {
  /// Complete context menu display by selecting the specified |command_id| and
  /// |event_flags|.
  pub fn cont(&mut self, command_id: i32, event_flags: crate::include::internal::CefEventFlags) -> () {
    unsafe {
      let ret = match self.raw.as_ref().cont {
        Some(f) => f(self.raw.as_ptr(),command_id,event_flags.into(),),
        None => panic!(),
      };
      ret
    }
  }
  /// Cancel context menu display.
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
#[allow(unused_variables)]
pub trait ContextMenuHandler {
  /// Called before a context menu is displayed. |params| provides information
  /// about the context menu state. |model| initially contains the default
  /// context menu. The |model| can be cleared to show no context menu or
  /// modified to show a custom menu. Do not keep references to |params| or
  /// |model| outside of this callback.
  fn on_before_context_menu(&mut self, browser: crate::include::CefBrowser, frame: crate::include::CefFrame, params: crate::include::CefContextMenuParams, model: crate::include::CefMenuModel) -> () { Default::default() }
  /// Called to allow custom display of the context menu. |params| provides
  /// information about the context menu state. |model| contains the context menu
  /// model resulting from OnBeforeContextMenu. For custom display return true
  /// and execute |callback| either synchronously or asynchronously with the
  /// selected command ID. For default display return false. Do not keep
  /// references to |params| or |model| outside of this callback.
  fn run_context_menu(&mut self, browser: crate::include::CefBrowser, frame: crate::include::CefFrame, params: crate::include::CefContextMenuParams, model: crate::include::CefMenuModel, callback: crate::include::CefRunContextMenuCallback) -> bool { Default::default() }
  /// Called to execute a command selected from the context menu. Return true if
  /// the command was handled or false for the default implementation. See
  /// cef_menu_id_t for the command ids that have default implementations. All
  /// user-defined command ids should be between MENU_ID_USER_FIRST and
  /// MENU_ID_USER_LAST. |params| will have the same values as what was passed to
  /// OnBeforeContextMenu(). Do not keep a reference to |params| outside of this
  /// callback.
  fn on_context_menu_command(&mut self, browser: crate::include::CefBrowser, frame: crate::include::CefFrame, params: crate::include::CefContextMenuParams, command_id: i32, event_flags: crate::include::internal::CefEventFlags) -> bool { Default::default() }
  /// Called when the context menu is dismissed irregardless of whether the menu
  /// was empty or a command was selected.
  fn on_context_menu_dismissed(&mut self, browser: crate::include::CefBrowser, frame: crate::include::CefFrame) -> () { Default::default() }
}
define_refcounted!(ContextMenuHandler, CefContextMenuHandler, cef_context_menu_handler_t, on_before_context_menu: cef_context_menu_handler_t_on_before_context_menu,run_context_menu: cef_context_menu_handler_t_run_context_menu,on_context_menu_command: cef_context_menu_handler_t_on_context_menu_command,on_context_menu_dismissed: cef_context_menu_handler_t_on_context_menu_dismissed,);
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
pub type CefContextMenuParams = crate::include::refcounting::CefProxy<cef_sys::cef_context_menu_params_t>;
#[allow(non_snake_case)]
impl CefContextMenuParams {
  /// Returns the X coordinate of the mouse where the context menu was invoked.
  /// Coords are relative to the associated RenderView's origin.
  pub fn get_xcoord(&mut self) -> i32 {
    unsafe {
      let ret = match self.raw.as_ref().get_xcoord {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  /// Returns the Y coordinate of the mouse where the context menu was invoked.
  /// Coords are relative to the associated RenderView's origin.
  pub fn get_ycoord(&mut self) -> i32 {
    unsafe {
      let ret = match self.raw.as_ref().get_ycoord {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  /// Returns flags representing the type of node that the context menu was
  /// invoked on.
  pub fn get_type_flags(&mut self) -> crate::include::internal::CefContextMenuTypeFlags {
    unsafe {
      let ret = match self.raw.as_ref().get_type_flags {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret.into()
    }
  }
  /// Returns the URL of the link, if any, that encloses the node that the
  /// context menu was invoked on.
  pub fn get_link_url(&mut self) -> crate::include::internal::CefStringUserFree {
    unsafe {
      let ret = match self.raw.as_ref().get_link_url {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefStringUserFree::from_cef(ret).unwrap()
    }
  }
  /// Returns the link URL, if any, to be used ONLY for "copy link address". We
  /// don't validate this field in the frontend process.
  pub fn get_unfiltered_link_url(&mut self) -> crate::include::internal::CefStringUserFree {
    unsafe {
      let ret = match self.raw.as_ref().get_unfiltered_link_url {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefStringUserFree::from_cef(ret).unwrap()
    }
  }
  /// Returns the source URL, if any, for the element that the context menu was
  /// invoked on. Example of elements with source URLs are img, audio, and video.
  pub fn get_source_url(&mut self) -> crate::include::internal::CefStringUserFree {
    unsafe {
      let ret = match self.raw.as_ref().get_source_url {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefStringUserFree::from_cef(ret).unwrap()
    }
  }
  /// Returns true if the context menu was invoked on an image which has
  /// non-empty contents.
  pub fn has_image_contents(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().has_image_contents {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns the title text or the alt text if the context menu was invoked on
  /// an image.
  pub fn get_title_text(&mut self) -> crate::include::internal::CefStringUserFree {
    unsafe {
      let ret = match self.raw.as_ref().get_title_text {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefStringUserFree::from_cef(ret).unwrap()
    }
  }
  /// Returns the URL of the top level page that the context menu was invoked on.
  pub fn get_page_url(&mut self) -> crate::include::internal::CefStringUserFree {
    unsafe {
      let ret = match self.raw.as_ref().get_page_url {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefStringUserFree::from_cef(ret).unwrap()
    }
  }
  /// Returns the URL of the subframe that the context menu was invoked on.
  pub fn get_frame_url(&mut self) -> crate::include::internal::CefStringUserFree {
    unsafe {
      let ret = match self.raw.as_ref().get_frame_url {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefStringUserFree::from_cef(ret).unwrap()
    }
  }
  /// Returns the character encoding of the subframe that the context menu was
  /// invoked on.
  pub fn get_frame_charset(&mut self) -> crate::include::internal::CefStringUserFree {
    unsafe {
      let ret = match self.raw.as_ref().get_frame_charset {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefStringUserFree::from_cef(ret).unwrap()
    }
  }
  /// Returns the type of context node that the context menu was invoked on.
  pub fn get_media_type(&mut self) -> crate::include::internal::CefContextMenuMediaType {
    unsafe {
      let ret = match self.raw.as_ref().get_media_type {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret.into()
    }
  }
  /// Returns flags representing the actions supported by the media element, if
  /// any, that the context menu was invoked on.
  pub fn get_media_state_flags(&mut self) -> crate::include::internal::CefContextMenuMediaStateFlags {
    unsafe {
      let ret = match self.raw.as_ref().get_media_state_flags {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret.into()
    }
  }
  /// Returns the text of the selection, if any, that the context menu was
  /// invoked on.
  pub fn get_selection_text(&mut self) -> crate::include::internal::CefStringUserFree {
    unsafe {
      let ret = match self.raw.as_ref().get_selection_text {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefStringUserFree::from_cef(ret).unwrap()
    }
  }
  /// Returns the text of the misspelled word, if any, that the context menu was
  /// invoked on.
  pub fn get_misspelled_word(&mut self) -> crate::include::internal::CefStringUserFree {
    unsafe {
      let ret = match self.raw.as_ref().get_misspelled_word {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefStringUserFree::from_cef(ret).unwrap()
    }
  }
  /// Returns true if the context menu was invoked on an editable node.
  pub fn is_editable(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_editable {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns true if the context menu was invoked on an editable node where
  /// spell-check is enabled.
  pub fn is_spell_check_enabled(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_spell_check_enabled {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns flags representing the actions supported by the editable node, if
  /// any, that the context menu was invoked on.
  pub fn get_edit_state_flags(&mut self) -> crate::include::internal::CefContextMenuEditStateFlags {
    unsafe {
      let ret = match self.raw.as_ref().get_edit_state_flags {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret.into()
    }
  }
  /// Returns true if the context menu contains items specified by the renderer
  /// process (for example, plugin placeholder or pepper plugin menu items).
  pub fn is_custom_menu(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_custom_menu {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns true if the context menu was invoked from a pepper plugin.
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
