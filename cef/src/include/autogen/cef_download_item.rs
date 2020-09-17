pub type CefDownloadItem = crate::include::base::CefProxy<cef_sys::cef_download_item_t>;
#[allow(non_snake_case)]
impl CefDownloadItem {
  /// Returns true if this object is valid. Do not call any other methods if this
  /// function returns false.
  pub fn is_valid(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_valid {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns true if the download is in progress.
  pub fn is_in_progress(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_in_progress {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns true if the download is complete.
  pub fn is_complete(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_complete {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns true if the download has been canceled or interrupted.
  pub fn is_canceled(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_canceled {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns a simple speed estimate in bytes/s.
  pub fn get_current_speed(&mut self) -> i64 {
    unsafe {
      let ret = match self.raw.as_ref().get_current_speed {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  /// Returns the rough percent complete or -1 if the receive total size is
  /// unknown.
  pub fn get_percent_complete(&mut self) -> i32 {
    unsafe {
      let ret = match self.raw.as_ref().get_percent_complete {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  /// Returns the total number of bytes.
  pub fn get_total_bytes(&mut self) -> i64 {
    unsafe {
      let ret = match self.raw.as_ref().get_total_bytes {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  /// Returns the number of received bytes.
  pub fn get_received_bytes(&mut self) -> i64 {
    unsafe {
      let ret = match self.raw.as_ref().get_received_bytes {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  /// Returns the time that the download started.
  pub fn get_start_time(&mut self) -> crate::include::internal::CefTime {
    unsafe {
      let ret = match self.raw.as_ref().get_start_time {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret.into()
    }
  }
  /// Returns the time that the download ended.
  pub fn get_end_time(&mut self) -> crate::include::internal::CefTime {
    unsafe {
      let ret = match self.raw.as_ref().get_end_time {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret.into()
    }
  }
  /// Returns the full path to the downloaded or downloading file.
  pub fn get_full_path(&mut self) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_full_path {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  /// Returns the unique identifier for this download.
  pub fn get_id(&mut self) -> u32 {
    unsafe {
      let ret = match self.raw.as_ref().get_id {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
  /// Returns the URL.
  pub fn get_url(&mut self) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_url {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  /// Returns the original URL before any redirections.
  pub fn get_original_url(&mut self) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_original_url {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  /// Returns the suggested file name.
  pub fn get_suggested_file_name(&mut self) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_suggested_file_name {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  /// Returns the content disposition.
  pub fn get_content_disposition(&mut self) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_content_disposition {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  /// Returns the mime type.
  pub fn get_mime_type(&mut self) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_mime_type {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
}
