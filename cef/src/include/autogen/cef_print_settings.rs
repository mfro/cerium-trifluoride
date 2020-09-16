pub type CefPrintSettings = crate::include::base::CefProxy<cef_sys::cef_print_settings_t>;
#[allow(non_snake_case)]
impl CefPrintSettings {
  pub fn is_valid(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_valid {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
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
  pub fn set_orientation(&mut self, landscape: bool) -> () {
    unsafe {
      let ret = match self.raw.as_ref().set_orientation {
        Some(f) => f(self.raw.as_ptr(),if landscape { 1 } else { 0 },),
        None => panic!(),
      };
      ret
    }
  }
  pub fn is_landscape(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_landscape {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn set_printer_printable_area(&mut self, physical_size_device_units: &crate::include::internal::CefSize, printable_area_device_units: &crate::include::internal::CefRect, landscape_needs_flip: bool) -> () {
    unsafe {
      let ret = match self.raw.as_ref().set_printer_printable_area {
        Some(f) => f(self.raw.as_ptr(),&physical_size_device_units.raw,&printable_area_device_units.raw,if landscape_needs_flip { 1 } else { 0 },),
        None => panic!(),
      };
      ret
    }
  }
  pub fn set_device_name(&mut self, name: Option<&crate::include::internal::CefString>) -> () {
    unsafe {
      let ret = match self.raw.as_ref().set_device_name {
        Some(f) => f(self.raw.as_ptr(),crate::include::internal::IntoCef::into_cef(name),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn get_device_name(&mut self) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_device_name {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  pub fn set_dpi(&mut self, dpi: i32) -> () {
    unsafe {
      let ret = match self.raw.as_ref().set_dpi {
        Some(f) => f(self.raw.as_ptr(),dpi,),
        None => panic!(),
      };
      ret
    }
  }
  pub fn get_dpi(&mut self) -> i32 {
    unsafe {
      let ret = match self.raw.as_ref().get_dpi {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn get_page_ranges_count(&mut self) -> u64 {
    unsafe {
      let ret = match self.raw.as_ref().get_page_ranges_count {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn set_selection_only(&mut self, selection_only: bool) -> () {
    unsafe {
      let ret = match self.raw.as_ref().set_selection_only {
        Some(f) => f(self.raw.as_ptr(),if selection_only { 1 } else { 0 },),
        None => panic!(),
      };
      ret
    }
  }
  pub fn is_selection_only(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_selection_only {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn set_collate(&mut self, collate: bool) -> () {
    unsafe {
      let ret = match self.raw.as_ref().set_collate {
        Some(f) => f(self.raw.as_ptr(),if collate { 1 } else { 0 },),
        None => panic!(),
      };
      ret
    }
  }
  pub fn will_collate(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().will_collate {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn set_color_model(&mut self, model: crate::include::internal::CefColorModel) -> () {
    unsafe {
      let ret = match self.raw.as_ref().set_color_model {
        Some(f) => f(self.raw.as_ptr(),model.into(),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn get_color_model(&mut self) -> crate::include::internal::CefColorModel {
    unsafe {
      let ret = match self.raw.as_ref().get_color_model {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret.into()
    }
  }
  pub fn set_copies(&mut self, copies: i32) -> () {
    unsafe {
      let ret = match self.raw.as_ref().set_copies {
        Some(f) => f(self.raw.as_ptr(),copies,),
        None => panic!(),
      };
      ret
    }
  }
  pub fn get_copies(&mut self) -> i32 {
    unsafe {
      let ret = match self.raw.as_ref().get_copies {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn set_duplex_mode(&mut self, mode: crate::include::internal::CefDuplexMode) -> () {
    unsafe {
      let ret = match self.raw.as_ref().set_duplex_mode {
        Some(f) => f(self.raw.as_ptr(),mode.into(),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn get_duplex_mode(&mut self) -> crate::include::internal::CefDuplexMode {
    unsafe {
      let ret = match self.raw.as_ref().get_duplex_mode {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret.into()
    }
  }
}
