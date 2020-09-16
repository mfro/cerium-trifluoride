pub type CefImage = crate::include::base::CefProxy<cef_sys::cef_image_t>;
#[allow(non_snake_case)]
impl CefImage {
  pub fn is_empty(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_empty {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn is_same(&mut self, that: crate::include::CefImage) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_same {
        Some(f) => f(self.raw.as_ptr(),crate::include::CefImage::to_cef_own(that),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn get_width(&mut self) -> u64 {
    unsafe {
      let ret = match self.raw.as_ref().get_width {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn get_height(&mut self) -> u64 {
    unsafe {
      let ret = match self.raw.as_ref().get_height {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn has_representation(&mut self, scale_factor: f32) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().has_representation {
        Some(f) => f(self.raw.as_ptr(),scale_factor,),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn remove_representation(&mut self, scale_factor: f32) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().remove_representation {
        Some(f) => f(self.raw.as_ptr(),scale_factor,),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn get_representation_info(&mut self, scale_factor: f32, actual_scale_factor: &mut f32, pixel_width: &mut i32, pixel_height: &mut i32) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().get_representation_info {
        Some(f) => f(self.raw.as_ptr(),scale_factor,actual_scale_factor,pixel_width,pixel_height,),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  pub fn get_as_bitmap(&mut self, scale_factor: f32, color_type: crate::include::internal::CefColorType, alpha_type: crate::include::internal::CefAlphaType, pixel_width: &mut i32, pixel_height: &mut i32) -> Option<crate::include::CefBinaryValue> {
    unsafe {
      let ret = match self.raw.as_ref().get_as_bitmap {
        Some(f) => f(self.raw.as_ptr(),scale_factor,color_type.into(),alpha_type.into(),pixel_width,pixel_height,),
        None => panic!(),
      };
      crate::include::CefBinaryValue::from_cef_own(ret)
    }
  }
  pub fn get_as_png(&mut self, scale_factor: f32, with_transparency: bool, pixel_width: &mut i32, pixel_height: &mut i32) -> Option<crate::include::CefBinaryValue> {
    unsafe {
      let ret = match self.raw.as_ref().get_as_png {
        Some(f) => f(self.raw.as_ptr(),scale_factor,if with_transparency { 1 } else { 0 },pixel_width,pixel_height,),
        None => panic!(),
      };
      crate::include::CefBinaryValue::from_cef_own(ret)
    }
  }
  pub fn get_as_jpeg(&mut self, scale_factor: f32, quality: i32, pixel_width: &mut i32, pixel_height: &mut i32) -> Option<crate::include::CefBinaryValue> {
    unsafe {
      let ret = match self.raw.as_ref().get_as_jpeg {
        Some(f) => f(self.raw.as_ptr(),scale_factor,quality,pixel_width,pixel_height,),
        None => panic!(),
      };
      crate::include::CefBinaryValue::from_cef_own(ret)
    }
  }
}
