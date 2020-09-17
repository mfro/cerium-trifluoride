pub type CefSSLStatus = crate::include::base::CefProxy<cef_sys::cef_sslstatus_t>;
#[allow(non_snake_case)]
impl CefSSLStatus {
  /// Returns true if the status is related to a secure SSL/TLS connection.
  pub fn is_secure_connection(&mut self) -> bool {
    unsafe {
      let ret = match self.raw.as_ref().is_secure_connection {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      if ret == 0 { false } else { true }
    }
  }
  /// Returns a bitmask containing any and all problems verifying the server
  /// certificate.
  pub fn get_cert_status(&mut self) -> crate::include::internal::CefCertStatus {
    unsafe {
      let ret = match self.raw.as_ref().get_cert_status {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret.into()
    }
  }
  /// Returns the SSL version used for the SSL connection.
  pub fn get_sslversion(&mut self) -> crate::include::internal::CefSslVersion {
    unsafe {
      let ret = match self.raw.as_ref().get_sslversion {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret.into()
    }
  }
  /// Returns a bitmask containing the page security content status.
  pub fn get_content_status(&mut self) -> crate::include::internal::CefSslContentStatus {
    unsafe {
      let ret = match self.raw.as_ref().get_content_status {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret.into()
    }
  }
  /// Returns the X.509 certificate.
  pub fn get_x509certificate(&mut self) -> Option<crate::include::CefX509Certificate> {
    unsafe {
      let ret = match self.raw.as_ref().get_x509certificate {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefX509Certificate::from_cef_own(ret)
    }
  }
}
