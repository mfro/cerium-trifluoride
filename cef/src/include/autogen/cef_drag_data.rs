pub type CefDragData = crate::include::base::CefProxy<cef_sys::cef_drag_data_t>;
#[allow(non_snake_case)]
impl CefDragData {
  pub fn clone(&mut self) -> Option<crate::include::CefDragData> {
    unsafe {
      let ret = match self.raw.as_ref().clone {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefDragData::from_cef_own(ret)
    }
  }
  pub fn is_read_only(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_read_only {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn is_link(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_link {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn is_fragment(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_fragment {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn is_file(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_file {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
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
  pub fn get_link_title(&mut self) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_link_title {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  pub fn get_link_metadata(&mut self) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_link_metadata {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  pub fn get_fragment_text(&mut self) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_fragment_text {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  pub fn get_fragment_html(&mut self) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_fragment_html {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  pub fn get_fragment_base_url(&mut self) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_fragment_base_url {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  pub fn get_file_name(&mut self) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_file_name {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  pub fn get_file_contents(&mut self, writer: Option<crate::include::CefStreamWriter>) -> u64 {
    unsafe {
      let ret = match self.raw.as_ref().get_file_contents {
        Some(f) => f(self.raw.as_ptr(),writer.map_or(std::ptr::null_mut(), |o| crate::include::CefStreamWriter::to_cef_own(o)),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn set_link_url(&mut self, url: Option<&crate::include::internal::CefString>) -> () {
    unsafe {
      let ret = match self.raw.as_ref().set_link_url {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(url),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn set_link_title(&mut self, title: Option<&crate::include::internal::CefString>) -> () {
    unsafe {
      let ret = match self.raw.as_ref().set_link_title {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(title),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn set_link_metadata(&mut self, data: Option<&crate::include::internal::CefString>) -> () {
    unsafe {
      let ret = match self.raw.as_ref().set_link_metadata {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(data),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn set_fragment_text(&mut self, text: Option<&crate::include::internal::CefString>) -> () {
    unsafe {
      let ret = match self.raw.as_ref().set_fragment_text {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(text),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn set_fragment_html(&mut self, html: Option<&crate::include::internal::CefString>) -> () {
    unsafe {
      let ret = match self.raw.as_ref().set_fragment_html {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(html),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn set_fragment_base_url(&mut self, base_url: Option<&crate::include::internal::CefString>) -> () {
    unsafe {
      let ret = match self.raw.as_ref().set_fragment_base_url {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(base_url),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn reset_file_contents(&mut self) -> () {
    unsafe {
      let ret = match self.raw.as_ref().reset_file_contents {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn add_file(&mut self, path: &crate::include::internal::CefString, display_name: Option<&crate::include::internal::CefString>) -> () {
    unsafe {
      let ret = match self.raw.as_ref().add_file {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(path),crate::include::internal::IntoCef::into_cef(display_name),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn get_image(&mut self) -> Option<crate::include::CefImage> {
    unsafe {
      let ret = match self.raw.as_ref().get_image {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefImage::from_cef_own(ret)
    }
  }
  pub fn get_image_hotspot(&mut self) -> crate::include::internal::CefPoint {
    unsafe {
      let ret = match self.raw.as_ref().get_image_hotspot {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret.into()
    }
  }
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
