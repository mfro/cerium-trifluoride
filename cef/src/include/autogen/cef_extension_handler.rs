pub type CefGetExtensionResourceCallback = crate::include::base::CefProxy<cef_sys::cef_get_extension_resource_callback_t>;
#[allow(non_snake_case)]
impl CefGetExtensionResourceCallback {
  pub fn cont(&mut self, stream: Option<crate::include::CefStreamReader>) -> () {
    unsafe {
      let ret = match self.raw.as_ref().cont {
        Some(f) => f(self.raw.as_ptr(),stream.map_or(std::ptr::null_mut(), |o| crate::include::CefStreamReader::to_cef_own(o)),),
        None => panic!(),
      };
      ret
    }
  }
  pub fn cancel(&mut self) -> () {
    unsafe {
      let ret = match self.raw.as_ref().cancel {
        Some(f) => f(self.raw.as_ptr(),),
        None => panic!(),
      };
      ret
    }
  }
}
/// Implement this interface to handle events related to browser extensions.
/// The methods of this class will be called on the UI thread. See
/// CefRequestContext::LoadExtension for information about extension loading.
#[allow(non_snake_case)]
pub trait ExtensionHandler {
  fn on_extension_load_failed(&mut self, result: crate::include::internal::CefErrorcode) -> () { Default::default() }
  fn on_extension_loaded(&mut self, extension: crate::include::CefExtension) -> () { Default::default() }
  fn on_extension_unloaded(&mut self, extension: crate::include::CefExtension) -> () { Default::default() }
  fn on_before_background_browser(&mut self, extension: crate::include::CefExtension, url: &crate::include::internal::CefString, client: &mut crate::include::CefClient, settings: &mut crate::include::internal::CefBrowserSettings) -> bool { Default::default() }
  fn on_before_browser(&mut self, extension: crate::include::CefExtension, browser: crate::include::CefBrowser, active_browser: crate::include::CefBrowser, index: i32, url: &crate::include::internal::CefString, active: bool, windowInfo: &mut crate::include::internal::CefWindowInfo, client: &mut crate::include::CefClient, settings: &mut crate::include::internal::CefBrowserSettings) -> bool { Default::default() }
  fn get_active_browser(&mut self, extension: crate::include::CefExtension, browser: crate::include::CefBrowser, include_incognito: bool) -> Option<crate::include::CefBrowser> { Default::default() }
  fn can_access_browser(&mut self, extension: crate::include::CefExtension, browser: crate::include::CefBrowser, include_incognito: bool, target_browser: crate::include::CefBrowser) -> bool { Default::default() }
  fn get_extension_resource(&mut self, extension: crate::include::CefExtension, browser: crate::include::CefBrowser, file: &crate::include::internal::CefString, callback: crate::include::CefGetExtensionResourceCallback) -> bool { Default::default() }
}
define_refcounted!(ExtensionHandler, extension_handler, on_extension_load_failed,on_extension_loaded,on_extension_unloaded,on_before_background_browser,on_before_browser,get_active_browser,can_access_browser,get_extension_resource,);
#[allow(non_snake_case)]
unsafe extern "C" fn cef_extension_handler_t_on_extension_load_failed(_self: *mut cef_sys::cef_extension_handler_t, result: cef_sys::cef_errorcode_t) -> () {
  let ret = CefExtensionHandler::from_cef(_self, true).get().on_extension_load_failed(result.into(),);
  ret
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_extension_handler_t_on_extension_loaded(_self: *mut cef_sys::cef_extension_handler_t, extension: *mut cef_sys::cef_extension_t) -> () {
  let ret = CefExtensionHandler::from_cef(_self, true).get().on_extension_loaded(crate::include::CefExtension::from_cef_own(extension).unwrap(),);
  ret
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_extension_handler_t_on_extension_unloaded(_self: *mut cef_sys::cef_extension_handler_t, extension: *mut cef_sys::cef_extension_t) -> () {
  let ret = CefExtensionHandler::from_cef(_self, true).get().on_extension_unloaded(crate::include::CefExtension::from_cef_own(extension).unwrap(),);
  ret
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_extension_handler_t_on_before_background_browser(_self: *mut cef_sys::cef_extension_handler_t, extension: *mut cef_sys::cef_extension_t, url: *const cef_sys::cef_string_t, client: *mut *mut cef_sys::cef_client_t, settings: *mut cef_sys::cef_browser_settings_t) -> i32 {
  let mut client__tmp = crate::include::CefClient::from_cef_own(*client).unwrap();
  let ret = CefExtensionHandler::from_cef(_self, true).get().on_before_background_browser(crate::include::CefExtension::from_cef_own(extension).unwrap(),&crate::include::internal::CefString::from_cef(url).unwrap(),&mut client__tmp,&mut *(settings as *mut _),);
  *client = crate::include::CefClient::to_cef_own(client__tmp);
  if ret { 1 } else { 0 }
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_extension_handler_t_on_before_browser(_self: *mut cef_sys::cef_extension_handler_t, extension: *mut cef_sys::cef_extension_t, browser: *mut cef_sys::cef_browser_t, active_browser: *mut cef_sys::cef_browser_t, index: i32, url: *const cef_sys::cef_string_t, active: i32, windowInfo: *mut cef_sys::cef_window_info_t, client: *mut *mut cef_sys::cef_client_t, settings: *mut cef_sys::cef_browser_settings_t) -> i32 {
  let mut client__tmp = crate::include::CefClient::from_cef_own(*client).unwrap();
  let ret = CefExtensionHandler::from_cef(_self, true).get().on_before_browser(crate::include::CefExtension::from_cef_own(extension).unwrap(),crate::include::CefBrowser::from_cef_own(browser).unwrap(),crate::include::CefBrowser::from_cef_own(active_browser).unwrap(),index,&crate::include::internal::CefString::from_cef(url).unwrap(),if active == 0 { false } else { true },&mut *(windowInfo as *mut _),&mut client__tmp,&mut *(settings as *mut _),);
  *client = crate::include::CefClient::to_cef_own(client__tmp);
  if ret { 1 } else { 0 }
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_extension_handler_t_get_active_browser(_self: *mut cef_sys::cef_extension_handler_t, extension: *mut cef_sys::cef_extension_t, browser: *mut cef_sys::cef_browser_t, include_incognito: i32) -> *mut cef_sys::cef_browser_t {
  let ret = CefExtensionHandler::from_cef(_self, true).get().get_active_browser(crate::include::CefExtension::from_cef_own(extension).unwrap(),crate::include::CefBrowser::from_cef_own(browser).unwrap(),if include_incognito == 0 { false } else { true },);
  ret.map_or(std::ptr::null_mut(), |o| crate::include::CefBrowser::to_cef_own(o))
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_extension_handler_t_can_access_browser(_self: *mut cef_sys::cef_extension_handler_t, extension: *mut cef_sys::cef_extension_t, browser: *mut cef_sys::cef_browser_t, include_incognito: i32, target_browser: *mut cef_sys::cef_browser_t) -> i32 {
  let ret = CefExtensionHandler::from_cef(_self, true).get().can_access_browser(crate::include::CefExtension::from_cef_own(extension).unwrap(),crate::include::CefBrowser::from_cef_own(browser).unwrap(),if include_incognito == 0 { false } else { true },crate::include::CefBrowser::from_cef_own(target_browser).unwrap(),);
  if ret { 1 } else { 0 }
}
#[allow(non_snake_case)]
unsafe extern "C" fn cef_extension_handler_t_get_extension_resource(_self: *mut cef_sys::cef_extension_handler_t, extension: *mut cef_sys::cef_extension_t, browser: *mut cef_sys::cef_browser_t, file: *const cef_sys::cef_string_t, callback: *mut cef_sys::cef_get_extension_resource_callback_t) -> i32 {
  let ret = CefExtensionHandler::from_cef(_self, true).get().get_extension_resource(crate::include::CefExtension::from_cef_own(extension).unwrap(),crate::include::CefBrowser::from_cef_own(browser).unwrap(),&crate::include::internal::CefString::from_cef(file).unwrap(),crate::include::CefGetExtensionResourceCallback::from_cef_own(callback).unwrap(),);
  if ret { 1 } else { 0 }
}
