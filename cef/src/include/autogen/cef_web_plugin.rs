pub type CefWebPluginInfo = crate::include::base::CefProxy<cef_sys::cef_web_plugin_info_t>;
#[allow(non_snake_case)]
impl CefWebPluginInfo {
  pub fn get_name(&mut self) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_name {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  pub fn get_path(&mut self) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_path {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  pub fn get_version(&mut self) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_version {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
  pub fn get_description(&mut self) -> crate::include::internal::CefString {
    unsafe {
      let ret = match self.raw.as_ref().get_description {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefString::userfree(ret)
    }
  }
}
/// Interface to implement for visiting web plugin information. The methods of
/// this class will be called on the browser process UI thread.
#[allow(non_snake_case)]
pub trait WebPluginInfoVisitor {
  fn visit(&mut self, info: crate::include::CefWebPluginInfo, count: i32, total: i32) -> bool { Default::default() }
}
define_refcounted!(WebPluginInfoVisitor, web_plugin_info_visitor, visit,);
#[allow(non_snake_case)]
unsafe extern "C" fn cef_web_plugin_info_visitor_t_visit(_self: *mut cef_sys::cef_web_plugin_info_visitor_t, info: *mut cef_sys::cef_web_plugin_info_t, count: i32, total: i32) -> i32 {
  let ret = CefWebPluginInfoVisitor::from_cef(_self, true).get().visit(crate::include::CefWebPluginInfo::from_cef_own(info).unwrap(),count,total,);
  if ret { 1 } else { 0 }
}
/// Interface to implement for receiving unstable plugin information. The methods
/// of this class will be called on the browser process IO thread.
#[allow(non_snake_case)]
pub trait WebPluginUnstableCallback {
  fn is_unstable(&mut self, path: &crate::include::internal::CefString, unstable: bool) -> () { Default::default() }
}
define_refcounted!(WebPluginUnstableCallback, web_plugin_unstable_callback, is_unstable,);
#[allow(non_snake_case)]
unsafe extern "C" fn cef_web_plugin_unstable_callback_t_is_unstable(_self: *mut cef_sys::cef_web_plugin_unstable_callback_t, path: *const cef_sys::cef_string_t, unstable: i32) -> () {
  let ret = CefWebPluginUnstableCallback::from_cef(_self, true).get().is_unstable(&crate::include::internal::CefString::from_cef(path).unwrap(),if unstable == 0 { false } else { true },);
  ret
}
/// Implement this interface to receive notification when CDM registration is
/// complete. The methods of this class will be called on the browser process
/// UI thread.
#[allow(non_snake_case)]
pub trait RegisterCdmCallback {
  fn on_cdm_registration_complete(&mut self, result: crate::include::internal::CefCdmRegistrationError, error_message: Option<&crate::include::internal::CefString>) -> () { Default::default() }
}
define_refcounted!(RegisterCdmCallback, register_cdm_callback, on_cdm_registration_complete,);
#[allow(non_snake_case)]
unsafe extern "C" fn cef_register_cdm_callback_t_on_cdm_registration_complete(_self: *mut cef_sys::cef_register_cdm_callback_t, result: cef_sys::cef_cdm_registration_error_t, error_message: *const cef_sys::cef_string_t) -> () {
  let ret = CefRegisterCdmCallback::from_cef(_self, true).get().on_cdm_registration_complete(result.into(),match &crate::include::internal::CefString::from_cef(error_message) { Some(ref x) => Some(x), None => None },);
  ret
}
