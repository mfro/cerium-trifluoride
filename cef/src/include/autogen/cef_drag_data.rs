pub type CefDragData = crate::include::refcounting::CefProxy<cef_sys::cef_drag_data_t>;
#[allow(non_snake_case)]
impl CefDragData {
  /// Create a new CefDragData object.
  #[allow(non_snake_case)]
  pub fn create() -> Option<crate::include::CefDragData> {
    unsafe {
      let ret = cef_sys::cef_drag_data_create();
      crate::include::CefDragData::from_cef_own(ret)
    }
  }
  /// Returns a copy of the current object.
  pub fn clone(&mut self) -> Option<crate::include::CefDragData> {
    unsafe {
      let ret = match self.raw.as_ref().clone {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefDragData::from_cef_own(ret)
    }
  }
  /// Returns true if this object is read-only.
  pub fn is_read_only(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_read_only {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns true if the drag data is a link.
  pub fn is_link(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_link {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns true if the drag data is a text or html fragment.
  pub fn is_fragment(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_fragment {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns true if the drag data is a file.
  pub fn is_file(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_file {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Return the link URL that is being dragged.
  pub fn get_link_url(&mut self) -> crate::include::internal::CefStringUserFree {
    unsafe {
      let ret = match self.raw.as_ref().get_link_url {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefStringUserFree::from_cef(ret).unwrap()
    }
  }
  /// Return the title associated with the link being dragged.
  pub fn get_link_title(&mut self) -> crate::include::internal::CefStringUserFree {
    unsafe {
      let ret = match self.raw.as_ref().get_link_title {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefStringUserFree::from_cef(ret).unwrap()
    }
  }
  /// Return the metadata, if any, associated with the link being dragged.
  pub fn get_link_metadata(&mut self) -> crate::include::internal::CefStringUserFree {
    unsafe {
      let ret = match self.raw.as_ref().get_link_metadata {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefStringUserFree::from_cef(ret).unwrap()
    }
  }
  /// Return the plain text fragment that is being dragged.
  pub fn get_fragment_text(&mut self) -> crate::include::internal::CefStringUserFree {
    unsafe {
      let ret = match self.raw.as_ref().get_fragment_text {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefStringUserFree::from_cef(ret).unwrap()
    }
  }
  /// Return the text/html fragment that is being dragged.
  pub fn get_fragment_html(&mut self) -> crate::include::internal::CefStringUserFree {
    unsafe {
      let ret = match self.raw.as_ref().get_fragment_html {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefStringUserFree::from_cef(ret).unwrap()
    }
  }
  /// Return the base URL that the fragment came from. This value is used for
  /// resolving relative URLs and may be empty.
  pub fn get_fragment_base_url(&mut self) -> crate::include::internal::CefStringUserFree {
    unsafe {
      let ret = match self.raw.as_ref().get_fragment_base_url {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefStringUserFree::from_cef(ret).unwrap()
    }
  }
  /// Return the name of the file being dragged out of the browser window.
  pub fn get_file_name(&mut self) -> crate::include::internal::CefStringUserFree {
    unsafe {
      let ret = match self.raw.as_ref().get_file_name {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefStringUserFree::from_cef(ret).unwrap()
    }
  }
  /// Write the contents of the file being dragged out of the web view into
  /// |writer|. Returns the number of bytes sent to |writer|. If |writer| is
  /// NULL this method will return the size of the file contents in bytes.
  /// Call GetFileName() to get a suggested name for the file.
  pub fn get_file_contents(&mut self, writer: Option<crate::include::CefStreamWriter>) -> u64 {
    unsafe {
      let ret = match self.raw.as_ref().get_file_contents {
        Some(f) => f(self.raw.as_ptr(),writer.map_or(std::ptr::null_mut(), |o| crate::include::CefStreamWriter::to_cef_own(o)),),
        None => panic!(),
      };
      ret
    }
  }
  /// Set the link URL that is being dragged.
  pub fn set_link_url(&mut self, url: Option<&crate::include::internal::CefString>) -> () {
    unsafe {
      let ret = match self.raw.as_ref().set_link_url {
        Some(f) => f(self.raw.as_ptr(),match url { Some(url) => url as *const _ as *const _, None => std::ptr::null_mut() },),
        None => panic!(),
      };
      ret
    }
  }
  /// Set the title associated with the link being dragged.
  pub fn set_link_title(&mut self, title: Option<&crate::include::internal::CefString>) -> () {
    unsafe {
      let ret = match self.raw.as_ref().set_link_title {
        Some(f) => f(self.raw.as_ptr(),match title { Some(title) => title as *const _ as *const _, None => std::ptr::null_mut() },),
        None => panic!(),
      };
      ret
    }
  }
  /// Set the metadata associated with the link being dragged.
  pub fn set_link_metadata(&mut self, data: Option<&crate::include::internal::CefString>) -> () {
    unsafe {
      let ret = match self.raw.as_ref().set_link_metadata {
        Some(f) => f(self.raw.as_ptr(),match data { Some(data) => data as *const _ as *const _, None => std::ptr::null_mut() },),
        None => panic!(),
      };
      ret
    }
  }
  /// Set the plain text fragment that is being dragged.
  pub fn set_fragment_text(&mut self, text: Option<&crate::include::internal::CefString>) -> () {
    unsafe {
      let ret = match self.raw.as_ref().set_fragment_text {
        Some(f) => f(self.raw.as_ptr(),match text { Some(text) => text as *const _ as *const _, None => std::ptr::null_mut() },),
        None => panic!(),
      };
      ret
    }
  }
  /// Set the text/html fragment that is being dragged.
  pub fn set_fragment_html(&mut self, html: Option<&crate::include::internal::CefString>) -> () {
    unsafe {
      let ret = match self.raw.as_ref().set_fragment_html {
        Some(f) => f(self.raw.as_ptr(),match html { Some(html) => html as *const _ as *const _, None => std::ptr::null_mut() },),
        None => panic!(),
      };
      ret
    }
  }
  /// Set the base URL that the fragment came from.
  pub fn set_fragment_base_url(&mut self, base_url: Option<&crate::include::internal::CefString>) -> () {
    unsafe {
      let ret = match self.raw.as_ref().set_fragment_base_url {
        Some(f) => f(self.raw.as_ptr(),match base_url { Some(base_url) => base_url as *const _ as *const _, None => std::ptr::null_mut() },),
        None => panic!(),
      };
      ret
    }
  }
  /// Reset the file contents. You should do this before calling
  /// CefBrowserHost::DragTargetDragEnter as the web view does not allow us to
  /// drag in this kind of data.
  pub fn reset_file_contents(&mut self) -> () {
    unsafe {
      let ret = match self.raw.as_ref().reset_file_contents {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  /// Add a file that is being dragged into the webview.
  pub fn add_file(&mut self, path: &crate::include::internal::CefString, display_name: Option<&crate::include::internal::CefString>) -> () {
    unsafe {
      let ret = match self.raw.as_ref().add_file {
        Some(f) => f(self.raw.as_ptr(),path as *const _ as *const _,match display_name { Some(display_name) => display_name as *const _ as *const _, None => std::ptr::null_mut() },),
        None => panic!(),
      };
      ret
    }
  }
  /// Get the image representation of drag data. May return NULL if no image
  /// representation is available.
  pub fn get_image(&mut self) -> Option<crate::include::CefImage> {
    unsafe {
      let ret = match self.raw.as_ref().get_image {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefImage::from_cef_own(ret)
    }
  }
  /// Get the image hotspot (drag start location relative to image dimensions).
  pub fn get_image_hotspot(&mut self) -> crate::include::internal::CefPoint {
    unsafe {
      let ret = match self.raw.as_ref().get_image_hotspot {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret.into()
    }
  }
  /// Returns true if an image representation of drag data is available.
  pub fn has_image(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().has_image {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
}
