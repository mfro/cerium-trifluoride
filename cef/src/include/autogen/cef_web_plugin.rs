pub type CefWebPluginInfo = crate::include::refcounting::CefProxy<cef_sys::cef_web_plugin_info_t>;
#[allow(non_snake_case)]
impl CefWebPluginInfo {
  /// Returns the plugin name (i.e. Flash).
  pub fn get_name(&mut self) -> crate::include::internal::CefStringUserFree {
    unsafe {
      let ret = match self.raw.as_ref().get_name {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefStringUserFree::from_cef(ret).unwrap()
    }
  }
  /// Returns the plugin file path (DLL/bundle/library).
  pub fn get_path(&mut self) -> crate::include::internal::CefStringUserFree {
    unsafe {
      let ret = match self.raw.as_ref().get_path {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefStringUserFree::from_cef(ret).unwrap()
    }
  }
  /// Returns the version of the plugin (may be OS-specific).
  pub fn get_version(&mut self) -> crate::include::internal::CefStringUserFree {
    unsafe {
      let ret = match self.raw.as_ref().get_version {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefStringUserFree::from_cef(ret).unwrap()
    }
  }
  /// Returns a description of the plugin from the version information.
  pub fn get_description(&mut self) -> crate::include::internal::CefStringUserFree {
    unsafe {
      let ret = match self.raw.as_ref().get_description {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      crate::include::internal::CefStringUserFree::from_cef(ret).unwrap()
    }
  }
}
/// Interface to implement for visiting web plugin information. The methods of
/// this class will be called on the browser process UI thread.
#[allow(non_snake_case)]
#[allow(unused_variables)]
pub trait WebPluginInfoVisitor {
  /// Method that will be called once for each plugin. |count| is the 0-based
  /// index for the current plugin. |total| is the total number of plugins.
  /// Return false to stop visiting plugins. This method may never be called if
  /// no plugins are found.
  fn visit(&mut self, info: crate::include::CefWebPluginInfo, count: i32, total: i32) -> bool { Default::default() }
}
define_refcounted!(WebPluginInfoVisitor, CefWebPluginInfoVisitor, cef_web_plugin_info_visitor_t, visit: cef_web_plugin_info_visitor_t_visit,);
#[allow(non_snake_case)]
unsafe extern "C" fn cef_web_plugin_info_visitor_t_visit(_self: *mut cef_sys::cef_web_plugin_info_visitor_t, info: *mut cef_sys::cef_web_plugin_info_t, count: i32, total: i32) -> i32 {
  let ret = CefWebPluginInfoVisitor::from_cef(_self, true).get().visit(crate::include::CefWebPluginInfo::from_cef_own(info).unwrap(),count,total,);
  if ret { 1 } else { 0 }
}
/// Interface to implement for receiving unstable plugin information. The methods
/// of this class will be called on the browser process IO thread.
#[allow(non_snake_case)]
#[allow(unused_variables)]
pub trait WebPluginUnstableCallback {
  /// Method that will be called for the requested plugin. |unstable| will be
  /// true if the plugin has reached the crash count threshold of 3 times in 120
  /// seconds.
  fn is_unstable(&mut self, path: &crate::include::internal::CefString, unstable: bool) -> () { Default::default() }
}
define_refcounted!(WebPluginUnstableCallback, CefWebPluginUnstableCallback, cef_web_plugin_unstable_callback_t, is_unstable: cef_web_plugin_unstable_callback_t_is_unstable,);
#[allow(non_snake_case)]
unsafe extern "C" fn cef_web_plugin_unstable_callback_t_is_unstable(_self: *mut cef_sys::cef_web_plugin_unstable_callback_t, path: *const cef_sys::cef_string_t, unstable: i32) -> () {
  let ret = CefWebPluginUnstableCallback::from_cef(_self, true).get().is_unstable(&*(path as *const _),if unstable == 0 { false } else { true },);
  ret
}
/// Implement this interface to receive notification when CDM registration is
/// complete. The methods of this class will be called on the browser process
/// UI thread.
#[allow(non_snake_case)]
#[allow(unused_variables)]
pub trait RegisterCdmCallback {
  /// Method that will be called when CDM registration is complete. |result|
  /// will be CEF_CDM_REGISTRATION_ERROR_NONE if registration completed
  /// successfully. Otherwise, |result| and |error_message| will contain
  /// additional information about why registration failed.
  fn on_cdm_registration_complete(&mut self, result: crate::include::internal::CefCdmRegistrationError, error_message: Option<&crate::include::internal::CefString>) -> () { Default::default() }
}
define_refcounted!(RegisterCdmCallback, CefRegisterCdmCallback, cef_register_cdm_callback_t, on_cdm_registration_complete: cef_register_cdm_callback_t_on_cdm_registration_complete,);
#[allow(non_snake_case)]
unsafe extern "C" fn cef_register_cdm_callback_t_on_cdm_registration_complete(_self: *mut cef_sys::cef_register_cdm_callback_t, result: cef_sys::cef_cdm_registration_error_t, error_message: *const cef_sys::cef_string_t) -> () {
  let ret = CefRegisterCdmCallback::from_cef(_self, true).get().on_cdm_registration_complete(result.into(),if error_message.is_null() { None } else { Some(&*(error_message as *const _)) },);
  ret
}
/// Visit web plugin information. Can be called on any thread in the browser
/// process.
#[allow(non_snake_case)]
pub fn cef_visit_web_plugin_info(visitor: crate::include::CefWebPluginInfoVisitor, ) -> () {
  unsafe {
    let ret = cef_sys::cef_visit_web_plugin_info(crate::include::CefWebPluginInfoVisitor::to_cef_own(visitor),);
    ret
  }
}
/// Cause the plugin list to refresh the next time it is accessed regardless
/// of whether it has already been loaded. Can be called on any thread in the
/// browser process.
#[allow(non_snake_case)]
pub fn cef_refresh_web_plugins() -> () {
  unsafe {
    let ret = cef_sys::cef_refresh_web_plugins();
    ret
  }
}
/// Unregister an internal plugin. This may be undone the next time
/// CefRefreshWebPlugins() is called. Can be called on any thread in the browser
/// process.
#[allow(non_snake_case)]
pub fn cef_unregister_internal_web_plugin(path: &crate::include::internal::CefString, ) -> () {
  unsafe {
    let ret = cef_sys::cef_unregister_internal_web_plugin(path as *const _ as *const _,);
    ret
  }
}
/// Register a plugin crash. Can be called on any thread in the browser process
/// but will be executed on the IO thread.
#[allow(non_snake_case)]
pub fn cef_register_web_plugin_crash(path: &crate::include::internal::CefString, ) -> () {
  unsafe {
    let ret = cef_sys::cef_register_web_plugin_crash(path as *const _ as *const _,);
    ret
  }
}
/// Query if a plugin is unstable. Can be called on any thread in the browser
/// process.
#[allow(non_snake_case)]
pub fn cef_is_web_plugin_unstable(path: &crate::include::internal::CefString, callback: crate::include::CefWebPluginUnstableCallback, ) -> () {
  unsafe {
    let ret = cef_sys::cef_is_web_plugin_unstable(path as *const _ as *const _,crate::include::CefWebPluginUnstableCallback::to_cef_own(callback),);
    ret
  }
}
/// Register the Widevine CDM plugin.
/// 
/// The client application is responsible for downloading an appropriate
/// platform-specific CDM binary distribution from Google, extracting the
/// contents, and building the required directory structure on the local machine.
/// The CefBrowserHost::StartDownload method and CefZipArchive class can be used
/// to implement this functionality in CEF. Contact Google via
/// https://www.widevine.com/contact.html for details on CDM download.
/// 
/// |path| is a directory that must contain the following files:
/// 1. manifest.json file from the CDM binary distribution (see below).
/// 2. widevinecdm file from the CDM binary distribution (e.g.
/// widevinecdm.dll on on Windows, libwidevinecdm.dylib on OS X,
/// libwidevinecdm.so on Linux).
/// 
/// If any of these files are missing or if the manifest file has incorrect
/// contents the registration will fail and |callback| will receive a |result|
/// value of CEF_CDM_REGISTRATION_ERROR_INCORRECT_CONTENTS.
/// 
/// The manifest.json file must contain the following keys:
/// A. "os": Supported OS (e.g. "mac", "win" or "linux").
/// B. "arch": Supported architecture (e.g. "ia32" or "x64").
/// C. "x-cdm-module-versions": Module API version (e.g. "4").
/// D. "x-cdm-interface-versions": Interface API version (e.g. "8").
/// E. "x-cdm-host-versions": Host API version (e.g. "8").
/// F. "version": CDM version (e.g. "1.4.8.903").
/// G. "x-cdm-codecs": List of supported codecs (e.g. "vp8,vp9.0,avc1").
/// 
/// A through E are used to verify compatibility with the current Chromium
/// version. If the CDM is not compatible the registration will fail and
/// |callback| will receive a |result| value of
/// CEF_CDM_REGISTRATION_ERROR_INCOMPATIBLE.
/// 
/// |callback| will be executed asynchronously once registration is complete.
/// 
/// On Linux this function must be called before CefInitialize() and the
/// registration cannot be changed during runtime. If registration is not
/// supported at the time that CefRegisterWidevineCdm() is called then |callback|
/// will receive a |result| value of CEF_CDM_REGISTRATION_ERROR_NOT_SUPPORTED.
#[allow(non_snake_case)]
pub fn cef_register_widevine_cdm(path: &crate::include::internal::CefString, callback: Option<crate::include::CefRegisterCdmCallback>, ) -> () {
  unsafe {
    let ret = cef_sys::cef_register_widevine_cdm(path as *const _ as *const _,callback.map_or(std::ptr::null_mut(), |o| crate::include::CefRegisterCdmCallback::to_cef_own(o)),);
    ret
  }
}
