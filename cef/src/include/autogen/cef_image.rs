pub type CefImage = crate::include::base::CefProxy<cef_sys::cef_image_t>;
#[allow(non_snake_case)]
impl CefImage {
  /// Create a new CefImage. It will initially be empty. Use the Add*() methods
  /// to add representations at different scale factors.
  #[allow(non_snake_case)]
  pub fn create_image() -> Option<crate::include::CefImage> {
    unsafe {
      let ret = cef_sys::cef_image_create();
      crate::include::CefImage::from_cef_own(ret)
    }
  }
  /// Returns true if this Image is empty.
  pub fn is_empty(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_empty {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns true if this Image and |that| Image share the same underlying
  /// storage. Will also return true if both images are empty.
  pub fn is_same(&mut self, that: crate::include::CefImage) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_same {
        Some(f) => f(self.raw.as_ptr(),crate::include::CefImage::to_cef_own(that),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns the image width in density independent pixel (DIP) units.
  pub fn get_width(&mut self) -> u64 {
    unsafe {
      let ret = match self.raw.as_ref().get_width {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  /// Returns the image height in density independent pixel (DIP) units.
  pub fn get_height(&mut self) -> u64 {
    unsafe {
      let ret = match self.raw.as_ref().get_height {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  /// Returns true if this image contains a representation for |scale_factor|.
  pub fn has_representation(&mut self, scale_factor: f32) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().has_representation {
        Some(f) => f(self.raw.as_ptr(),scale_factor,),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Removes the representation for |scale_factor|. Returns true on success.
  pub fn remove_representation(&mut self, scale_factor: f32) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().remove_representation {
        Some(f) => f(self.raw.as_ptr(),scale_factor,),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns information for the representation that most closely matches
  /// |scale_factor|. |actual_scale_factor| is the actual scale factor for the
  /// representation. |pixel_width| and |pixel_height| are the representation
  /// size in pixel coordinates. Returns true on success.
  pub fn get_representation_info(&mut self, scale_factor: f32, actual_scale_factor: &mut f32, pixel_width: &mut i32, pixel_height: &mut i32) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().get_representation_info {
        Some(f) => f(self.raw.as_ptr(),scale_factor,actual_scale_factor,pixel_width,pixel_height,),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns the bitmap representation that most closely matches |scale_factor|.
  /// Only 32-bit RGBA/BGRA formats are supported. |color_type| and |alpha_type|
  /// values specify the desired output pixel format. |pixel_width| and
  /// |pixel_height| are the output representation size in pixel coordinates.
  /// Returns a CefBinaryValue containing the pixel data on success or NULL on
  /// failure.
  pub fn get_as_bitmap(&mut self, scale_factor: f32, color_type: crate::include::internal::CefColorType, alpha_type: crate::include::internal::CefAlphaType, pixel_width: &mut i32, pixel_height: &mut i32) -> Option<crate::include::CefBinaryValue> {
    unsafe {
      let ret = match self.raw.as_ref().get_as_bitmap {
        Some(f) => f(self.raw.as_ptr(),scale_factor,color_type.into(),alpha_type.into(),pixel_width,pixel_height,),
        None => panic!(),
      };
      crate::include::CefBinaryValue::from_cef_own(ret)
    }
  }
  /// Returns the PNG representation that most closely matches |scale_factor|. If
  /// |with_transparency| is true any alpha transparency in the image will be
  /// represented in the resulting PNG data. |pixel_width| and |pixel_height| are
  /// the output representation size in pixel coordinates. Returns a
  /// CefBinaryValue containing the PNG image data on success or NULL on failure.
  pub fn get_as_png(&mut self, scale_factor: f32, with_transparency: bool, pixel_width: &mut i32, pixel_height: &mut i32) -> Option<crate::include::CefBinaryValue> {
    unsafe {
      let ret = match self.raw.as_ref().get_as_png {
        Some(f) => f(self.raw.as_ptr(),scale_factor,if with_transparency { 1 } else { 0 },pixel_width,pixel_height,),
        None => panic!(),
      };
      crate::include::CefBinaryValue::from_cef_own(ret)
    }
  }
  /// Returns the JPEG representation that most closely matches |scale_factor|.
  /// |quality| determines the compression level with 0 == lowest and 100 ==
  /// highest. The JPEG format does not support alpha transparency and the alpha
  /// channel, if any, will be discarded. |pixel_width| and |pixel_height| are
  /// the output representation size in pixel coordinates. Returns a
  /// CefBinaryValue containing the JPEG image data on success or NULL on
  /// failure.
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
