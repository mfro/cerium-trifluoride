pub type CefSSLInfo = crate::include::refcounting::CefProxy<cef_sys::cef_sslinfo_t>;
#[allow(non_snake_case)]
impl CefSSLInfo {
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
/// Returns true if the certificate status represents an error.
#[allow(non_snake_case)]
pub fn cef_is_cert_status_error(status: crate::include::internal::CefCertStatus, ) -> bool {
  unsafe {
    let ret = cef_sys::cef_is_cert_status_error(status.into(),);
    if ret == 0 { false } else { true }
  }
}
