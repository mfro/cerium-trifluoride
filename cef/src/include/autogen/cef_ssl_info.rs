pub type CefSSLInfo = crate::include::base::CefProxy<cef_sys::cef_sslinfo_t>;
#[allow(non_snake_case)]
impl CefSSLInfo {
  pub fn get_cert_status(&mut self) -> crate::include::internal::CefCertStatus {
    unsafe {
      let ret = match self.raw.as_ref().get_cert_status {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret.into()
    }
  }
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
