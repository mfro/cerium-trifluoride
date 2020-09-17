pub type CefX509CertPrincipal = crate::include::base::CefProxy<cef_sys::cef_x509cert_principal_t>;
#[allow(non_snake_case)]
impl CefX509CertPrincipal {
  /// Returns a name that can be used to represent the issuer. It tries in this
  /// order: Common Name (CN), Organization Name (O) and Organizational Unit
  /// Name (OU) and returns the first non-empty one found.
  pub fn get_display_name(&mut self) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_display_name {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  /// Returns the common name.
  pub fn get_common_name(&mut self) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_common_name {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  /// Returns the locality name.
  pub fn get_locality_name(&mut self) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_locality_name {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  /// Returns the state or province name.
  pub fn get_state_or_province_name(&mut self) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_state_or_province_name {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  /// Returns the country name.
  pub fn get_country_name(&mut self) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_country_name {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
}
pub type CefX509Certificate = crate::include::base::CefProxy<cef_sys::cef_x509certificate_t>;
#[allow(non_snake_case)]
impl CefX509Certificate {
  /// Returns the subject of the X.509 certificate. For HTTPS server
  /// certificates this represents the web server.  The common name of the
  /// subject should match the host name of the web server.
  pub fn get_subject(&mut self) -> Option<crate::include::CefX509CertPrincipal> {
    unsafe {
      let ret = match self.raw.as_ref().get_subject {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefX509CertPrincipal::from_cef_own(ret)
    }
  }
  /// Returns the issuer of the X.509 certificate.
  pub fn get_issuer(&mut self) -> Option<crate::include::CefX509CertPrincipal> {
    unsafe {
      let ret = match self.raw.as_ref().get_issuer {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefX509CertPrincipal::from_cef_own(ret)
    }
  }
  /// Returns the DER encoded serial number for the X.509 certificate. The value
  /// possibly includes a leading 00 byte.
  pub fn get_serial_number(&mut self) -> Option<crate::include::CefBinaryValue> {
    unsafe {
      let ret = match self.raw.as_ref().get_serial_number {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefBinaryValue::from_cef_own(ret)
    }
  }
  /// Returns the date before which the X.509 certificate is invalid.
  /// CefTime.GetTimeT() will return 0 if no date was specified.
  pub fn get_valid_start(&mut self) -> crate::include::internal::CefTime {
    unsafe {
      let ret = match self.raw.as_ref().get_valid_start {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret.into()
    }
  }
  /// Returns the date after which the X.509 certificate is invalid.
  /// CefTime.GetTimeT() will return 0 if no date was specified.
  pub fn get_valid_expiry(&mut self) -> crate::include::internal::CefTime {
    unsafe {
      let ret = match self.raw.as_ref().get_valid_expiry {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret.into()
    }
  }
  /// Returns the DER encoded data for the X.509 certificate.
  pub fn get_derencoded(&mut self) -> Option<crate::include::CefBinaryValue> {
    unsafe {
      let ret = match self.raw.as_ref().get_derencoded {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefBinaryValue::from_cef_own(ret)
    }
  }
  /// Returns the PEM encoded data for the X.509 certificate.
  pub fn get_pemencoded(&mut self) -> Option<crate::include::CefBinaryValue> {
    unsafe {
      let ret = match self.raw.as_ref().get_pemencoded {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::CefBinaryValue::from_cef_own(ret)
    }
  }
  /// Returns the number of certificates in the issuer chain.
  /// If 0, the certificate is self-signed.
  pub fn get_issuer_chain_size(&mut self) -> u64 {
    unsafe {
      let ret = match self.raw.as_ref().get_issuer_chain_size {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
}
